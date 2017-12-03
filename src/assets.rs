//! Constants for easier access to the assets

use texture::{SourcePixelRegion, TextureId, SourceTextureRegion};

pub const FONT_BIG_SIZE: u32 = 48;
pub const FONT_SMALL_SIZE: u32 = 14;

pub const FONT_ID: &str = "FredokaOne-Regular";
pub const FONT: &[u8] = include_bytes!("../assets/fonts/FredokaOne-Regular.ttf");

pub const GAME_TITLE: &str = "StackBoxes!";

pub const START_SCREEN_BUTTON_00_ID: &str = "../assets/images/ui/PNG/yellow_button00.png";
pub const START_SCREEN_BUTTON_00: &[u8] = include_bytes!("../assets/images/ui/PNG/yellow_button00.png");
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

pub const START_SCREEN_BUTTON_01_ID: &str = "../assets/images/ui/PNG/yellow_button01.png";
pub const START_SCREEN_BUTTON_01: &[u8] = include_bytes!("../assets/images/ui/PNG/yellow_button01.png");

pub const START_SCREEN_BUTTON_02_ID: &str = "../assets/images/ui/PNG/yellow_button02.png";
pub const START_SCREEN_BUTTON_02: &[u8] = include_bytes!("../assets/images/ui/PNG/yellow_button02.png");

pub const START_SCREEN_BUTTON_03_ID: &str = "../assets/images/ui/PNG/yellow_button03.png";
pub const START_SCREEN_BUTTON_03: &[u8] = include_bytes!("../assets/images/ui/PNG/yellow_button03.png");
