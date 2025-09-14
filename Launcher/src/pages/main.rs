use mvengine::modify_style;
use std::path::PathBuf;
use std::process::Command;
use api::player::profile::PlayerProfile;
use mvengine::expect_element_by_id;
use mvengine::game::fs::smartdir::SmartDir;
use mvengine::game::timing::{DelayedTask, TaskId};
use mvengine::input::consts::MouseButton;
use mvengine::ui::elements::Element;
use mvengine::ui::elements::events::UiClickAction;
use mvengine::ui::elements::prelude::*;
use mvengine::ui::page::Page;
use mvengine::ui::utils::ToRope;
use mvengine::window::Window;
use mvengine_proc::{style_expr_empty, ui};
use mvutils::state::State;
use mvutils::utils::TetrahedronOp;
use ropey::Rope;
use crate::app::LauncherOptions;
use crate::{winmsg, uistyles};
use crate::pages::world::WorldPage;

pub struct MainPage {
    element: Element,
    save_btn: Element,
    world_btn: Element,
    launch_btn: Element,

    fi_dir: SmartDir,
    launcher_dir: SmartDir,

    profile_name: State<Rope>,
    singleplayer: State<bool>,
    profile: Option<PlayerProfile>,
    version: State<Rope>,

    btn_ok_task: Option<TaskId>,
}

impl MainPage {
    pub fn new(window: &mut Window, fi_dir: &SmartDir, launcher_dir: &SmartDir, world_page: &mut WorldPage) -> Self {
        let profile_name = State::new(Rope::new());
        let singleplayer = State::new(false);

        let profile_name = profile_name.clone();
        let mut sprofile = None;
        if let Some(profile) = fi_dir.read_object::<PlayerProfile>("profile.sav") {
            *profile_name.write() = profile.name.to_rope();
            sprofile = Some(profile);
        }

        let singleplayer = singleplayer.clone();
        if let Some(options) = launcher_dir.read_object::<LauncherOptions>("launcher.sav") {
            *singleplayer.write() = options.singleplayer;
            world_page.default_select_world(options.selected_world);
        }

        //STOP TWEAKING INTELLIJ thank you
        let ui = ui! {
            <Ui context={window.ui().context()}>
                <Div id="main" style={uistyles::FRAME.clone()}>
                    <Div style={uistyles::V_LAYOUT.clone()}>
                        <Button style={uistyles::CLEAR_PRESET.clone()}>Profile:</Button>
                        <TextBox style={uistyles::EDIT_PRESET.clone()} id="profile_name" content={profile_name.clone()} placeholder="Name"/>
                        <CheckBox style={uistyles::CHECKBOX_PRESET.clone()} selected={singleplayer.clone()}>Singleplayer</CheckBox>
                        <Button style={uistyles::PRESET.clone()} id="profile_save">Save changes</Button>
                        <Button style={uistyles::PRESET.clone()} id="world_select">{"World -->"}</Button>
                        <Button style={uistyles::PRESET.clone()} id="start_game">Launch Game</Button>
                    </Div>
                </Div>
            </Ui>
        };

        let save_btn = expect_element_by_id!(ui, "profile_save");
        let world_btn = expect_element_by_id!(ui, "world_select");
        let launch_btn = expect_element_by_id!(ui, "start_game");

        Self {
            element: ui,
            save_btn,
            world_btn,
            launch_btn,
            fi_dir: fi_dir.clone(),
            launcher_dir: launcher_dir.clone(),
            profile_name,
            singleplayer,
            profile: sprofile,
            btn_ok_task: None,
            version: State::new(Rope::from_str("latest")),
        }
    }

    pub fn check_events(&mut self, window: &mut Window, world_page: &WorldPage) -> bool {
        let scheduler = &mut window.ui_mut().context().scheduler;
        if let Some(id) = self.btn_ok_task {
            if scheduler.tick(id) {
                let b = self.launch_btn.get_mut();
                *b.style_mut() = uistyles::PRESET.clone();
            }
        }

        if self.save_btn.was_left_clicked() {
            let name = self.profile_name.read().to_string();
            if let Some(profile) = &mut self.profile {
                profile.name = name;
                self.fi_dir.save_object(profile, "profile.sav");
            } else {
                let mut profile = PlayerProfile::new();
                profile.name = name;
                self.fi_dir.save_object(&profile, "profile.sav");
                self.profile = Some(profile);
            }
            let options = LauncherOptions {
                singleplayer: *self.singleplayer.read(),
                selected_world: world_page.selected_world(),
            };
            self.launcher_dir.save_object(&options, "launcher.sav");
        }

        if self.launch_btn.was_left_clicked() {
            let exe = self.launcher_dir.path().join(&self.version.read().to_string());
            let exe = exe.join("FactoryIsland.exe");

            let mut args = Vec::new();
            if *self.singleplayer.read() {
               args.push("-internalserver".to_string());
            }
            if let Some(sel_world) = world_page.selected_world() {
                args.push("-world".to_string());
                args.push(sel_world);
            } else {
                if !winmsg::ask_yes_no("No world selected? This will use the 'main' default world!") {
                    return false;
                }
            }

            if self.launch_app(&exe, args) {
                let b = self.launch_btn.get_mut();
                b.style_mut().merge_at_set_of(&style_expr_empty!("background.color: green;"));
                let id = scheduler.queue(DelayedTask::new(1000));
                self.btn_ok_task = Some(id);
            }
        }

        self.world_btn.navigate_page("world", window)
    }

    pub fn launch_app(&self, exe: &PathBuf, args: Vec<String>) -> bool {
        let path = self.launcher_dir.path();
        let c = Command::new(&exe)
            .current_dir(path)
            .args(args)
            .spawn();
        match c {
            Ok(_) => true,
            Err(e) => {
                let msg = format!("Error when running game:\n{e:?}\nFile needed: {exe:?}");
                winmsg::error(&msg);
                false
            }
        }
    }
}

impl Page for MainPage {
    fn get_elem(&self) -> Element {
        self.element.clone()
    }
}