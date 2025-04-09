
pub struct Assets;

impl Assets {
    // Favicon
    pub const FAVICON: &'static [u8] = include_bytes!("../.././assets/favicon.ico");
    
    // CSS Files
    pub const TAILWIND_CSS: &'static str = include_str!("../.././assets/tailwind.css");
    
    // You can add other assets similarly
    pub const HEADER_SVG: &'static str = include_str!("../.././assets/header.svg");
}