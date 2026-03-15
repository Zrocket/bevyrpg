use bevy::{asset::RenderAssetUsages, camera::RenderTarget, ecs::{lifecycle::HookContext, world::DeferredWorld}, prelude::*, render::render_resource::{TextureDimension, TextureFormat, TextureUsages}};
use bevy_enhanced_input::prelude::Start;
use bevy_tnua::{TnuaController, prelude::TnuaBuiltinWalk};
use bevy_landmass::{AgentTarget3d};
use avian3d::prelude::Collider;

use crate::{GameState, PlayerControlScheme, TnuaRoverController};

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
#[require(
    TnuaRoverController,
    RoverMovementInput,
    AgentTarget3d,
    Collider::cuboid(0.5, 0.5, 0.5),
)]
#[component(on_add = on_rover_add)]
pub struct Rover;

fn on_rover_add(
    mut world: DeferredWorld,
    context: HookContext,
) {
    let rover_camera = world.commands().spawn(RoverCamera).id();

    world.commands()
        .entity(context.entity)
        .add_child(rover_camera)
        .observe(on_rover_forward_observer)
        .observe(on_rover_backward_observer)
        .observe(on_rover_left_observer)
        .observe(on_rover_right_observer)
        .observe(on_rover_pickup_observer)
        .insert(Collider::cuboid(1.0, 1.0, 1.0));
}

fn spawn_rover(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
            Rover,
            Transform::from_xyz(15.0, 1.75, 15.0),
            Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
            MeshMaterial3d(materials.add(Color::WHITE)),
    ));
}

#[derive(Resource)]
pub struct RoverCamreaRenderImage(pub Handle<Image>);
impl FromWorld for RoverCamreaRenderImage {
    fn from_world(world: &mut World) -> Self {
        // Set up a texture for the 3D camrea to render to.
        // The size of the texture will be based on the viewport's ui size.
        let mut image = Image::new_uninit(
            default(),
            TextureDimension::D2,
            TextureFormat::Bgra8UnormSrgb,
            RenderAssetUsages::all(),
        );
        image.texture_descriptor.usage =
            TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST | TextureUsages::RENDER_ATTACHMENT;

        let mut images = world.resource_mut::<Assets<Image>>();
        let image_handle = images.add(image);

        Self(image_handle)
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
#[require(
    Name::new("Rover Camrea"),
)]
#[component(on_add = on_rover_camrea_add)]
pub struct RoverCamera;

fn on_rover_camrea_add(
    mut world: DeferredWorld,
    context: HookContext,
) {
    let render_image = world.resource::<RoverCamreaRenderImage>().0.clone();

    world.commands()
        .entity(context.entity)
        .insert((
            Camera3d::default(),
            Camera {
                order: -1,
                ..default()
            },
            RenderTarget::Image(render_image.into()),
        ));
}

#[derive(Component)]
pub struct ForwardHeld;

#[derive(Component)]
pub struct BackwardHeld;

#[derive(Component)]
pub struct LeftHeld;

#[derive(Component)]
pub struct RightHeld;

#[derive(EntityEvent)]
pub struct RoverForwardEvent {
    pub entity: Entity,
}

#[derive(EntityEvent)]
pub struct RoverBackwardEvent {
    pub entity: Entity,
}

#[derive(EntityEvent)]
pub struct RoverLeftEvent {
    pub entity: Entity,
}

#[derive(EntityEvent)]
pub struct RoverRightEvent {
    pub entity: Entity,
}

#[derive(EntityEvent)]
pub struct RoverPickupEvent {
    pub entity: Entity,
}

#[derive(Component, Default, Debug)]
pub struct RoverMovementInput {
    pub rotation: Quat,
    pub movement: Vec3,
}

pub struct RoverPlugin;
impl Plugin for RoverPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<Rover>()
            .init_resource::<RoverCamreaRenderImage>()
            .add_systems(OnEnter(GameState::Gameplay), spawn_rover)
            .add_systems(Update, apply_rover_movement.run_if(in_state(GameState::Gameplay)));
    }
}

fn apply_rover_movement(
    mut rover_query: Query<(&mut TnuaController<PlayerControlScheme>, &RoverMovementInput, &mut Transform), With<Rover>>,
) {
    if let Ok((mut tnua_controller, input, mut transform)) = rover_query.single_mut() {
        tnua_controller.initiate_action_feeding();

        tnua_controller.basis = TnuaBuiltinWalk {
            desired_motion: input.movement.normalize_or_zero(),
            ..default()
        };

        transform.rotate(input.rotation);
    }
}

fn on_rover_forward_observer(
    _trigger: On<RoverForwardEvent>,
    mut rover_query: Query<(&GlobalTransform, &mut RoverMovementInput), With<Rover>>,
    mut toggle: Local<bool>,
) {
    if let Ok((global_transform, mut input)) = rover_query.single_mut() {
        let mut move_to_world = Mat3::from_quat(global_transform.rotation());
        move_to_world.z_axis *= -1.0;
        move_to_world.y_axis = Vec3::Y;
        let movement_direction = move_to_world * Vec3::Z;

        if !*toggle {
            input.movement = movement_direction;
            *toggle = true;
        } else {
            input.movement = Vec3::ZERO;
            *toggle = false;
        }

    }
}

fn on_rover_backward_observer(
    _trigger: On<RoverBackwardEvent>,
    mut rover_query: Query<(&GlobalTransform, &mut RoverMovementInput), With<Rover>>,
    mut toggle: Local<bool>,
) {
    if let Ok((global_transform, mut input)) = rover_query.single_mut() {
        let mut move_to_world = Mat3::from_quat(global_transform.rotation());
        move_to_world.z_axis *= -1.0;
        move_to_world.y_axis = Vec3::Y;
        let movement_direction = move_to_world * -Vec3::Z;

        if !*toggle {
            input.movement = movement_direction;
            *toggle = true;
        } else {
            input.movement = Vec3::ZERO;
            *toggle = false;
        }
    }
}

fn on_rover_right_observer(
    _trigger: On<RoverRightEvent>,
    mut rover_query: Query<&mut RoverMovementInput, With<Rover>>,
    mut toggle: Local<bool>,
) {
    if let Ok(mut input) = rover_query.single_mut() {
        if !*toggle {
            input.rotation = Quat::from_rotation_y(-0.1);
            *toggle = true;
        } else {
            input.rotation = Quat::from_rotation_y(0.);
            *toggle = false;
        }
    }
}

fn on_rover_left_observer(
    _trigger: On<RoverLeftEvent>,
    mut rover_query: Query<&mut RoverMovementInput, With<Rover>>,
    mut toggle: Local<bool>,
) {
    if let Ok(mut input) = rover_query.single_mut() {
        if !*toggle {
            input.rotation = Quat::from_rotation_y(0.1);
            *toggle = true;
        } else {
            input.rotation = Quat::from_rotation_y(0.);
            *toggle = false;
        }
    }
}

fn on_rover_pickup_observer(
    _trigger: On<RoverPickupEvent>,
    mut rover_query: Query<(Entity, &mut TnuaController<PlayerControlScheme>, &GlobalTransform), With<Rover>>,
) {
}
