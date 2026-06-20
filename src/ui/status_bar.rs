use bevy::{color::palettes::css::GREEN, ecs::{lifecycle::HookContext, world::DeferredWorld}, prelude::*, render::render_resource::AsBindGroup};

use super::*;

const HEALTH_SHADER_ASSET_PATH: &str = "shaders/health_ui_material.wgsl";
const MANA_SHADER_ASSET_PATH: &str = "shaders/mana_ui_material.wgsl";

#[derive(AsBindGroup, Asset, TypePath, Debug, Clone)]
struct SleepUiMaterial {
    /// Color multiplied with the image
    #[uniform(0)]
    color: Vec4,
    /// Represents how much of the image is visible
    /// Goes from 0 to 1
    /// A `Vec4` is used here because Bevy with webgl2 requires that uniforms are 16-byte aligned but only the first component is read.
    #[uniform(1)]
    slider: Vec4,
    /// Image used to represent the slider
    #[texture(2)]
    #[sampler(3)]
    color_texture: Handle<Image>,
    /// Color of the image's border
    #[uniform(4)]
    border_color: Vec4,
}

impl UiMaterial for SleepUiMaterial {
    fn fragment_shader() -> bevy::shader::ShaderRef {
        HEALTH_SHADER_ASSET_PATH.into()
    }
}

#[derive(AsBindGroup, Asset, TypePath, Debug, Clone)]
struct HealthUiMaterial {
    /// Color multiplied with the image
    #[uniform(0)]
    color: Vec4,
    /// Represents how much of the image is visible
    /// Goes from 0 to 1
    /// A `Vec4` is used here because Bevy with webgl2 requires that uniforms are 16-byte aligned but only the first component is read.
    #[uniform(1)]
    slider: Vec4,
    /// Image used to represent the slider
    #[texture(2)]
    #[sampler(3)]
    color_texture: Handle<Image>,
    /// Color of the image's border
    #[uniform(4)]
    border_color: Vec4,
}

impl UiMaterial for HealthUiMaterial {
    fn fragment_shader() -> bevy::shader::ShaderRef {
        HEALTH_SHADER_ASSET_PATH.into()
    }
}

#[derive(AsBindGroup, Asset, TypePath, Debug, Clone)]
struct ManaUiMaterial {
    /// Color multiplied with the image
    #[uniform(0)]
    color: Vec4,
    /// Represents how much of the image is visible
    /// Goes from 0 to 1
    /// A `Vec4` is used here because Bevy with webgl2 requires that uniforms are 16-byte aligned but only the first component is read.
    #[uniform(1)]
    slider: Vec4,
    /// Image used to represent the slider
    #[texture(2)]
    #[sampler(3)]
    color_texture: Handle<Image>,
    /// Color of the image's border
    #[uniform(4)]
    border_color: Vec4,
}

impl UiMaterial for ManaUiMaterial {
    fn fragment_shader() -> bevy::shader::ShaderRef {
        MANA_SHADER_ASSET_PATH.into()
    }
}

#[derive(Component, Reflect)]
#[require(
    Node {
        width: Val::Percent(100.),
        height: Val::Percent(10.),
        justify_self: JustifySelf::Start,
        ..default()
    },
    BackgroundColor(GREEN.into()),
    Visibility::Visible,
)]
#[component(on_add = on_ui_status_add)]
pub struct UiStatus;

#[derive(Component, Reflect)]
#[require(
    Node { ..default() },
    Button,
    Text("Player Mana".to_string()),
    TextColor(Color::WHITE),
    //BackgroundColor::from(BLUE),
    ZIndex(10),
)]
#[component(on_add = on_mana_ui_node_add)]
pub struct ManaUiNode;

#[derive(Component, Reflect)]
#[require(
    Node {
        ..default()
    },
    Button,
    Text("Player Health".to_string()),
    TextColor(Color::WHITE),
    ZIndex(10),
)]
#[component(on_add = on_health_ui_node_add)]
pub struct HealthUiNode;

#[derive(Component, Reflect)]
#[require(
    Node {
        ..default()
    },
    Button,
    Text("Player Sleep".to_string()),
    TextColor(Color::WHITE),
    ZIndex(10),
)]
#[component(on_add = on_sleep_ui_node_add)]
pub struct SleepUiNode;

pub struct StatusUIPlugin;
impl Plugin for StatusUIPlugin {
    fn build(&self, app: &mut App) {
       app
           .add_plugins(UiMaterialPlugin::<HealthUiMaterial>::default())
           .add_plugins(UiMaterialPlugin::<ManaUiMaterial>::default())
           .add_plugins(UiMaterialPlugin::<SleepUiMaterial>::default())
           .add_systems(Update, draw_status_ui.run_if(in_state(GameState::Gameplay)))
           //.add_systems(OnEnter(GameState::Gameplay), draw_status_ui.after(spawn_player_observer))
           //.add_systems(OnEnter(GameState::Gameplay), draw_status_ui.after(init_player))
           //.add_systems(OnEnter(GameState::Gameplay), draw_status_ui.after(spawn_player_observer))
           .add_systems(Update, (
                   animate_health_material,
                   animate_mana_material,
                   animate_sleep_material,
           ));
    }
}

fn on_sleep_ui_node_add(
    mut world: DeferredWorld,
    context: HookContext,
) {
    let asset_server = world.resource::<AssetServer>();
    let font = asset_server.load("FiraSans-Bold.ttf");

    let ui_assets = world.resource::<DAUiAssets>();
    let image = ui_assets.health_1.clone();

    let mut ui_materials = world.resource_mut::<Assets<SleepUiMaterial>>();
    let material = ui_materials.add(SleepUiMaterial {
        color: LinearRgba::WHITE.to_f32_array().into(),
        slider: Vec4::splat(0.5),
        color_texture: image,
        border_color: LinearRgba::WHITE.to_f32_array().into(),
    });

    world.commands()
        .entity(context.entity)
        .insert(TextFont {
            font,
            font_size: 50.0,
            ..default()
        })
        .insert((
                MaterialNode(material),
        ));
}

fn on_health_ui_node_add(
    mut world: DeferredWorld,
    context: HookContext,
) {
    let asset_server = world.resource::<AssetServer>();
    let font = asset_server.load("FiraSans-Bold.ttf");

    let ui_assets = world.resource::<DAUiAssets>();
    let image = ui_assets.health_1.clone();

    let mut ui_materials = world.resource_mut::<Assets<HealthUiMaterial>>();
    let material = ui_materials.add(HealthUiMaterial {
        color: LinearRgba::WHITE.to_f32_array().into(),
        slider: Vec4::splat(0.5),
        color_texture: image,
        border_color: LinearRgba::WHITE.to_f32_array().into(),
    });

    world.commands()
        .entity(context.entity)
        .insert(TextFont {
            font,
            font_size: 50.0,
            ..default()
        })
        .insert((
                MaterialNode(material),
        ));
}

fn on_mana_ui_node_add(
    mut world: DeferredWorld,
    context: HookContext,
) {
    let asset_server = world.resource::<AssetServer>();
    let font = asset_server.load("FiraSans-Bold.ttf");

    let ui_assets = world.resource::<DAUiAssets>();
    let image = ui_assets.mana_1.clone();

    let mut ui_materials = world.resource_mut::<Assets<ManaUiMaterial>>();
    let material = ui_materials.add(ManaUiMaterial {
        color: LinearRgba::WHITE.to_f32_array().into(),
        slider: Vec4::splat(0.5),
        color_texture: image,
        border_color: LinearRgba::WHITE.to_f32_array().into(),
    });

    world.commands()
        .entity(context.entity)
        .insert(TextFont {
            font,
            font_size: 50.0,
            ..default()
        })
    .insert((
            MaterialNode(material),
    ));
}

fn on_ui_status_add(
    mut world: DeferredWorld,
    context: HookContext,
) {
    world.commands()
        .entity(context.entity);
}

pub fn draw_status_ui(
    mut commands: Commands,
    health_query: Query<(&Health, &MaxHealth), With<Player>>,
    mana_query: Query<(&Mana, &MaxMana), With<Player>>,
    status_node_query: Query<Entity, With<UiStatus>>,
) {
    trace!("draw_status_ui");
    if let Ok(_status_bar) = status_node_query.single() {
        return;
    }
    if let Ok((_mana, _max_mana)) = mana_query.single()
    && let Ok((_health, _max_health)) = health_query.single() {
            let status_bar_node = commands
                .spawn((
                    UiStatus,
                ))
                .id();

            let player_health_node = commands
                .spawn((
                    HealthUiNode,
                ))
                .id();

            let player_mana_node = commands
                .spawn((
                    ManaUiNode,
                ))
                .id();

            let player_sleep_node = commands
                .spawn((
                    SleepUiNode,
                ))
                .id();

            commands.entity(status_bar_node).add_child(player_health_node);
            commands.entity(status_bar_node).add_child(player_mana_node);
            commands.entity(status_bar_node).add_child(player_sleep_node);
    }
}

fn animate_sleep_material(
    mut materials: ResMut<Assets<SleepUiMaterial>>,
    query: Query<&MaterialNode<SleepUiMaterial>>,
    sleep_query: Query<&Sleep, With<Player>>,
    time: Res<Time>,
) {
    //let duration = 2.0;
    for handle in &query {
        if let Some(material) = materials.get_mut(handle)
        && let Ok(sleep) = sleep_query.single() {
            let value = sleep.value as f32 / 100.0;
            // rainbow color effect
            let new_color = Color::hsl((time.elapsed_secs() * 60.0) % 360.0, 1., 0.5);
            let border_color = Color::hsl((time.elapsed_secs() * 60.0) % 360.0, 0.75, 0.75);
            material.color = new_color.to_linear().to_vec4();
            material.slider.x =
                //((time.elapsed_secs() % (duration * 2.0)) - duration).abs() / duration;
                value;
            material.border_color = border_color.to_linear().to_vec4();
        }
    }
}

fn animate_health_material(
    mut materials: ResMut<Assets<HealthUiMaterial>>,
    query: Query<&MaterialNode<HealthUiMaterial>>,
    health_query: Query<(&Health, &MaxHealth), With<Player>>,
    time: Res<Time>,
) {
    //let duration = 2.0;
    for handle in &query {
        if let Some(material) = materials.get_mut(handle)
        && let Ok((health, max_health)) = health_query.single() {
            let value = health.0 as f32 / max_health.0 as f32;
            // rainbow color effect
            let new_color = Color::hsl((time.elapsed_secs() * 60.0) % 360.0, 1., 0.5);
            let border_color = Color::hsl((time.elapsed_secs() * 60.0) % 360.0, 0.75, 0.75);
            material.color = new_color.to_linear().to_vec4();
            material.slider.x =
                //((time.elapsed_secs() % (duration * 2.0)) - duration).abs() / duration;
                value;
            material.border_color = border_color.to_linear().to_vec4();
        }
    }
}

fn animate_mana_material(
    mut materials: ResMut<Assets<ManaUiMaterial>>,
    query: Query<&MaterialNode<ManaUiMaterial>>,
    mana_query: Query<(&Mana, &MaxMana), With<Player>>,
    time: Res<Time>,
) {
    //let duration = 2.0;
    for handle in &query {
        if let Some(material) = materials.get_mut(handle)
        && let Ok((mana, max_mana)) = mana_query.single() {
            let value = mana.0 as f32 / max_mana.0 as f32;
            //println!("{:?}", value);
            // rainbow color effect
            let new_color = Color::hsl((time.elapsed_secs() * 60.0) % 360.0, 1., 0.5);
            let border_color = Color::hsl((time.elapsed_secs() * 60.0) % 360.0, 0.75, 0.75);
            material.color = new_color.to_linear().to_vec4();
            material.slider.x =
                //((time.elapsed_secs() % (duration * 2.0)) - duration).abs() / duration;
                value;
            material.border_color = border_color.to_linear().to_vec4();
        }
    }
}
