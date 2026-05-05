pub const RECT_HALF: f32 = 0.5;

pub struct Rect {
    pub time: f32,
    pub speed: f32,
}

impl Rect {
    pub fn new() -> Self {
        Self {
            time: 0.0,
            speed: 0.5,
        }
    }

    pub fn update(&mut self) -> f32 {
        self.time += 1.0;
        self.time
    }
}
