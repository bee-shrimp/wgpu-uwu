pub const RECT_HALF: f32 = 0.05;

#[derive(Debug, Clone, Copy)]
pub struct Rect {
    pub pos_x: f32,
    pub pos_y: f32,
    time: f32,
    speed: f32,
}

impl Rect {
    pub fn new() -> Self {
        Self {
            pos_x: 0.0,
            pos_y: 0.0,
            time: 0.0,
            speed: 0.01,
        }
    }

    pub fn update(&mut self) -> (f32, f32) {
        self.pos_x += f32::sin(self.time) / 2.0 * self.speed;
        self.pos_y += f32::cos(self.time) / 2.0 * self.speed;

        self.time += 0.1;
        (self.pos_x - RECT_HALF, self.pos_y)
    }
}

//TODO make World struct with array of rects. let the rects pos be instances pos.
