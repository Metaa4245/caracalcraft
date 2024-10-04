use super::Material;

pub struct LogicMaterial {}

impl Material for LogicMaterial {
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
