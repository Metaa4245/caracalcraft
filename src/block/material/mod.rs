pub mod burnable;
pub mod liquid;
pub mod logic;
pub mod normal;
pub mod transparent;
pub mod types;

pub trait Material {
    fn is_liquid() -> bool {
        false
    }

    fn is_solid() -> bool {
        false
    }

    fn can_grass() -> bool {
        false
    }

    fn can_burn(&self) -> bool;
}
