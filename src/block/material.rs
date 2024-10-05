pub enum MaterialType {
    Air(TransparentMaterial),
    Grass(NormalMaterial),
    Wood(BurnableMaterial),
    Rock(NormalMaterial),
    Iron(NormalMaterial),
    Water(LiquidMaterial),
    Lava(LiquidMaterial),
    Leaves(BurnableMaterial),
    Plants(LogicMaterial),
    Sponge(NormalMaterial),
    Cloth(BurnableMaterial),
    Fire(TransparentMaterial),
    Sand(NormalMaterial),
    Circuits(LogicMaterial),
    Glass(NormalMaterial),
    Tnt(BurnableMaterial),
    Ice(NormalMaterial),
    Snow(LogicMaterial),
    CraftedSnow(NormalMaterial),
    Cactus(NormalMaterial),
    Clay(NormalMaterial),
}

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

pub struct BurnableMaterial;
pub struct LiquidMaterial;
pub struct LogicMaterial;
pub struct NormalMaterial;
pub struct TransparentMaterial;

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
