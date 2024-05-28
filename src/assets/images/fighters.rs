use std::fmt::Display;

#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq)]
pub enum FighterSprite {
    CrystalMauler,
    FireKnight,
    GroundMonk,
    LeafRanger,
    MetalBladekeeper,
    WaterPriestess,
    WindHashashin,
    None,
}

impl Display for FighterSprite {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FighterSprite::CrystalMauler => {
                write!(f, "images/fighters/crystal_mauler_288x128.png")
            }
            FighterSprite::FireKnight => {
                write!(f, "images/fighters/fire_knight_288x128.png")
            }
            FighterSprite::GroundMonk => {
                write!(f, "images/fighters/ground_monk_288x128.png")
            }
            FighterSprite::LeafRanger => {
                write!(f, "images/fighters/leaf_ranger_288x128.png")
            }
            FighterSprite::MetalBladekeeper => {
                write!(f, "images/fighters/metal_bladekeeper_288x128.png")
            }
            FighterSprite::WaterPriestess => {
                write!(f, "images/fighters/water_priestess_288x128.png")
            }
            FighterSprite::WindHashashin => {
                write!(f, "images/fighters/wind_hashashin_288x128.png")
            }
            FighterSprite::None => {
                write!(f, "")
            }
        }
    }
}
