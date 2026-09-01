use bevy::prelude::*;
use avian_pickup::prop::HeldProp;
use avian3d::prelude::CollisionLayers;
use bevy_common_assets::ron::RonAssetPlugin;
use serde::Deserialize;
use std::{collections::HashMap, iter};

mod ammo;
mod armor;
mod books;
mod cart;
mod consumable;
mod container;
mod drillable;
mod equip;
mod health_pack;
mod key;
mod mana_pack;
mod misc;
mod sample;
mod socket;
mod weapons;

pub use ammo::*;
pub use armor::*;
pub use books::*;
pub use cart::*;
pub use consumable::*;
pub use container::*;
pub use drillable::*;
pub use equip::*;
pub use health_pack::*;
pub use key::*;
pub use mana_pack::*;
pub use misc::*;
pub use sample::*;
pub use socket::*;
pub use weapons::*;

use crate::{AnalysisResults, BootStrap};

#[derive(Deserialize, Clone, Debug)]
pub enum ItemKind {
    Misc,
    Container,
    Sample { analysis: AnalysisResults },
    Book { contents: String },
    Ammo,
    Armor { armor_type: ArmorType, defense: i32 },
    Weapon { weapon_type: WeaponType },
    HealthPack,
    ManaPack,
}

#[derive(Asset, TypePath, Deserialize, Clone, Debug)]
pub struct ItemDefinition {
    pub id: String,
    pub name: String,
    pub description: String,
    pub weight: i32,
    pub kind: ItemKind,
}

#[derive(Asset, TypePath, Deserialize, Clone, Debug)]
pub struct ItemDefinitions(pub Vec<ItemDefinition>);

#[derive(Resource)]
pub struct ItemDefinitionsHandle(pub Handle<ItemDefinitions>);

#[derive(Resource, Default)]
pub struct ItemDatabase(pub HashMap<String, ItemDefinition>);

#[derive(Component, Reflect, Clone, Default)]
#[reflect(Component)]
pub struct RegisteredItem;

#[derive(Component, Reflect, Clone, Default)]
#[reflect(Component)]
pub struct Weight(pub i32);

#[derive(Component, Reflect, Clone, Default)]
#[reflect(Component)]
pub struct Description(pub String);

#[derive(EntityEvent)]
pub struct ItemInteractionEvent {
    entity: Entity,
}

#[derive(Component, Reflect, Clone)]
#[reflect(Component)]
pub struct ItemId(pub String);

#[derive(Component, Reflect, Clone, Default)]
#[reflect(Component)]
pub struct ItemDetails {
    pub name: String,
    pub description: Description,
    pub weight: Weight,
}

fn spawn_item_from_definition(commands: &mut Commands, def: &ItemDefinition) -> Entity {
    let mut entity = commands.spawn((
            ItemId(def.id.clone()),
            ItemDetails {
                name: def.name.clone(),
                description: Description(def.description.clone()),
                weight: Weight(def.weight),
            },
    ));

    match &def.kind {
        ItemKind::Misc                                          => { entity.insert(MiscItem); }
        ItemKind::Container                                     => { entity.insert(Container); }
        ItemKind::Sample { .. }                                 => { entity.insert(SampleItem { analyzed: false, botched: false }); }
        ItemKind::Book { contents }                     => { entity.insert(Book { title: def.name.clone(), contents: contents.clone() }); }
        ItemKind::Ammo                                          => { entity.insert(Ammo); }
        ItemKind::Armor { armor_type, defense } => { entity.insert(Armor { armor_type: armor_type.clone(), defense: defense.clone() }); }
        ItemKind::Weapon { weapon_type }            => { entity.insert(Weapon { weapon_type: weapon_type.clone() }); }
        ItemKind::HealthPack                                    => { entity.insert(HealthItem); }
        ItemKind::ManaPack                                      => { entity.insert(ManaItem); }
    }

    entity.id()
}

pub struct ItemPlugin;
impl Plugin for ItemPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<ItemDetails>()
            .register_type::<SocketItem>()
            .register_type::<PlugItem>()
            .register_type::<MountPoint>()
            .add_plugins((
                    AmmoPlugin,
                    ArmorPlugin,
                    BookPlugin,
                    MiscItemPlugin,
                    ContainerPlugin,
                    WeaponPlugin,
                    SampleItemPlugin,
                    DrillableItemPlugin,
                    SocketItemPlugin,
                    RonAssetPlugin::<ItemDefinitions>::new(&["items.ron"]),
            ))
            .add_systems(OnEnter(BootStrap::Loading), (load_items))
            .add_systems(OnEnter(BootStrap::Postload), (build_item_database))
            .add_observer(disabled_held_prop_collision)
            .add_observer(enable_dropped_prop_collision);
    }
}

fn disabled_held_prop_collision(
    add: On<Add, HeldProp>,
    children_query: Query<&Children>,
    mut collision_layers_query: Query<&CollisionLayers>,
) {
    let rigid_body = add.entity;
    for child in iter::once(rigid_body).chain(children_query.iter_descendants(rigid_body)) {
        let Ok(mut collision_layers) = collision_layers_query.get(child) else {
            continue;
        };
        //collision_layers.filters.remove(CollisionLayer::Player);
    }
}

fn enable_dropped_prop_collision(
    remove: On<Remove, HeldProp>,
    children_query: Query<&Children>,
    mut collision_layers_query: Query<&CollisionLayers>,
) {
    let rigid_body = remove.entity;
    for child in iter::once(rigid_body).chain(children_query.iter_descendants(rigid_body)) {
        let Ok(mut collision_layers) = collision_layers_query.get(child) else {
            continue;
        };
        //collision_layers.filters.add(CollisionLayer::Player);
    }
}

fn load_items(mut commands: Commands, asset_server: Res<AssetServer>) {
    let handle = asset_server.load("items.ron");
    commands.insert_resource(ItemDefinitionsHandle(handle));
}

fn build_item_database(
    mut commands: Commands,
    mut events: MessageReader<AssetEvent<ItemDefinitions>>,
    handle: Res<ItemDefinitionsHandle>,
    definitions: Res<Assets<ItemDefinitions>>,
) {
    for event in events.read() {
        if event.is_loaded_with_dependencies(&handle.0)
        && let Some(list) = definitions.get(&handle.0) {
            let db = list.0.iter()
                .map(|def| (def.id.clone(), def.clone()))
                .collect();
            commands.insert_resource(ItemDatabase(db));
        }
    }
}
