use clap::Args;

#[derive(Args)]
pub struct PickArgs {
    /// Auto copy the output to the clipboard uses wl-clipboard(wayland)
    #[arg(short, long, default_value_t = false)]
    pub autocopy: bool,

    /// Define the output color type e.g. rgb, hex, cmyk, hsl
    #[arg(short, long, default_value_t = String::from("hex"))]
    pub format: String,

    /// Notify user that color is picked via notify-send + dunst or like
    #[arg(short, long, default_value_t = false)]
    pub notify: bool,

    /// Define the zoom level between 1 to 10
    #[arg(short, long, default_value_t = 3)]
    pub zoom: u8,
}

// #[derive(Clone, Debug)]
// pub struct Region {
//     pub x: u32,
//     pub y: u32,
//     pub width: u32,
//     pub height: u32,
// }
