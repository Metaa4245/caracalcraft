pub mod blocks;
pub mod material;
pub mod sky_block;
pub mod step_sound;

pub trait Block {
    fn id() -> i32;
}
