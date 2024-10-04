use super::Material;

pub struct LiquidMaterial {}

impl Material for LiquidMaterial {
    fn is_liquid() -> bool {
        true
    }

    fn is_solid() -> bool {
        true
    }

    fn can_burn(&self) -> bool {
        false
    }
}
