use std::thread::sleep;
use std::time::Duration;
use api::player::profile::PlayerProfile;
use api::world::{World, WorldMeta, META_FILENAME};
use mvengine::ui::context::{UiContext, UiResources};
use crate::res::R;
use mvengine_proc::resolve_resource;
use mvengine::{expect_element_by_id, modify_style};
use mvengine::game::fs::smartdir::SmartDir;
use mvengine::ui::page::Page;
use mvengine::window::Window;
use mvengine::input::consts::MouseButton;
use mvengine::ui::elements::Element;
use mvengine::ui::elements::events::UiClickAction;
use mvengine::ui::elements::prelude::*;
use mvengine::ui::utils::ToRope;
use mvengine_proc::{style_expr_empty, ui};
use mvutils::state::State;
use rand::{rng, RngCore};
use ropey::Rope;
use crate::app::LauncherOptions;
use crate::{winmsg, uistyles};

pub struct WorldPage {
    element: Element,
    back_btn: Element,
    create_btn: Element,
    delete_btn: Element,
    worlds_list: Element,

    world_dir: SmartDir,

    selected_index: Option<usize>,
    worlds: Vec<String>,
    world_elements: Vec<Element>,

    new_world_name: State<Rope>,
    default_select_world: Option<String>
}

impl WorldPage {
    pub fn new(window: &mut Window, fi_dir: &SmartDir) -> Self {
        let mut frame_style = uistyles::FRAME.clone();
        frame_style.merge_at_set_of(&style_expr_empty!("child_align_y: end; height: 100%;"));

        let mut h_lay20 = uistyles::H_LAYOUT.clone();
        h_lay20.merge_at_set_of(&style_expr_empty!("width: 100%; child_align_x: middle;"));

        let mut small_style = uistyles::PRESET_SMALL.clone();
        small_style.merge_at_set_of(&style_expr_empty!("margin: 0, 2mm;"));

        let mut list_style = uistyles::V_LAYOUT.clone();
        list_style.merge_at_set_of(&style_expr_empty!("width: 100%; height: 60%; border.resource: color; border.color: @R.color/ui_highlight; margin.bottom: 2mm; padding: 2mm;"));

        let mut name_style = uistyles::EDIT_PRESET.clone();
        name_style.merge_at_set_of(&style_expr_empty!("width: 100%; text.align_x: start;"));

        let new_world_name = State::new(Rope::new());

        let ui = ui! {
            <Ui context={window.ui().context()}>
                <Div id="world" style={uistyles::FRAME.clone()}>
                    <Div style={frame_style}>
                        <Div style={h_lay20}>
                            <Button style={small_style.clone()} id="back_btn">Back</Button>
                            <Button style={small_style.clone()} id="create_btn">Create</Button>
                            <Button style={small_style.clone()} id="delete_btn">Delete</Button>
                        </Div>
                        <TextBox style={name_style} placeholder="New World Name..." content={new_world_name.clone()}/>
                        <Div style={list_style} id="worlds_list">

                        </Div>
                    </Div>
                </Div>
            </Ui>
        };

        let back_btn = expect_element_by_id!(ui, "back_btn");
        let create_btn = expect_element_by_id!(ui, "create_btn");
        let delete_btn = expect_element_by_id!(ui, "delete_btn");
        let worlds_list = expect_element_by_id!(ui, "worlds_list");

        Self {
            element: ui,
            back_btn,
            create_btn,
            delete_btn,
            worlds_list,
            world_dir: fi_dir.join("worlds"),
            selected_index: None,
            worlds: vec![],
            world_elements: vec![],
            new_world_name,
            default_select_world: None,
        }
    }

    pub fn check_events(&mut self, window: &mut Window) {
        self.back_btn.navigate_back(window);

        if self.create_btn.was_left_clicked() {
            let world_name = self.new_world_name.read().to_string();
            if world_name.is_empty() {
                winmsg::error("World name cannot be empty!");
            } else {
                let dir_name = world_name.replace(' ', "_");
                if dir_name == "main" {
                    winmsg::error("World name 'main' is reserved and cannot be used!");
                } else {
                    let world_dir = self.world_dir.join(dir_name);
                    let meta = WorldMeta::new(&world_name, rng().next_u32());
                    world_dir.save_object(&meta, META_FILENAME);

                    self.create_world_elements(window.ui().context());
                }
            }
        }

        if self.delete_btn.was_left_clicked() {
            if let Some(i) = self.selected_index {
                if winmsg::ask_yes_no("Delete this world 100%?") {
                    let dir_name = &self.worlds[i];
                    self.world_dir.delete_child(dir_name);

                    self.create_world_elements(window.ui().context());
                }
            }
        }

        let mut clicked = None;
        for (i, e) in self.world_elements.iter().enumerate() {
            if e.was_left_clicked() {
                clicked = Some(i);
                break;
            }
        }
        if let Some(i) = clicked {
            self.select_world(i);
        }
    }

    pub fn create_world_elements(&mut self, context: UiContext) {
        self.selected_index = None;
        let mut worlds = vec![];
        let mut world_elems = vec![];

        for world in self.world_dir.iter_dirs() {
            if let Some(meta) = world.read_object::<WorldMeta>(META_FILENAME) {
                let folder_name = world.dir_name();
                let rope = meta.name.to_rope();
                worlds.push(folder_name);

                let mut style = uistyles::PRESET.clone();
                style.merge_at_set_of(&style_expr_empty!("width: 100%;"));

                let ui = ui! {
                    <Ui context={context.clone()}>
                        <Button style={style}>{rope}</Button>
                    </Ui>
                };
                world_elems.push(ui);
            }
        }

        self.worlds = worlds;
        self.world_elements = world_elems;
        let div = self.worlds_list.get_mut();
        div.remove_all_children();
        let child = self.world_elements.clone().into_iter();
        let child = child.to_child();
        div.add_child(child);
        div.state_mut().invalidate();

        if let Some(def) = &self.default_select_world {
            let i = self.worlds.iter().position(|world| world == def);
            if let Some(idx) = i {
                self.select_world(idx);
            }
            self.selected_index = i;
        }
    }

    fn select_world(&mut self, idx: usize) {
        if let Some(prev) = self.selected_index {
            let prev_e = self.world_elements[prev].get_mut();
            let mut style = prev_e.style_mut();
            style.merge_at_set_of(&style_expr_empty!("border.resource: none;"));
        }
        self.selected_index = Some(idx);
        let e = &self.world_elements[idx];
        let mut style = e.get_mut().style_mut();
        style.merge_at_set_of(&style_expr_empty!("border.resource: color; border.color: @R.color/ui_highlight;"));
        self.default_select_world = Some(self.worlds[idx].clone());
    }

    pub fn selected_world(&self) -> Option<String> {
        if let Some(i) = self.selected_index {
            Some(self.worlds[i].clone())
        } else {
            None
        }
    }

    pub fn default_select_world(&mut self, s: Option<String>) {
        self.default_select_world = s;
    }
}

impl Page for WorldPage {
    fn get_elem(&self) -> Element {
        self.element.clone()
    }
}