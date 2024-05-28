use std::fmt::Display;

#[allow(dead_code)]
// #[allow(clippy::enum_variant_names)] // TODO may use regular background
#[derive(Clone, Copy, PartialEq)]
pub enum BackgroundSprite {
    Mountain,
    // Skybox,
    // BackgroundMountain,
    // ForegroundMountain,
    // BackgroundTrees,
    // ForegroundTrees,
}

impl Display for BackgroundSprite {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BackgroundSprite::Mountain => {
                write!(f, "images/background/mountain.png")
            } // Background::Skybox => {
              //     write!(f, "images/background/mountain/skybox.png")
              // }
              // Background::BackgroundMountain => {
              //     write!(f, "images/background/mountain/background_mountain.png")
              // }
              // Background::ForegroundMountain => {
              //     write!(f, "images/background/mountain/foreground_mountain.png")
              // }
              // Background::BackgroundTrees => {
              //     write!(f, "images/background/mountain/background_trees.png")
              // }
              // Background::ForegroundTrees => {
              //     write!(f, "images/background/mountain/foreground_trees.png")
              // }
        }
    }
}
