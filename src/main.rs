//! A game where you have to build a pyramid from crates
//!
//! You are a character that can walk around an push crates on top of a heap.

#![windows_subsystem = "windows"]

#![allow(dead_code)]

#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]
#![allow(unused_doc_comment)]
#![warn(missing_copy_implementations,
        trivial_numeric_casts,
        trivial_casts,
        unused_extern_crates,
        unused_import_braces,
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

macro_rules! gui_error {
    ($message:expr) => (::tinyfiledialogs::message_box_ok("Error", &$message, ::tinyfiledialogs::MessageBoxIcon::Error);)
}

fn main() {
    set_up_logging();
    println!("Hello, world!");
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
