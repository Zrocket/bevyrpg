use avian3d::collision::collider::Collider;
use bevy::{asset::RenderAssetUsages, camera::RenderTarget, ecs::{lifecycle::HookContext, world::DeferredWorld}, prelude::*, render::{render_resource::{TextureDimension, TextureFormat, TextureUsages}, view::{ColorGrading, ColorGradingGlobal}}};
use bevy_tnua::{TnuaController, builtins::TnuaBuiltinWalk};

use crate::{GameState, PlayerControlScheme};

#[derive(Resource)]
pub struct CctvCamreaRenderImage(pub Handle<Image>);
impl FromWorld for CctvCamreaRenderImage {
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
    Name::new("CCTV Parent"),
    CctvCamMovementInput,
    crate::TnuaCctvController,
)]
#[component(on_add = on_cctv_parent_add)]
pub struct CctvParent;

fn on_cctv_parent_add(
    mut world: DeferredWorld,
    context: HookContext,
) {
    let render_image = world.resource::<CctvCamreaRenderImage>().0.clone();

    world.commands()
        .entity(context.entity)
        .insert(RenderTarget::Image(render_image.into()))
        .observe(on_cctv_right_observer)
        .observe(on_cctv_left_observer);
}

#[derive(Component, Reflect)]
#[reflect(Component)]
#[require(
    Name::new("CCTV Camera"),
    //CctvCamMovementInput,
    Camera3d::default(),
    Camera {
        order: -2,
        ..default()
    },
    ColorGrading {
        global: ColorGradingGlobal {
            exposure: -7.,
            ..default()
        },
        ..default()
    },
    //crate::TnuaCctvController,
)]
#[component(on_add = on_cctv_cam_add)]
pub struct CctvCam;

fn on_cctv_cam_add(
    mut world: DeferredWorld,
    context: HookContext,
) {
    let render_image = world.resource::<CctvCamreaRenderImage>().0.clone();

    world.commands()
        .entity(context.entity)
        .insert(RenderTarget::Image(render_image.into()))
        .observe(on_cctv_right_observer)
        .observe(on_cctv_left_observer);
}

#[derive(Component, Default, Debug)]
pub struct CctvCamMovementInput {
    pub rotation: Quat,
    pub movement: Vec3,
}

#[derive(EntityEvent)]
pub struct CctvLeftEvent {
    pub entity: Entity,
}

#[derive(EntityEvent)]
pub struct CctvRightEvent {
    pub entity: Entity,
}

#[derive(EntityEvent)]
pub struct CctvUpEvent {
    pub entity: Entity,
}

#[derive(EntityEvent)]
pub struct CctvDownEvent {
    pub entity: Entity,
}

pub(crate) struct CctvPlugin;
impl Plugin for CctvPlugin {
    fn build(&self, app: &mut App) {
       app
           .register_type::<CctvCam>()
           .register_type::<CctvParent>()
           .init_resource::<CctvCamreaRenderImage>()
           .add_systems(Update,
               apply_cctv_movement.run_if(in_state(GameState::Gameplay)),
               );
    }
}

fn on_cctv_right_observer(
    _trigger: On<CctvRightEvent>,
    mut cctv_query: Query<&mut CctvCamMovementInput, With<CctvParent>>,
    mut toggle: Local<bool>
) {
    if let Ok(mut input) = cctv_query.single_mut() {
        if !*toggle {
            input.rotation = Quat::from_rotation_y(-0.01);
            *toggle = true;
        } else {
            input.rotation = Quat::from_rotation_y(0.);
            *toggle = false;
        }
    }
}

fn on_cctv_left_observer(
    _trigger: On<CctvLeftEvent>,
    mut cctv_query: Query<&mut CctvCamMovementInput, With<CctvParent>>,
    mut toggle: Local<bool>
) {
    if let Ok(mut input) = cctv_query.single_mut() {
        if !*toggle {
            input.rotation = Quat::from_rotation_y(0.01);
            *toggle = true;
        } else {
            input.rotation = Quat::from_rotation_y(0.);
            *toggle = false;
        }
    }
}

pub(crate) fn apply_cctv_movement(
    mut query: Query<(&mut TnuaController<PlayerControlScheme>, &CctvCamMovementInput, &mut Transform), With<CctvParent>>,
) {
    if let Ok((mut tnua_controller, input, mut transform)) = query.single_mut() {
        tnua_controller.initiate_action_feeding();

        tnua_controller.basis = TnuaBuiltinWalk {
            desired_motion: input.movement.normalize_or_zero(),
            ..default()
        };

        transform.rotate(input.rotation);
    }
}
