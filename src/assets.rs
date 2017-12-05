//! Constants for easier access to the assets

use texture::{SourcePixelRegion, TextureId, SourceTextureRegion, TextureInstanceId};

// fonts

pub const FONT_BIG_SIZE: u32 = 48;
pub const FONT_MEDIUM_SIZE: u32 = 18;
pub const FONT_SMALL_SIZE: u32 = 14;

pub const FONT_ID: &str = "FredokaOne-Regular";
pub const FONT: &[u8] = include_bytes!("../assets/fonts/FredokaOne-Regular.ttf");

pub const GAME_TITLE: &str = "StackBoxes!";

// audio

pub const TITLE_SCREEN_SONG_DATA: &[u8] = include_bytes!("../assets/sounds/music/title_screen.ogg");
pub const AUDIO_MSG_PLAY_TITLE_SCREEN_SONG: &'static str = "../assets/sounds/music/title_screen.ogg";

pub const GAME_SONG_1_DATA: &[u8] = include_bytes!("../assets/sounds/music/level_1.ogg");
// pub const GAME_SONG_2_DATA: &[u8] = include_bytes!("../assets/sounds/music/level_2.ogg");
// pub const GAME_SONG_3_DATA: &[u8] = include_bytes!("../assets/sounds/music/level_3.ogg");
pub const AUDIO_MSG_PLAY_GAME_SONG: &'static str = "../assets/sounds/music/level_1.ogg";

pub const ENDING_SONG_1_DATA: &[u8] = include_bytes!("../assets/sounds/music/ending.ogg");
pub const AUDIO_MSG_PLAY_ENDING_SONG: &'static str = "../assets/sounds/music/ending.ogg";


// -- textures

// start screen
pub const START_SCREEN_BUTTON_00_ID: &str = "../assets/images/ui/PNG/yellow_button04.png";
pub const START_SCREEN_BUTTON_00: &[u8] = include_bytes!("../assets/images/ui/PNG/yellow_button04.png");
pub const START_SCREEN_BUTTON_00_TX_STR: SourceTextureRegion = SourceTextureRegion {
    texture_id: TextureId { texture_id: START_SCREEN_BUTTON_00_ID },
    region: SourcePixelRegion {
        bottom_x: 0,
        bottom_y: 0,
        width: 190,
        height: 49,
    }
};

// hero texture
pub const HERO_TEXTURE_ID: &str = "../assets/images/hero.png";
pub const HERO_TEXTURE: &[u8] = include_bytes!("../assets/images/hero.png");
/* todo: add source pixel regions*/
pub const HERO_TX_NORMAL_STR: SourceTextureRegion = SourceTextureRegion {
    texture_id: TextureId { texture_id: HERO_TEXTURE_ID },
    region: SourcePixelRegion {
        bottom_x: 0,
        bottom_y: 64,
        width: 16,
        height: 16,
    }
};

// crate texture
pub const CRATE_TEXTURE_ID: &str = "../assets/images/crate.png";
pub const CRATE_TEXTURE_DATA: &[u8] = include_bytes!("../assets/images/crate.png");
pub const CRATE_TEXTURE_TX_STR: SourceTextureRegion = SourceTextureRegion {
    texture_id: TextureId { texture_id: CRATE_TEXTURE_ID },
    region: SourcePixelRegion {
        bottom_x: 0,
        bottom_y: 0,
        width: 32,
        height: 32,
    }
};

// background texture
pub const BACKGROUND_3_TEXTURE_ID: &str = "../assets/images/background/3.png";
pub const BACKGROUND_3_TEXTURE_DATA: &[u8] = include_bytes!("../assets/images/background/3.png");
pub const BACKGROUND_3_TEXTURE_TX_STR: SourceTextureRegion = SourceTextureRegion {
    texture_id: TextureId { texture_id: BACKGROUND_3_TEXTURE_ID },
    region: SourcePixelRegion {
        bottom_x: 0,
        bottom_y: 0,
        width: 256,
        height: 182,
    }
};
