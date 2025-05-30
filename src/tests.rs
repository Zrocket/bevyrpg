use bevy::{asset::RenderAssetUsages, prelude::*, render::{camera::RenderTarget, render_resource::{Extent3d, TextureFormat, TextureUsages}}};
use ratatui::Frame;
use soft_ratatui::SoftBackend;

use crate::{new_computer_screen, ChangeScreenEvent, ComputerTextureCam, DamageEvent, Description, Health, Inventory, Item, ItemType, MyProcGenMaterial, PickUpEvent, Player, RemoveEvent, SoftTerminal};
use super::Weight;

pub struct TestsPlugin;
impl Plugin for TestsPlugin {
    fn build(&self, app: &mut App) {
        //app
           // .add_systems(Update, (
                    //computer_test,
                    //health_test,
                    //inventory_add_test,
                    //inventory_remove_test,
            //));
    }
}

//BACKUP
/*fn computer_test(
    key: Res<ButtonInput<KeyCode>>,
    mut softatui: ResMut<SoftTerminal>,
    proc_material: Res<MyProcGenMaterial>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    trace!("SYSTEM: computer_test");

    if key.just_pressed(KeyCode::KeyK) {
        softatui.draw(new_computer_screen).expect("oops");

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
    }
}*/

fn _computer_test (
    key: Res<ButtonInput<KeyCode>>,
    mut event_writer: EventWriter<ChangeScreenEvent>
) {
    trace!("SYSTEM: computer_test");

    if key.just_pressed(KeyCode::KeyK) {
        event_writer.write(ChangeScreenEvent { frame_closure: new_computer_screen });
    }
}

fn _health_test(
    key: Res<ButtonInput<KeyCode>>,
    mut player: Query<(Entity, &Health), With<Player>>,
    mut damage_event_writer: EventWriter<DamageEvent>,
) {
    trace!("SYSTEM: health_test");
    let (player_entity, _player) = player.single_mut().unwrap();
    if key.just_pressed(KeyCode::KeyK) {
        damage_event_writer.write(DamageEvent {
            target: player_entity,
            ammount: 5,
        });
    }
}

fn _inventory_add_test(
    mut commands: Commands,
    key: Res<ButtonInput<KeyCode>>,
    mut player: Query<Entity, With<Player>>,
    mut event_writer: EventWriter<PickUpEvent>,
) {
    trace!("SYSTEM: inventory_add_test");
    let player = player.single_mut().unwrap();
    if key.just_pressed(KeyCode::KeyJ) {
        let item = commands
            .spawn((Item {
                item_type: ItemType::None,
                name: Name::new(format!("Test {}", rand::random::<u8>() as char)),
                description: Description("Test".to_string()),
                weight: Weight(0),
            },))
            .id();
        event_writer.write(PickUpEvent {
            actor: player,
            target: item,
        });
    }
}

fn _inventory_remove_test(
    key: Res<ButtonInput<KeyCode>>,
    mut player: Query<Entity, With<Player>>,
    mut inventory_query: Query<&Inventory, With<Player>>,
    mut event_writer: EventWriter<RemoveEvent>,
) {
    trace!("SYSTEM: inventory_remove_test");
    let player = player.single_mut().unwrap();
    if key.just_pressed(KeyCode::KeyL) {
        let inventory = inventory_query.single_mut().unwrap();
        let item = inventory.items.last().unwrap();
        event_writer.write(RemoveEvent {
            actor: player,
            target: *item,
        });
    }
}
