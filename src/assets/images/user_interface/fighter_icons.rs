use std::fmt::Display;

#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq)]
pub enum FighterIcon {
    CrystalMauler,
    FireKnight,
    GroundMonk,
    LeafRanger,
    MetalBladekeeper,
    WaterPriestess,
    WindHashashin,
}

impl Display for FighterIcon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FighterIcon::CrystalMauler => {
                write!(f, "images/user_interface/fighter_icons/crystal_mauler.png")
            }
            FighterIcon::FireKnight => {
                write!(f, "images/user_interface/fighter_icons/fire_knight.png")
            }
            FighterIcon::GroundMonk => {
                write!(f, "images/user_interface/fighter_icons/ground_monk.png")
            }
            FighterIcon::LeafRanger => {
                write!(f, "images/user_interface/fighter_icons/leaf_ranger.png")
            }
            FighterIcon::MetalBladekeeper => {
                write!(
                    f,
                    "images/user_interface/fighter_icons/metal_bladekeeper.png"
                )
            }
            FighterIcon::WaterPriestess => {
                write!(f, "images/user_interface/fighter_icons/water_priestess.png")
            }
            FighterIcon::WindHashashin => {
                write!(f, "images/user_interface/fighter_icons/wind_hashashin.png")
            }
        }
    }
}
