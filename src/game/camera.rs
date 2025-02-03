use mvutils::utils::PClamp;

pub struct Camera {
    pub x: f64,
    pub y: f64,
    pub zoom: f32,
    pub pivot_x: i32,
    pub pivot_y: i32,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            zoom: 1.0,
            pivot_x: 0,
            pivot_y: 0,
        }
    }

    pub fn move_rel(&mut self, dx: f64, dy: f64) {
        self.x += dx;
        self.y += dy;
    }

    pub fn move_to(&mut self, x: f64, y: f64) {
        self.x = x;
        self.y = y;
    }

    pub fn zoom(&mut self, dz: f32) {
        self.zoom += dz;
        self.zoom = self.zoom.p_clamp(0.5, 5.0);
    }

    pub fn set_zoom(&mut self, zoom: f32) {
        self.zoom = zoom;
    }
}