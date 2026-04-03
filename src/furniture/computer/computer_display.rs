use bevy::{asset::RenderAssetUsages, camera::RenderTarget, ecs::{lifecycle::HookContext, world::DeferredWorld}, gltf::GltfMaterialName, prelude::*, render::{extract_resource::ExtractResource, render_resource::{Extent3d, TextureFormat, TextureUsages}}};
use bevy_old_tv_shader::prelude::OldTvSettings;

#[derive(Resource, Clone, ExtractResource)]
pub struct ComputerImage(pub Handle<Image>);
impl FromWorld for ComputerImage {
    fn from_world(world: &mut World) -> Self {
        //let mut softatui = world.resource_mut::<SoftTerminal>();

        //let width = softatui.backend().get_pixmap_width() as u32;
        //let height = softatui.backend().get_pixmap_height() as u32;
        //let data = softatui.backend().get_pixmap_data_as_rgba();

        //softatui.draw(draw_computer_screen)
        //    .expect("oops");

        //let mut image = Image::new(
        let mut image = Image::new_fill(
            Extent3d {
                //width,
                width: 512,
                //height,
                height: 512,
                depth_or_array_layers: 1,
            },
            bevy::render::render_resource::TextureDimension::D2,
            //data,
            &[0, 0, 0, 0],
            TextureFormat::Rgba8UnormSrgb,
            //RenderAssetUsages::RENDER_WORLD | RenderAssetUsages::MAIN_WORLD,
            RenderAssetUsages::default()
        );
        // You need to set these texture usage flags in order to use the image as a render target
        image.texture_descriptor.usage =
            TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST | TextureUsages::RENDER_ATTACHMENT;

        let mut images = world.resource_mut::<Assets<Image>>();

        let image_handle = images.add(image);

        Self(image_handle)
    }
}

#[derive(Resource)]
pub struct MyProcGenMaterial(pub Handle<StandardMaterial>);
impl FromWorld for MyProcGenMaterial {
    fn from_world(world: &mut World) -> Self {
        let computer_image = world.resource::<ComputerImage>();
        let computer_image = computer_image.0.clone();

        let mut materials = world.resource_mut::<Assets<StandardMaterial>>();

        let material_handle = materials.add(StandardMaterial {
            base_color_texture: Some(computer_image.clone()),
            reflectance: 0.02,
            unlit: false,
            emissive: Color::WHITE.into(),
            emissive_texture: Some(computer_image),
            ..default()
        });

        Self(material_handle.clone())
    }
}

#[derive(Resource)]
pub struct ComputerScreenMaterial(pub Handle<StandardMaterial>);

fn poll_screen_material_loaded(
    mut commands: Commands,
    models: Res<Assets<Gltf>>,
) {
}

#[derive(Debug, Clone, Component, Reflect)]
#[reflect(Component)]
#[require(
    Camera2d,
    Camera {
        order: -1,
        ..default()
    },
    Name::new("ComputerTextureCam"),
    OldTvSettings {
        //screen_shape_factor: 0.0004,
        screen_shape_factor: 0.01,
        rows: 16000.0,
        //rows: 1000000.0,
        brightness: 5.0,
        edges_transition_size: 0.025,
        channels_mask_min: 0.0,
    }
)]
#[component(on_add = on_computer_texture_cam_add)]
pub struct ComputerTextureCam;

fn on_computer_texture_cam_add(
    mut world: DeferredWorld,
    context: HookContext,
) {
    let computer_image = world.resource::<ComputerImage>().0.clone();

    world.commands()
        .entity(context.entity)
        .insert(RenderTarget::Image(computer_image.into()));
}
