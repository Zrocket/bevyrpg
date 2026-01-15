use avian3d::prelude::{Collider, RigidBody};
use bevy::ecs::lifecycle::HookContext;
use bevy::ecs::world::DeferredWorld;
use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureFormat, TextureUsages};
use bevy::color::palettes::css::GOLD;
use ratatui::style::Stylize;
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};
use ratatui::{Frame, Terminal};
use soft_ratatui::embedded_graphics_unicodefonts::{mono_8x13_atlas, mono_8x13_italic_atlas, mono_8x13_bold_atlas};
use soft_ratatui::{EmbeddedGraphics, SoftBackend};
use bevy::asset::RenderAssetUsages;
use bevy::camera::RenderTarget;
use std::f32::consts::PI;

use crate::{GameState, Interactable, InteractionEvent};

#[derive(Clone, Hash, Debug, Eq, PartialEq, Default, States)]
pub enum ComputerState {
    #[default]
    MainMenu,
    Console,
}

#[derive(Event)]
pub struct UseComputerEvent {
    pub target: Entity,
}

pub struct ComputerPlugin;
impl Plugin for ComputerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SoftTerminal>()
            .register_type::<ComputerCube>()
            .register_type::<ComputerTextureCam>()
            .add_systems(OnEnter(GameState::Loading), spawn_computer);
    }
}

static FONT_DATA: &[u8] = include_bytes!("../../assets/iosevka.ttf");

#[derive(Debug, Clone, Component, Reflect)]
#[reflect(Component)]
#[component(on_add = on_computer_add)]
#[require(
    Interactable,
)]
pub struct ComputerCube;

#[derive(Debug, Clone, Component, Reflect)]
#[reflect(Component)]
pub struct ComputerTextureCam;

#[derive(Resource)]
pub struct MyProcGenMaterial(pub Handle<StandardMaterial>);

#[derive(Event)]
pub struct ChangeScreenEvent {
    pub frame_closure: fn(&mut Frame),
}

// Create resource to hold the ratatui terminal
#[derive(Resource, Deref, DerefMut)]
pub struct SoftTerminal(Terminal<SoftBackend<EmbeddedGraphics>>);
impl Default for SoftTerminal {
    fn default() -> Self {
        let font_regular = mono_8x13_atlas();
        let font_italic = mono_8x13_italic_atlas();
        let font_bold = mono_8x13_bold_atlas();
        let backend = SoftBackend::<EmbeddedGraphics>::new(
            30,
            30,
            font_regular,
            Some(font_bold),
            Some(font_italic),
            );
       //let backend = SoftBackend::new_with_font(15, 15, 12, FONT_DATA);
       //backend.set_font_size(12);
       Self(Terminal::new(backend).unwrap())
    }
}

fn on_computer_add(
    mut world: DeferredWorld,
    context: HookContext,
) {
    trace!("HOOK: on_computer_add");
    world.commands()
        .entity(context.entity)
        .observe(change_computer_screen);
}

fn change_computer_screen (
    _trigger: On<InteractionEvent>,
    mut softatui: ResMut<SoftTerminal>,
    proc_material: Res<MyProcGenMaterial>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    trace!("SYSTEM: computer_test");

    softatui.draw(new_computer_screen)
        .expect("oops");

    let width = softatui.backend().get_pixmap_width() as u32;
    let height = softatui.backend().get_pixmap_height() as u32;
    let data = softatui.backend().get_pixmap_data_as_rgba();
    let material = materials
        .get_mut(&proc_material.0)
        .expect("material not found!");

    let image = images
        .get_mut(material.base_color_texture.as_ref().unwrap().id())
        .expect("Image not found!");

    let mut temp = Image::new(
        Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        },
        bevy::render::render_resource::TextureDimension::D2,
        data,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::RENDER_WORLD | RenderAssetUsages::MAIN_WORLD,
    );
    temp.texture_descriptor.usage = TextureUsages::TEXTURE_BINDING
        | TextureUsages::COPY_DST
        | TextureUsages::RENDER_ATTACHMENT;
    *image = temp;
}

fn spawn_computer(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut softatui: ResMut<SoftTerminal>,
    mut images: ResMut<Assets<Image>>,
) {
    trace!("SYSTEM: spawn_computer");
    softatui
        .draw(draw_computer_screen)
        .expect("epic fail");

    let width = softatui.backend().get_pixmap_width() as u32;
    let height = softatui.backend().get_pixmap_height() as u32;
    let data = softatui.backend().get_pixmap_data_as_rgba();

    let mut image = Image::new(
        Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        },
        bevy::render::render_resource::TextureDimension::D2,
        data,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::RENDER_WORLD | RenderAssetUsages::MAIN_WORLD,
    );
    // You need to set these texture usage flags in order to use the image as a render target
    image.texture_descriptor.usage =
        TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST | TextureUsages::RENDER_ATTACHMENT;

    let image_handle = images.add(image);

    // Light
    commands.spawn(DirectionalLight::default());

    let texture_camera = commands
        .spawn((
            Camera2d,
            Camera {
                target: RenderTarget::Image(image_handle.clone().into()),
                ..default()
            },
            ComputerTextureCam,
            Name::new("ComputerTextureCam"),
        ))
        .id();

    commands
        .spawn((
            Node {
                // Cover the whole image
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(GOLD.into()),
            UiTargetCamera(texture_camera),
            Name::new("ComputerBox"),
        ))
        .with_children(|parent| {
            parent.spawn(ImageNode::new(image_handle.clone()));
        });

    let cube_size = 4.0;
    let cube_handle = meshes.add(Cuboid::new(cube_size, cube_size, cube_size));

    // This material has the texture that has been rendered.
    let material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(image_handle.clone()),
        reflectance: 0.02,
        unlit: false,

        ..default()
    });

    commands.insert_resource(MyProcGenMaterial(material_handle.clone()));

    // Cube with material containing the rendered UI texture.
    commands.spawn((
        Mesh3d(cube_handle),
        MeshMaterial3d(material_handle),
        Transform::from_xyz(15.0, 2.0, 1.5).with_rotation(Quat::from_rotation_y(PI)),
        Collider::cuboid(4.0, 4.0, 4.0),
        RigidBody::Static,
        ComputerCube,
        Name::new("ComputerCube"),
    ));
}

pub fn draw_computer_screen(frame: &mut Frame) {
    let area = frame.area();
    let textik = format!("Hello bevy! The window area is {}", area);
    frame.render_widget(
        Paragraph::new(textik)
            .block(Block::new().title("Ratatui").borders(Borders::ALL))
            .white()
            .on_blue()
            .wrap(Wrap { trim: false }),
        area,
    );
}

pub fn new_computer_screen(frame: &mut Frame) {
    let area = frame.area();
    let textik = format!("Hello bevy! The window area is {}", area);
    frame.render_widget(
        Paragraph::new(textik)
            .block(Block::new().title("Ratatui").borders(Borders::ALL))
            .white()
            .on_red()
            .wrap(Wrap { trim: false }),
        area,
    );
}
