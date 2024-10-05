pub mod blocks;
pub mod material;
pub mod step_sound;

pub trait Block {
    fn id() -> i32;
}
