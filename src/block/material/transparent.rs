use super::Material;

pub struct TransparentMaterial {}

impl Material for TransparentMaterial {
    fn is_solid() -> bool {
        false
    }

    fn can_grass() -> bool {
        false
    }

    fn can_burn(&self) -> bool {
        false
    }
}
