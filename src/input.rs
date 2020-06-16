pub struct Input {

}

// Related Functions
impl Input {
    pub fn new() -> Input {
        Input {
        }
    }
}

impl Input {
    pub fn key_pressed(&self, key_code: u8) -> bool {
        true
    }
}