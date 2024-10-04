use super::{
    burnable::BurnableMaterial, liquid::LiquidMaterial, logic::LogicMaterial,
    normal::NormalMaterial, transparent::TransparentMaterial,
};

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
