pub struct StepSound {
    name: String,
    volume: f32,
    pitch: f32,
}

impl StepSound {
    pub const fn new(name: String, volume: f32, pitch: f32) -> Self {
        Self {
            name,
            volume,
            pitch,
        }
    }
}
