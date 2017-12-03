//! Constants for easier access to the assets

use texture::{SourcePixelRegion, TextureId, SourceTextureRegion, TextureInstanceId};

pub const FONT_BIG_SIZE: u32 = 48;
pub const FONT_MEDIUM_SIZE: u32 = 18;
pub const FONT_SMALL_SIZE: u32 = 14;

pub const FONT_ID: &str = "FredokaOne-Regular";
pub const FONT: &[u8] = include_bytes!("../assets/fonts/FredokaOne-Regular.ttf");

pub const GAME_TITLE: &str = "StackBoxes!";

pub const START_SCREEN_BUTTON_00_ID: &str = "../assets/images/ui/PNG/yellow_button04.png";
pub const START_SCREEN_BUTTON_00: &[u8] = include_bytes!("../assets/images/ui/PNG/yellow_button04.png");
pub const START_SCREEN_BUTTON_00_TX_STR: SourceTextureRegion = SourceTextureRegion {
    texture_id: TextureId { texture_id: START_SCREEN_BUTTON_00_ID },
    /*image dimensions: 190 w * 49 h*/
    region: SourcePixelRegion {
        bottom_x: 0,
        bottom_y: 0,
        width: 190,
        height: 49,
    }
};


pub const HERO_TEXTURE_ID: &str = "../assets/images/ui/PNG/yellow_button04.png";
pub const HERO_TEXTURE: &[u8] = include_bytes!("../assets/images/hero.png");
/* todo: add source pixel regions*/
pub const HERO_TX_STR: SourceTextureRegion = SourceTextureRegion {
    texture_id: TextureId { texture_id: START_SCREEN_BUTTON_00_ID },
    /*image dimensions: 190 w * 49 h*/
    region: SourcePixelRegion {
        bottom_x: 0,
        bottom_y: 0,
        width: 190,
        height: 49,
    }
};
