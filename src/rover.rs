use bevy::{asset::RenderAssetUsages, camera::RenderTarget, ecs::{lifecycle::HookContext, world::DeferredWorld}, prelude::*, render::render_resource::{TextureDimension, TextureFormat, TextureUsages}};
use bevy_tnua::{TnuaController, prelude::TnuaBuiltinWalk};
use bevy_landmass::{AgentTarget3d};
use avian3d::{prelude::{Collider, SpatialQuery, SpatialQueryFilter}};

use crate::{GameState, InteractionEvent, MetaState, PlayerControlScheme, TnuaRoverController, add_to_inventory_observer, level::CollisionLayer};

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
#[require(
    TnuaRoverController,
    RoverMovementInput,
    AgentTarget3d,
    Collider::cuboid(1.0, 1.0, 1.0),
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
        .observe(add_to_inventory_observer::<Rover>)
        .observe(on_rover_interact_observer)
        .observe(on_rover_return_observer);
}

fn spawn_rover(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let pickup_zone = commands.spawn((
            RoverPickupZone,
            //MeshMaterial3d(materials.add(Color::BLACK)),
    )).id();

    commands.spawn((
            Rover,
            Transform::from_xyz(15.0, 1.75, 15.0),
            Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
            MeshMaterial3d(materials.add(Color::WHITE)),
    ))
    .add_child(pickup_zone);
}

#[derive(Component)]
#[require(
    Transform::from_xyz(0., 0., -1.5),
    //Collider::cuboid(1.0, 1.0, 2.0),
)]
#[component(on_add = on_rover_pickup_zone_add)]
pub struct RoverPickupZone;

fn on_rover_pickup_zone_add(
    mut world: DeferredWorld,
    context: HookContext,
) {
    let mut meshes = world.resource_mut::<Assets<Mesh>>();
    let mesh = meshes.add(Cuboid::new(3.0, 1.0, 2.0));

    world.commands()
        .entity(context.entity)
        .insert(Mesh3d(mesh));
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

#[derive(EntityEvent)]
pub struct RoverInteractEvent {
    pub entity: Entity,
}


fn test_rover_interact(
    rover_query: Query<Entity, With<Rover>>,
    mut commands: Commands,
    key: Res<ButtonInput<KeyCode>>,
) {
    if let Ok(rover_entity) = rover_query.single()
    && key.just_pressed(KeyCode::KeyP) {
        commands.entity(rover_entity).trigger(|entity| RoverInteractEvent { entity });
    }
}

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

#[derive(Component, Default, Debug)]
pub struct RoverMovementInput {
    pub rotation: Quat,
    pub movement: Vec3,
}

#[derive(EntityEvent)]
pub struct RoverReturnEvent {
    pub entity: Entity,
}

#[derive(EntityEvent)]
pub struct RoverDropEvent {
    pub entity: Entity,
}

pub struct RoverPlugin;
impl Plugin for RoverPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<Rover>()
            .init_resource::<RoverCamreaRenderImage>()
            .add_systems(OnEnter(MetaState::Gameplay), spawn_rover)
            .add_systems(Update, apply_rover_movement.run_if(in_state(GameState::Gameplay)))
            .add_systems(Update, test_rover_interact.run_if(in_state(GameState::Gameplay)));
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

fn on_rover_interact_observer(
    _trigger: On<RoverInteractEvent>,
    mut commands: Commands,
    spatial_query: SpatialQuery,
    trigger_query: Query<&GlobalTransform, With<RoverPickupZone>>,
    rover_query: Query<Entity, With<Rover>>,
) {
    if let Ok(trigger_transform) = trigger_query.single()
    && let Ok(rover_entity) = rover_query.single() {
        let temp = spatial_query.shape_intersections(
            &Collider::cuboid(1.0, 1.0, 2.0),
            trigger_transform.translation(),
            trigger_transform.rotation(),
            &SpatialQueryFilter::from_mask(CollisionLayer::Prop)
        );
        if !temp.is_empty() {
            commands.entity(temp[0]).trigger(|entity| InteractionEvent { entity, actor: rover_entity });
        }
    }
}

fn on_rover_drop_observer(
    _trigger: On<RoverDropEvent>,
    mut commands: Commands,
    spatial_query: SpatialQuery,
    trigger_query: Query<&GlobalTransform, With<RoverPickupZone>>,
    rover_query: Query<Entity, With<Rover>>,
) {
    trace!("OBSERVER: on_rover_drop_observer");
    if let Ok(trigger_transform) = trigger_query.single()
    && let Ok(rover_entity) = rover_query.single() {
    }
}

fn on_rover_return_observer(
    _trigger: On<RoverReturnEvent>,
    mut rover_query: Query<&mut RoverMovementInput, With<Rover>>,
) {
    trace!("OBSERVER: on_rover_return_observer");
}
