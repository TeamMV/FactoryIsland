use mvengine::ui::geometry::SimpleRect;
use mvengine::window::Window;

pub struct Camera {
    pub pos: (i32, i32),
    pub width: i32,
    pub height: i32,
    pub view_area: SimpleRect,
}

impl Camera {
    pub fn new(width: i32, height: i32) -> Self {
        let w = width;
        let h = height;
        Self {
            pos: (0, 0),
            width: w,
            height: h,
            view_area: SimpleRect::new(0, 0, w, h),
        }
    }

    pub fn update(&mut self) {
        self.view_area = SimpleRect::new(self.pos.0, self.pos.1, self.width, self.height);
    }
}
