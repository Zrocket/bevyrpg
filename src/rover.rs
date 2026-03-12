use bevy::{asset::RenderAssetUsages, camera::RenderTarget, ecs::{lifecycle::HookContext, world::DeferredWorld}, prelude::*, render::render_resource::{TextureDimension, TextureFormat, TextureUsages}};
use bevy_enhanced_input::prelude::Start;
use bevy_tnua::TnuaController;

use crate::{GameState, PlayerControlScheme};

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
#[require(
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
        .add_child(rover_camera);
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

pub struct RoverPlugin;
impl Plugin for RoverPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<Rover>()
            .init_resource::<RoverCamreaRenderImage>();
    }
}

fn on_rover_forward_observer(
    trigger: On<RoverForwardEvent>,
    mut rover_query: Query<(Entity, &mut TnuaController<PlayerControlScheme>), With<Rover>>,
) {
    if let Ok((rover_entity, tnua_controller)) = rover_query.single_mut() {
    }
}

fn on_rover_backward_observer(
    trigger: On<RoverBackwardEvent>,
    rover_query: Query<Entity, With<Rover>>,
) {
}

fn on_rover_right_observer(
    trigger: On<RoverRightEvent>,
    rover_query: Query<Entity, With<Rover>>,
) {
}

fn on_rover_left_observer(
    trigger: On<RoverLeftEvent>,
    rover_query: Query<Entity, With<Rover>>,
) {
}
