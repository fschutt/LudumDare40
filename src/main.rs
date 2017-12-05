//! A game where you have to build a pyramid from crates
//!
//! You are a character that can walk around an push crates on top of a heap.

#![windows_subsystem = "windows"]

#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(unused_doc_comment)]
#![allow(unused_variables)]
#![allow(unused_assignments)]

#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]
#![warn(trivial_numeric_casts,
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
extern crate image;
extern crate glium_text;
extern crate twox_hash;
extern crate cpal;
extern crate lewton;

pub mod input;
pub mod renderer;
pub mod context;
pub mod errors;
pub mod color;
pub mod audio;
pub mod game;
pub mod render_data;
pub mod font;
pub mod texture;
pub mod physics;
pub mod assets;
pub mod camera;
pub mod frame;
pub mod ui;
pub mod player_state;
pub mod actions;

pub type FastHashMap<T, U> = ::std::collections::HashMap<T, U, ::std::hash::BuildHasherDefault<::twox_hash::XxHash>>;
pub type FontInstanceIdMap = FastHashMap<&'static str, font::FontInstanceId>;
pub type TextureInstanceIdMap = FastHashMap<&'static str, texture::TextureId>;
pub type ShaderHashMap = FastHashMap<&'static str, ::glium::Program>;

fn main() {
    let mut game = game::Game::new(800, 600);
    game.run_main_loop();
}
