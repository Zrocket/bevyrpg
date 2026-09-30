use bevy::{color::palettes::css::{DARK_GREY, GREY}, prelude::*};
use bevy_egui::egui::Key::B;

pub const RICH_MAHOGANY: &str = "3C1518";
pub const DARK_GARNET: &str = "69140E";
pub const RUST_BROWN: &str = "A44200";
pub const BRONZE: &str = "D58936";
pub const VANILLA_CUSTARD: &str = "F2F3AE";

#[derive(Component)]
#[require(
    BackgroundColor::from(Srgba::hex(RICH_MAHOGANY).unwrap()),
)]
pub struct TitleBar;

#[derive(Component)]
#[require(
)]
pub struct TitleText;

#[derive(Component)]
#[require(
    BackgroundColor::from(Srgba::hex(RICH_MAHOGANY).unwrap()),
)]
pub struct TitleButtons;

#[derive(Component)]
#[require(
    BackgroundColor::from(Srgba::hex(BRONZE).unwrap()),
)]
pub struct PrimaryBackground;

#[derive(Component)]
#[require(
)]
pub struct HoverText;

#[derive(Component)]
#[require(
    BackgroundColor::from(Srgba::hex(VANILLA_CUSTARD).unwrap()),
)]
pub struct HoverBackground;

#[derive(Component)]
#[require(
)]
pub struct PrimaryText;

#[derive(Component)]
#[require(
    BackgroundColor::from(Srgba::hex(VANILLA_CUSTARD).unwrap()),
)]
pub struct SubBackground;

#[derive(Component)]
#[require(
)]
pub struct SubText;

#[derive(Component)]
#[require(
    BackgroundColor::from(Srgba::hex(BRONZE).unwrap()),
)]
pub struct IconBackground;
