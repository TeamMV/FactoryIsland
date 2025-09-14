use mvengine::modify_style;
use mvengine::ui::elements::checkbox::CheckBox;
use std::env;
use mvengine::ui::elements::button::Button;
use mvengine::ui::elements::textbox::TextBox;
use crate::{winmsg, uistyles};
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
use mvengine::ui::context::UiResources;
use mvengine::ui::elements::events::UiClickAction;
use mvengine::ui::geometry::shape::shapes;
use mvengine::ui::page::Page;
use mvengine::ui::styles::InheritSupplier;
use mvengine::ui::utils::ToRope;
use mvengine::window::app::WindowCallbacks;
use mvengine::window::Window;
use mvengine_proc::{style_expr_empty, ui};
use mvutils::once::CreateOnce;
use mvutils::Savable;
use mvutils::state::State;
use mvutils::utils::TetrahedronOp;
use ropey::Rope;
use crate::pages::main::MainPage;
use crate::pages::world::WorldPage;
use crate::res::R;

pub struct LauncherApp {
    fi_dir: SmartDir,
    launcher_dir: SmartDir,
    draw_ctx: CreateOnce<RenderingPipeline<OpenGLRenderer>>,

    main_page: CreateOnce<MainPage>,
    world_page: CreateOnce<WorldPage>
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
            main_page: CreateOnce::new(),
            world_page: CreateOnce::new(),
        }
    }
}

impl WindowCallbacks for LauncherApp {
    fn post_init(&mut self, window: &mut Window) {
        R::initialize();
        window.ui_mut().init(R.deref().deref());

        let mut world_page = WorldPage::new(window, &self.fi_dir);
        let main_page = MainPage::new(window, &self.fi_dir, &self.launcher_dir, &mut world_page);

        let page_manager = window.ui_mut().page_manager_mut();
        page_manager.add_page(main_page.get_elem());
        page_manager.add_page(world_page.get_elem());

        self.main_page.create(|| main_page);
        self.world_page.create(|| world_page);

        page_manager.open("main");

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
        if self.main_page.check_events(window, &self.world_page) {
            //world clicked
            self.world_page.create_world_elements(window.ui().context());
        }
        self.world_page.check_events(window);
    }
}

#[derive(Clone, Savable)]
pub struct LauncherOptions {
    pub(crate) singleplayer: bool,
    pub(crate) selected_world: Option<String>
}