use mvengine::modify_style;
use mvengine::ui::elements::checkbox::CheckBox;
use std::env;
use mvengine::ui::elements::button::Button;
use mvengine::ui::elements::textbox::TextBox;
use crate::{err, uistyles};
use mvengine::ui::elements::{Element, UiElementStub};
use mvengine::ui::elements::div::Div;
use std::ops::Deref;
use std::path::PathBuf;
use std::process::Command;
use std::str::FromStr;
use api::player::profile::PlayerProfile;
use mvengine::color::RgbColor;
use mvengine::expect_element_by_id;
use mvengine::game::fs::smartdir::SmartDir;
use mvengine::game::timing::{DelayedTask, TaskId};
use mvengine::input::consts::MouseButton;
use mvengine::rendering::OpenGLRenderer;
use mvengine::rendering::pipeline::RenderingPipeline;
use mvengine::ui::attributes::ToRope;
use mvengine::ui::context::UiResources;
use mvengine::ui::elements::events::UiClickAction;
use mvengine::ui::geometry::shape::shapes;
use mvengine::window::app::WindowCallbacks;
use mvengine::window::Window;
use mvengine_proc::{style_expr_empty, ui};
use mvutils::once::CreateOnce;
use mvutils::Savable;
use mvutils::state::State;
use mvutils::utils::TetrahedronOp;
use ropey::Rope;
use crate::res::R;

pub struct LauncherApp {
    fi_dir: SmartDir,
    launcher_dir: SmartDir,
    draw_ctx: CreateOnce<RenderingPipeline<OpenGLRenderer>>,

    save_btn: CreateOnce<Element>,
    launch_btn: CreateOnce<Element>,

    profile_name: State<Rope>,
    profile: Option<PlayerProfile>,
    singleplayer: State<bool>,
    version: State<Rope>,

    btn_ok_task: Option<TaskId>
}

impl LauncherApp {
    pub fn new() -> Self {
        let appdata = env::var("APPDATA").unwrap();
        let path = PathBuf::from_str(&appdata).unwrap();
        let path = path.join(".factoryisland");
        let fi_dir = SmartDir::new(path);
        let launcher_dir = fi_dir.join("launcher");

        Self {
            fi_dir,
            launcher_dir,
            draw_ctx: CreateOnce::new(),
            save_btn: CreateOnce::new(),
            launch_btn: CreateOnce::new(),
            profile_name: State::new(Rope::new()),
            profile: None,
            singleplayer: State::new(false),
            version: State::new(Rope::from_str("latest")),
            btn_ok_task: None,
        }
    }
}

impl WindowCallbacks for LauncherApp {
    fn post_init(&mut self, window: &mut Window) {
        R::initialize();
        window.ui_mut().init(R.deref().deref());

        let profile_name = self.profile_name.clone();
        if let Some(profile) = self.fi_dir.read_object::<PlayerProfile>("profile.sav") {
            *profile_name.write() = profile.name.to_rope();
            self.profile = Some(profile);
        }

        let singleplayer = self.singleplayer.clone();
        if let Some(options) = self.launcher_dir.read_object::<LauncherOptions>("launcher.sav") {
            *singleplayer.write() = options.singleplayer;
        }

        let ui = ui! {
            <Ui context={window.ui().context()}>
                <Div style={uistyles::FRAME.clone()}>
                    <Button style={uistyles::CLEAR_PRESET.clone()}>Profile:</Button>
                    <TextBox style={uistyles::EDIT_PRESET.clone()} id="profile_name" content={profile_name} placeholder="Name"/>
                    <CheckBox style={uistyles::CHECKBOX_PRESET.clone()} selected={singleplayer}>Singleplayer</CheckBox>
                    <Button style={uistyles::PRESET.clone()} id="profile_save">Save changes</Button>
                    <Button style={uistyles::PRESET.clone()} id="start_game">Launch Game</Button>
                </Div>
            </Ui>
        };

        let profile_save_btn = expect_element_by_id!(ui, "profile_save");
        self.save_btn.create(|| profile_save_btn);

        let launch_btn = expect_element_by_id!(ui, "start_game");
        self.launch_btn.create(|| launch_btn);

        window.ui_mut().add_root(ui);

        let pipeline = RenderingPipeline::new_default_opengl(window).unwrap();
        self.draw_ctx.create(|| pipeline);
    }

    fn update(&mut self, window: &mut Window, delta_u: f64) {

    }

    fn draw(&mut self, window: &mut Window, delta_t: f64) {
        R.tick_all_animations();
        self.check_events(window);

        OpenGLRenderer::clear();
        OpenGLRenderer::enable_depth_test();
        OpenGLRenderer::enable_depth_buffer();

        self.draw_ctx.begin_frame();

        let area = window.area();
        window.ui_mut().draw(&mut *self.draw_ctx, &area);
        self.draw_ctx.advance(window, |_| {});
        self.draw_ctx.flush();
    }

    fn exiting(&mut self, window: &mut Window) {

    }

    fn resize(&mut self, window: &mut Window, width: u32, height: u32) {
        self.draw_ctx.resize(window);
    }
}

impl LauncherApp {
    pub fn check_events(&mut self, window: &mut Window) {
        let scheduler = &mut window.ui_mut().context().scheduler;
        if let Some(id) = self.btn_ok_task {
            if scheduler.tick(id) {
                let b = self.launch_btn.get_mut();
                *b.style_mut() = uistyles::PRESET.clone();
            }
        }

        if let Some(event) = &self.save_btn.get().state().events.click_event {
            if let MouseButton::Left = event.button {
                if let UiClickAction::Click = event.base.action {
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
                    };
                    self.launcher_dir.save_object(&options, "launcher.sav");
                }
            }
        }
        if let Some(event) = &self.launch_btn.get().state().events.click_event {
            if let MouseButton::Left = event.button {
                if let UiClickAction::Click = event.base.action {
                    let exe = self.launcher_dir.path().join(&self.version.read().to_string());
                    let exe = exe.join("FactoryIsland.exe");
                    let args = self.singleplayer.read().yn("-internalserver", "");
                    if self.launch_app(&exe, args) {
                        let b = self.launch_btn.get_mut();
                        b.style_mut().merge_at_set_of(&style_expr_empty!("background.color: green;"));
                        let id = scheduler.queue(DelayedTask::new(1000));
                        self.btn_ok_task = Some(id);
                    }
                }
            }
        }
    }

    pub fn launch_app(&self, exe: &PathBuf, args: &str) -> bool {
        let arg_list: Vec<&str> = args.split_whitespace().collect();

        let path = self.launcher_dir.path();
        let c = Command::new(&exe)
            .current_dir(path)
            .args(arg_list)
            .spawn();
        match c {
            Ok(_) => true,
            Err(e) => {
                let msg = format!("Error when running game:\n{e:?}\nFile needed: {exe:?}");
                err::error(&msg);
                false
            }
        }
    }
}

#[derive(Clone, Savable)]
pub struct LauncherOptions {
    singleplayer: bool,
}