use bevy::{asset::RenderAssetUsages, camera::RenderTarget, ecs::{lifecycle::HookContext, world::DeferredWorld}, prelude::*, render::{render_resource::{TextureDimension, TextureFormat, TextureUsages}, view::{ColorGrading, ColorGradingGlobal}}};
use bevy_landmass::{AgentTarget3d};
use avian3d::{prelude::{Collider, SpatialQuery, SpatialQueryFilter}};

mod attachment;
mod battery;
mod movement;

pub use attachment::*;
pub use movement::*;

use crate::{GameState, InteractionEvent, MetaState, TnuaRoverController, add_to_inventory_observer, level::CollisionLayer, Interactable, container_interaction_observer, display_inventory_event_observer};

#[derive(EntityEvent)]
pub struct RoverRecallEvent {
    pub entity: Entity,
}

#[derive(EntityEvent)]
pub struct RoverInteractEvent {
    pub entity: Entity,
}

#[derive(EntityEvent)]
pub struct RoverDropEvent {
    pub entity: Entity,
}
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
#[type_path("api")]
pub struct RoverSpawner;

#[derive(Message)]
pub struct SpawnRoverMessage;

#[derive(Message, Default)]
pub struct RoverSpawnedMessage;

#[derive(Component, Reflect, Default)]
#[require(
    SpotLight {
        intensity: 1_000_000_000.0,
        shadows_enabled: true,
        ..default()
    },
)]
pub struct RoverFlashlight;

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
                //clear_color: Color::WHITE.into(),
                ..default()
            },
            RenderTarget::Image(render_image.into()),
            ColorGrading {
                global: ColorGradingGlobal {
                    exposure: -7.,
                    ..default()
                },
                ..default()
            },
        ));
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
#[require(
    TnuaRoverController,
    RoverMovementInput,
    AgentTarget3d,
    Collider::cuboid(0.5, 0.5, 0.5),
    Interactable,
    Name::new("Rover"),
    //RoverAttachments,
)]
#[component(on_add = on_rover_add)]
pub struct Rover;

fn on_rover_add(
    mut world: DeferredWorld,
    context: HookContext,
) {
    let rover_camera = world.commands().spawn(RoverCamera).id();

    let mut meshes = world.resource_mut::<Assets<Mesh>>();
    let rover_mesh = Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0)));

    let mut materials = world.resource_mut::<Assets<StandardMaterial>>();
    let rover_material = MeshMaterial3d(materials.add(Color::WHITE));

    world.commands()
        .entity(context.entity)
        .insert((
                RoverFlashlight,
                rover_mesh,
                rover_material,
        ))
        //.insert(related!(RoverAttachments[
        //        (FoamGunAttachment),
        //]))
        .insert(related!(RoverAttachments[
                (SampleDrillAttachment),
        ]))
        .add_child(rover_camera)
        .observe(on_rover_forward_observer)
        .observe(on_rover_backward_observer)
        .observe(on_rover_left_observer)
        .observe(on_rover_right_observer)
        .observe(add_to_inventory_observer::<Rover>)
        .observe(on_rover_interact_observer)
        .observe(on_rover_recall_observer)
        .observe(container_interaction_observer)
        .observe(display_inventory_event_observer)
        .observe(on_rover_camera_up_observer)
        .observe(on_rover_camera_down_observer);

    world.write_message_default::<RoverSpawnedMessage>();
}

fn spawn_rover(
    mut spawn_rover_message_writer: MessageWriter<SpawnRoverMessage>,
) {
    spawn_rover_message_writer.write(SpawnRoverMessage);

    /*let pickup_zone = commands.spawn((
            RoverPickupZone,
            //MeshMaterial3d(materials.add(Color::BLACK)),
    )).id();

    commands.spawn((
            Rover,
            Transform::from_xyz(15.0, 1.75, 15.0),
            Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
            MeshMaterial3d(materials.add(Color::WHITE)),
    ))
    .add_child(pickup_zone);*/
}

fn spawn_rover_observer(
    mut commands: Commands,
    mut spawn_rover_message_reader: MessageReader<SpawnRoverMessage>,
    rover_spawner_query: Query<&GlobalTransform, With<RoverSpawner>>,
    mut rover_query: Query<Entity, With<Rover>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    trace!("OBSERVER: spawn_rover_observer");

    for _message in spawn_rover_message_reader.read() {
        if let Ok(rover) = rover_query.single_mut() {
            commands.entity(rover).despawn();
        }
        let mut spawn_point = Transform::from_xyz(0.0, 50.0, 0.0);

        if let Ok(rover_spawner) = rover_spawner_query.single() {
            spawn_point.translation = rover_spawner.translation();
            spawn_point.rotation = rover_spawner.rotation();
        }

        let pickup_zone = commands.spawn((
                RoverPickupZone,
        )).id();

        commands.spawn((
                Rover,
                spawn_point,
        ))
        .add_child(pickup_zone);
    }
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

pub struct RoverPlugin;
impl Plugin for RoverPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<Rover>()
            .register_type::<RoverSpawner>()
            .init_resource::<RoverCamreaRenderImage>()
            .add_message::<SpawnRoverMessage>()
            .add_message::<RoverSpawnedMessage>()
            .add_systems(OnEnter(MetaState::Gameplay), spawn_rover)
            .add_systems(Update, (
                    apply_rover_movement.run_if(in_state(GameState::Gameplay)),
                    test_rover_interact.run_if(in_state(GameState::Gameplay)),
                    spawn_rover_observer,
                    dart_timer.run_if(in_state(GameState::Gameplay)),
            ));
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

fn on_rover_recall_observer(
    _trigger: On<RoverRecallEvent>,
    mut spawn_rover_message_writer: MessageWriter<SpawnRoverMessage>,
) {
    trace!("OBSERVER: on_rover_recall_observer");
    spawn_rover_message_writer.write(SpawnRoverMessage);
}
