use super::Block;

pub struct Stone {}

impl Block for Stone {
    fn id() -> i32 {
        1
    }
}
