use avian3d::prelude::RigidBody;
use bevy::asset::uuid::Uuid;
use bevy::ecs::lifecycle::HookContext;
use bevy::ecs::world::DeferredWorld;
use bevy::picking::PickingSystems;
use bevy::picking::pointer::PointerId;
use bevy::prelude::*;
use bevy::color::palettes::css::{BLUE, GRAY};
use bevy_ingame_clock::InGameClock;
use bevy_old_tv_shader::OldTvPlugin;

use crate::{Interactable};

mod computer_input;
mod computer_display;
//mod crt_shader;

use computer_input::*;
use computer_display::*;
//use crt_shader::*;

#[derive(Component, Reflect)]
#[reflect(Component)]
#[require(
    Interactable,
    Name::new("Desktop"),
)]
#[component(on_add = on_desktop_add)]
pub struct Desktop;

fn on_desktop_add(
    mut world: DeferredWorld,
    context: HookContext,
) {
    let my_material = world.resource::<MyProcGenMaterial>().0.clone();

    world.commands()
        .entity(context.entity)
        .remove::<MeshMaterial3d<StandardMaterial>>()
        .insert(MeshMaterial3d(my_material));
}

#[derive(Component)]
pub struct ComputerNode;

#[derive(Component)]
#[require(
    Node {
        position_type: PositionType::Absolute,
        width: Val::Percent(10.),
        height: Val::Percent(15.),
        align_items: AlignItems::Center,
        border_radius: BorderRadius::all(Val::Px(10.)),
        left: Val::Px(20.),
        top: Val::Px(400.),
        flex_direction: FlexDirection::Column,
        overflow: Overflow { x: OverflowAxis::Hidden, y: OverflowAxis::Hidden },
        ..default()
    },
    BackgroundColor(BLUE.into()),
    IconClickTimer(Timer::from_seconds(1.0, TimerMode::Once)),
)]
#[component(on_add = on_computer_icon_add)]
pub struct ComputerIcon;

#[derive(Component)]
pub struct IconClickTimer(pub Timer);

fn on_computer_icon_add(
    mut world: DeferredWorld,
    context: HookContext,
) {
    let icon: Handle<Image> = world.resource::<AssetServer>().load("icons/geometrica/save-block.png");

    let icon_node = world.commands().spawn((
        Node {
            width: Val::Auto,
            height: Val::Percent(75.),
            ..default()
        },
        //BackgroundColor(GREEN.into()),
        //Text::new("TEST"),
        ImageNode::new(icon),
    )).id();

    let text_node = world.commands().spawn((
        Node {
            width: Val::Auto,
            height: Val::Percent(25.),
            ..default()
        },
        //BackgroundColor(GOLDENROD.into()),
        Text::new("TEST"),
    )).id();

    world.commands()
        .entity(context.entity)
        .observe(icon_drag_observer)
        .observe(icon_over)
        .observe(icon_out)
        .observe(icon_double_click_observer)
        .add_child(icon_node)
        .add_child(text_node);
}

#[derive(Component)]
#[require(
    Node {
        position_type: PositionType::Absolute,
        width: Val::Percent(25.),
        height: Val::Percent(15.),
        align_items: AlignItems::Center,
        padding: UiRect::all(Val::Px(20.)),
        border_radius: BorderRadius::all(Val::Px(10.)),
        left: Val::Px(380.),
        top: Val::Px(50.),
        ..default()
    },
    Text("".into()),
    BackgroundColor(BLUE.into()),
)]
#[component(on_add = on_computer_clock_add)]
pub struct ComputerClock;

fn on_computer_clock_add(
    mut world: DeferredWorld,
    context: HookContext,
) {
    world.commands()
        .entity(context.entity)
        .observe(icon_drag_observer)
        .observe(icon_over)
        .observe(icon_out);
}

// Marks the cube, to which the UI texture is applied.
#[derive(Component, Reflect)]
#[reflect(Component)]
#[require(
    Interactable,
    RigidBody::Static,
    Transform::from_xyz(15.0, 2.0, 1.5),
    Name::new("ComputerCube"),
)]
#[component(on_add = on_cube_screen_add)]
struct ComputerScreenCube;

fn on_cube_screen_add(
    mut world: DeferredWorld,
    context: HookContext,
) {
    let my_material = world.resource::<MyProcGenMaterial>().0.clone();
    let mut meshes = world.resource_mut::<Assets<Mesh>>();

    let cube_size = 4.0;
    let cube_handle = meshes.add(Cuboid::new(cube_size, cube_size, cube_size));

    world.commands()
        .entity(context.entity)
        .insert(MeshMaterial3d(my_material))
        .insert(Mesh3d(cube_handle));
}

const CUBE_POINTER_ID: PointerId = PointerId::Custom(Uuid::from_u128(90870987));

#[derive(Clone, Hash, Debug, Eq, PartialEq, Default, States)]
pub enum ComputerState {
    #[default]
    MainMenu,
    Console,
}

pub struct ComputerPlugin;
impl Plugin for ComputerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(OldTvPlugin)
            .register_type::<ComputerScreenCube>()
            .register_type::<ComputerTextureCam>()
            .register_type::<Desktop>()
            .init_resource::<ComputerImage>()
            .init_resource::<MyProcGenMaterial>()
            .add_systems(Startup, setup)
            .add_systems(First, drive_diegetic_pointer.in_set(PickingSystems::Input))
            .add_systems(Update, update_click_timer)
            .add_systems(Update, display_time);
    }
}

#[derive(Debug, Clone, Component, Reflect)]
#[reflect(Component)]
#[require(
    Node {
        // Cover the whole image
        width: percent(100),
        height: percent(100),
        flex_direction: FlexDirection::Column,
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    },
    BackgroundColor(GRAY.into()),
    Name::new("ComputerUiNode"),
)]
#[component(on_add = on_computer_ui_node_add)]
pub struct ComputerUiNode;

fn on_computer_ui_node_add(
    mut world: DeferredWorld,
    context: HookContext,
) {
    world.commands()
        .entity(context.entity)
        .with_children(|parent| {
            parent
                .spawn((
                    ComputerIcon,
                ));
            parent
                .spawn((
                    ComputerClock,
                ));
        });
}

fn setup(
    mut commands: Commands,
) {
    let texture_camera = commands
        .spawn(ComputerTextureCam)
        .id();

    commands
        .spawn((
            ComputerUiNode,
            UiTargetCamera(texture_camera),
        ));

    // Cube with material containing the rendered UI texture.
    commands.spawn(ComputerScreenCube);

    commands.spawn(CUBE_POINTER_ID);
}

fn display_time(
    clock: Res<InGameClock>,
    mut clock_text_query: Query<&mut Text, With<ComputerClock>>,
) {
    if let Ok(mut clock_text) = clock_text_query.single_mut() {
        clock_text.0 = format!(
            "{}",
            clock.format_datetime(None)
        );
    }
}
