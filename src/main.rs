//! A game where you have to build a pyramid from crates
//!
//! You are a character that can walk around an push crates on top of a heap.

#![windows_subsystem = "windows"]

#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_macros)]

#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]
#![allow(unused_doc_comment)]
#![warn(missing_copy_implementations,
        trivial_numeric_casts,
        trivial_casts,
        unused_qualifications)]
#![cfg_attr(feature = "clippy", warn(cast_possible_truncation))]
#![cfg_attr(feature = "clippy", warn(cast_possible_wrap))]
#![cfg_attr(feature = "clippy", warn(cast_precision_loss))]
#![cfg_attr(feature = "clippy", warn(cast_sign_loss))]
#![cfg_attr(feature = "clippy", warn(missing_docs_in_private_items))]
#![cfg_attr(feature = "clippy", warn(mut_mut))]
#![cfg_attr(feature = "clippy", warn(print_stdout))]
#![cfg_attr(all(not(test), feature = "clippy"), warn(result_unwrap_used))]
#![cfg_attr(feature = "clippy", warn(unseparated_literal_suffix))]
#![cfg_attr(feature = "clippy", warn(wrong_pub_self_convention))]

#[macro_use] extern crate glium;
#[macro_use] extern crate failure;
#[macro_use] extern crate log;
extern crate image;
extern crate fern;
extern crate tinyfiledialogs;
extern crate glium_text;
extern crate twox_hash;
extern crate nphysics2d;
extern crate rodio;

macro_rules! fast_hashmap {
    ($T:ident, $U:ident) => (::std::collections::HashMap<$T, $U, ::std::hash::BuildHasherDefault<::twox_hash::XxHash>>::new();)
}

pub mod input;
pub mod renderer;
pub mod context;
pub mod errors;
pub mod color;
pub mod audio;
pub mod game;

use game::Game;

fn main() {
    set_up_logging();
    let mut game = Game::new(800, 600);
    game.run_main_loop();
}

fn write_to_screen(text: &str) {
    /*
        // The `TextSystem` contains the shaders and elements used for text display.
        let system = glium_text::TextSystem::new(&display);

        // Creating a `FontTexture`, which a regular `Texture` which contains the font.
        // Note that loading the systems fonts is not covered by this library.
        let font = glium_text::FontTexture::new(&display, std::fs::File::open(&std::path::Path::new("my_font.ttf")).unwrap(), 24).unwrap();

        // Creating a `TextDisplay` which contains the elements required to draw a specific sentence.
        let text = glium_text::TextDisplay::new(&system, &font, text);

        // Finally, drawing the text is done like this:
        let matrix = [[1.0, 0.0, 0.0, 0.0],
                      [0.0, 1.0, 0.0, 0.0],
                      [0.0, 0.0, 1.0, 0.0],
                      [0.0, 0.0, 0.0, 1.0]];
        glium_text::draw(&text, &system, &mut display.draw(), matrix, (1.0, 1.0, 0.0, 1.0));
    */
}

/// Sets up the global logger
#[inline]
fn set_up_logging()
{
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{}] {}",
                record.level(),
                message
            ))
        })
        .level(log::LogLevelFilter::Debug)
        .chain(std::io::stdout())
        .chain(fern::log_file("output.log").unwrap())
        .apply()
        .expect("[FATAL] Failed to initialize global logger");
}
