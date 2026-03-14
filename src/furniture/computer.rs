use std::time::Instant;

use avian3d::prelude::RigidBody;
use bevy::asset::uuid::Uuid;
use bevy::ecs::lifecycle::HookContext;
use bevy::ecs::world::DeferredWorld;
use bevy::input::ButtonState;
use bevy::picking::PickingSystems;
use bevy::picking::backend::ray::RayMap;
use bevy::picking::pointer::{Location, PointerAction, PointerId, PointerInput};
use bevy::prelude::*;
use bevy::render::render_resource::{AsBindGroup, Extent3d, TextureFormat, TextureUsages};
use bevy::color::palettes::css::{BLUE, GOLDENROD, GRAY, GREEN, RED};
use bevy::window::{PrimaryWindow, WindowEvent};
use bevy_ingame_clock::InGameClock;
use ratatui::style::Stylize;
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};
use ratatui::{Frame, Terminal};
use soft_ratatui::embedded_graphics_unicodefonts::{mono_8x13_atlas, mono_8x13_italic_atlas, mono_8x13_bold_atlas};
use soft_ratatui::{EmbeddedGraphics, SoftBackend};
use bevy::asset::RenderAssetUsages;
use bevy::camera::RenderTarget;

use crate::widgets::floating_windows::{FloatingWindow, floating_computer_rover_window_root, floating_computer_window_root};
use crate::{Interactable, InteractionEvent, Rover, RoverBackwardEvent, RoverCamera, RoverCamreaRenderImage, RoverForwardEvent, RoverLeftEvent, RoverRightEvent, widgets};

/*
#[derive(AsBindGroup, Asset, TypePath, Debug, Clone)]
struct ComputerScreenMaterial {
}

impl UiMaterial for ComputerScreenMaterial {
    fn fragment_shader() -> bevy::shader::ShaderRef {
    }
}
*/

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

#[derive(Event)]
pub struct UseComputerEvent {
    pub target: Entity,
}

pub struct ComputerPlugin;
impl Plugin for ComputerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SoftTerminal>()
            .register_type::<ComputerScreenCube>()
            .register_type::<ComputerTextureCam>()
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

fn update_click_timer(
    mut timer_query: Query<&mut IconClickTimer>,
    time: Res<Time>,
) {
    for mut timer in timer_query.iter_mut() {
        timer.0.tick(time.delta());
    }
}

fn icon_double_click_observer(
    trigger: On<Pointer<Click>>,
    mut timer_query: Query<&mut IconClickTimer>,
    mut commands: Commands,
    computer_ui_query: Query<Entity, With<ComputerUiNode>>,
    rover_camera_query: Query<Entity, With<RoverCamera>>,
) {
    if let Ok(mut timer) = timer_query.get_mut(trigger.entity)
    && let Ok(computer_ui) = computer_ui_query.single()
    && let Ok(rover_camrea) = rover_camera_query.single() {
        if timer.0.is_finished() {
            timer.0.reset();
        } else {
            let window = commands.spawn((
                    ComputerNode,
                    floating_computer_rover_window_root("TEST".to_string(), (
                        Node {
                            width: Val::Auto,
                            height: px(300),
                            border: UiRect::all(px(5)),
                            overflow: Overflow { x: OverflowAxis::Hidden, y: OverflowAxis::Hidden },
                            flex_direction: FlexDirection::ColumnReverse,
                            ..default()
                        },
                        ViewportNode::new(rover_camrea),
                        BorderColor::all(Color::WHITE),
                        Children::spawn(SpawnWith(|root_parent: &mut ChildSpawner| {
                            // Button pane
                            root_parent.spawn((
                                Node {
                                    width: px(300),
                                    height: px(50),
                                    flex_direction: FlexDirection::Row,
                                    ..default()
                                },
                                Children::spawn(SpawnWith(|buttons_parent: &mut ChildSpawner| {
                                    // Left Buttons
                                    buttons_parent.spawn((
                                        Node {
                                            width: px(300),
                                            height: px(50),
                                            flex_direction: FlexDirection::Column,
                                            ..default()
                                        },
                                        Children::spawn(SpawnWith(|parent: &mut ChildSpawner| {
                                            parent.spawn((
                                                Node {
                                                    //width: px(300),
                                                    //height: px(50),
                                                    flex_direction: FlexDirection::Column,
                                                    ..default()
                                                },
                                                BackgroundColor(GREEN.into()),
                                                Text("FORWARD".into()),
                                            ))
                                            .observe(icon_over)
                                            .observe(icon_out)
                                            .observe(forward_pressed);
                                            parent.spawn((
                                                Node {
                                                    //width: px(300),
                                                    //height: px(50),
                                                    flex_direction: FlexDirection::Column,
                                                    ..default()
                                                },
                                                BackgroundColor(GREEN.into()),
                                                Text("BACKWARD".into()),
                                            ))
                                            .observe(icon_over)
                                            .observe(icon_out)
                                            .observe(backward_pressed);
                                        })),
                                    ));
                                    // Right Buttons
                                    buttons_parent.spawn((
                                        Node {
                                            width: px(300),
                                            height: px(50),
                                            flex_direction: FlexDirection::Column,
                                            ..default()
                                        },
                                        Children::spawn(SpawnWith(|parent: &mut ChildSpawner| {
                                            parent.spawn((
                                                Node {
                                                    //width: px(300),
                                                    //height: px(50),
                                                    flex_direction: FlexDirection::Column,
                                                    ..default()
                                                },
                                                BackgroundColor(GREEN.into()),
                                                Text("LEFT".into()),
                                            ))
                                            .observe(icon_over)
                                            .observe(icon_out)
                                            .observe(left_pressed);
                                            parent.spawn((
                                                Node {
                                                    //width: px(300),
                                                    //height: px(50),
                                                    flex_direction: FlexDirection::Column,
                                                    ..default()
                                                },
                                                BackgroundColor(GREEN.into()),
                                                Text("RIGHT".into()),
                                            ))
                                            .observe(icon_over)
                                            .observe(icon_out)
                                            .observe(right_pressed);
                                        })),
                                    ));
                                })),
                            ));
                        })),
                    )),
            )).id();

            commands.entity(computer_ui).add_child(window);
        }
    }
}

fn icon_drag_observer(
    drag: On<Pointer<Drag>>,
    mut nodes: Query<(&mut Node, &ComputedNode)>,
) {
    if let Ok((mut node, computed)) = nodes.get_mut(drag.entity) {
        node.left = Val::Px(drag.pointer_location.position.x - computed.size.x / 2.0);
        node.top = Val::Px(drag.pointer_location.position.y - 50.0);
    }
}

fn icon_over(
    over: On<Pointer<Over>>,
    mut colors: Query<&mut BackgroundColor>,
) {
    if let Ok(mut colors) = colors.get_mut(over.entity) {
        colors.0 = RED.into();
    }
}

fn icon_out(
    out: On<Pointer<Out>>,
    mut colors: Query<&mut BackgroundColor>,
) {
    if let Ok(mut colors) = colors.get_mut(out.entity) {
        colors.0 = BLUE.into();
    }
}

fn forward_pressed(
    _trigger: On<Pointer<Click>>,
    mut commands: Commands,
    rover_query: Query<Entity, With<Rover>>,
) {
    if let Ok(rover_entity) = rover_query.single() {
        commands.entity(rover_entity).trigger(|entity| RoverForwardEvent { entity });
    }
}

fn backward_pressed(
    _trigger: On<Pointer<Click>>,
    mut commands: Commands,
    rover_query: Query<Entity, With<Rover>>,
) {
    if let Ok(rover_entity) = rover_query.single() {
        commands.entity(rover_entity).trigger(|entity| RoverBackwardEvent { entity });
    }
}

fn left_pressed(
    _trigger: On<Pointer<Click>>,
    mut commands: Commands,
    rover_query: Query<Entity, With<Rover>>,
) {
    if let Ok(rover_entity) = rover_query.single() {
        commands.entity(rover_entity).trigger(|entity| RoverLeftEvent { entity });
    }
}

fn right_pressed(
    _trigger: On<Pointer<Click>>,
    mut commands: Commands,
    rover_query: Query<Entity, With<Rover>>,
) {
    if let Ok(rover_entity) = rover_query.single() {
        commands.entity(rover_entity).trigger(|entity| RoverRightEvent { entity });
    }
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

#[derive(Resource)]
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
            base_color_texture: Some(computer_image),
            reflectance: 0.02,
            unlit: false,
            ..default()
        });

        Self(material_handle.clone())
    }
}

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

/*fn change_computer_screen (
    _trigger: On<InteractionEvent>,
    mut softatui: ResMut<SoftTerminal>,
    proc_material: Res<MyProcGenMaterial>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    trace!("SYSTEM: computer_test");

    //softatui.draw(new_computer_screen)
    softatui.draw(draw_computer_screen)
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
}*/

/*pub fn draw_computer_screen(frame: &mut Frame) {
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
}*/

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

/// Because bevy has no way to know how to map a mouse input to the UI texture, we need to write a
/// system that tells it there is a pointer on the UI texture. We cast a ray into the scene and find
/// the UV (2D texture) coordinates of the raycast hit. This UV coordinate is effectively the same
/// as a pointer coordinate on a 2D UI rect.
#[allow(clippy::too_many_arguments)]
fn drive_diegetic_pointer(
    mut cursor_last: Local<Vec2>,
    mut raycast: MeshRayCast,
    rays: Res<RayMap>,
    cubes: Query<&Mesh3d, With<ComputerScreenCube>>,
    ui_camera: Query<&RenderTarget, With<Camera2d>>,
    primary_window: Query<Entity, With<PrimaryWindow>>,
    windows: Query<(Entity, &Window)>,
    images: Res<Assets<Image>>,
    manual_texture_views: Res<ManualTextureViews>,
    mut window_events: MessageReader<WindowEvent>,
    mut pointer_inputs: MessageWriter<PointerInput>,
) -> Result {
    // Get the size of the texture, so we can convert from dimensionless UV coordinates that span
    // from 0 to 1, to pixel coordinates.
    let target = ui_camera
        .single()?
        .normalize(primary_window.single().ok())
        .unwrap();
    let target_info = target
        .get_render_target_info(windows, &images, &manual_texture_views)
        .unwrap();
    let size = target_info.physical_size.as_vec2();

    // Find raycast hits and update the virtual pointer.
    let raycast_settings = MeshRayCastSettings {
        visibility: RayCastVisibility::VisibleInView,
        filter: &|entity| cubes.contains(entity),
        early_exit_test: &|_| false,
    };
    for (_id, ray) in rays.iter() {
        for (_cube, hit) in raycast.cast_ray(*ray, &raycast_settings) {
            let position = size * hit.uv.unwrap();
            if position != *cursor_last {
                pointer_inputs.write(PointerInput::new(
                    CUBE_POINTER_ID,
                    Location {
                        target: target.clone(),
                        position,
                    },
                    PointerAction::Move {
                        delta: position - *cursor_last,
                    },
                ));
                *cursor_last = position;
            }
        }
    }

    // Pipe pointer button presses to the virtual pointer on the UI texture.
    for window_event in window_events.read() {
        if let WindowEvent::MouseButtonInput(input) = window_event {
            let button = match input.button {
                MouseButton::Left => PointerButton::Primary,
                MouseButton::Right => PointerButton::Secondary,
                MouseButton::Middle => PointerButton::Middle,
                _ => continue,
            };
            let action = match input.state {
                ButtonState::Pressed => PointerAction::Press(button),
                ButtonState::Released => PointerAction::Release(button),
            };
            pointer_inputs.write(PointerInput::new(
                CUBE_POINTER_ID,
                Location {
                    target: target.clone(),
                    position: *cursor_last,
                },
                action,
            ));
        }
    }

    Ok(())
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
