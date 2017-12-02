macro_rules! gui_error {
    ($message:expr) => (::tinyfiledialogs::message_box_ok("Error", &$message, ::tinyfiledialogs::MessageBoxIcon::Error);)
}

#[derive(Debug, Copy, Clone)]
pub struct Error {

}
