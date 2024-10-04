use super::Material;

pub struct NormalMaterial;

impl Material for NormalMaterial {
    fn is_liquid() -> bool {
        false
    }

    fn is_solid() -> bool {
        true
    }

    fn can_grass() -> bool {
        true
    }

    fn can_burn(&self) -> bool {
        false
    }
}
