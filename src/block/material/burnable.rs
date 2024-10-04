use super::Material;

pub struct BurnableMaterial;

impl Material for BurnableMaterial {
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
        true
    }
}
