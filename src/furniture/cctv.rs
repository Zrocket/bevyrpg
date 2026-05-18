use bevy::{asset::RenderAssetUsages, camera::RenderTarget, ecs::{lifecycle::HookContext, world::DeferredWorld}, prelude::*, render::{render_resource::{TextureDimension, TextureFormat, TextureUsages}, view::{ColorGrading, ColorGradingGlobal}}};

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
    Name::new("CCTV Camera"),
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
        /*.insert((
            Camera3d::default(),
            Camera {
                order: -2,
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
        ))*/
        .observe(on_cctv_right_observer)
        .observe(on_cctv_left_observer);
}

#[derive(Component, Default, Debug)]
struct CcTvCamMovementInput {
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
           .init_resource::<CctvCamreaRenderImage>();
    }
}

fn on_cctv_right_observer(
    _trigger: On<CctvRightEvent>,
    mut cctv_query: Query<&mut CcTvCamMovementInput, With<CctvCam>>,
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
    mut cctv_query: Query<&mut CcTvCamMovementInput, With<CctvCam>>,
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
