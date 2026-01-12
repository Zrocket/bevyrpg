use bevy::{color::palettes::css::{DARK_GOLDENROD, DARK_GRAY, DARK_KHAKI, DARK_OLIVEGREEN, DARK_ORCHID, LIGHT_GREEN}, picking::hover::Hovered, prelude::*};

use crate::{Book, GameState, OpenBookEvent, widgets::{floating_windows::floating_window_root}};

#[derive(Debug, Component)]
pub struct UiBook;

#[derive(EntityEvent)]
pub struct SpawnBookUiEvent {
    entity: Entity,
}

pub struct BookUiPlugin;
impl Plugin for BookUiPlugin {
    fn build(&self, app: &mut App) {
       app
           .add_systems(Update, spawn_book_ui.run_if(in_state(GameState::Gameplay)));
    }
}

pub fn spawn_book_ui(
    mut commands: Commands,
    book: Query<Entity, With<Book>>,
    _asset_server: Res<AssetServer>,
) {
    trace!("SYSTEM: spawn_inventory_ui");

    if let Ok(entity) = book.single() {
        commands.entity(entity).trigger(|entity| OpenBookEvent { entity });
    }
}

pub fn display_book_ui(
    trigger: On<OpenBookEvent>,
    mut commands: Commands,
    book_query: Query<&Book>,
) {
    if let Ok(book) = book_query.get(trigger.entity) {
        let contents = book.contents.clone();
        commands.spawn((
                floating_window_root(format!("{} Book", book.title),
                    (
                        Node {
                            flex_grow: 1.,
                            flex_direction: FlexDirection::Column,
                            overflow: Overflow { x: OverflowAxis::Hidden, y: OverflowAxis::Hidden },
                            ..default()
                        },
                        BackgroundColor::from(DARK_GRAY),
                        Children::spawn(SpawnWith(|parent: &mut ChildSpawner| {
                            parent.spawn(book_page(contents));
                            parent.spawn(
                                book_nav_bar()
                            )
                            .insert(BackgroundColor::from(DARK_ORCHID));
                        })),
                    ),
                    ),
        ));
    }
}

fn book_page(text: String) -> impl Bundle {
    (
        Node {
            width: Val::Percent(100.),
            flex_grow: 1.,
            ..default()
        },
        BackgroundColor::from(LIGHT_GREEN),
        Children::spawn(SpawnWith(|parent: &mut ChildSpawner| {
            parent.spawn(page_margin());
            parent.spawn((
                    Node {
                        flex_grow: 1.,
                        ..default()
                    },
                    Text(text),
            ));
            parent.spawn(page_margin());
        })),
    )
}

fn page_margin() -> impl Bundle {
    (
        Node {
            width: Val::Px(40.),
            ..default()
        },
        BackgroundColor::from(DARK_GOLDENROD)
    )
}

fn book_nav_bar() -> impl Bundle {
    (
        Node {
            width: Val::Percent(100.),
            height: Val::Px(30.),
            ..default()
        },
        BackgroundColor::from(DARK_KHAKI),
        Children::spawn(SpawnWith(|parent: &mut ChildSpawner| {
            parent.spawn(page_left());
            parent.spawn(page_count());
            parent.spawn(page_right());
        })),
    )
}

fn page_count() -> impl Bundle {
    (
        Node {
            flex_grow: 1.,
            justify_content: JustifyContent::Center,
            ..default()
        },
        BackgroundColor::from(DARK_OLIVEGREEN),
        Children::spawn(SpawnWith(|parent: &mut ChildSpawner| {
            parent.spawn(Text("PAGE #: X".into()));
        }))
    )
}

fn page_left() -> impl Bundle {
    (
        Node::default(),
        Children::spawn(SpawnWith(|parent: &mut ChildSpawner| {
            parent.spawn((
                    Node {
                        flex_grow: 0.,
                        aspect_ratio: Some(1.),
                        padding: px(4.).into(),
                        ..default()
                    },
                    Button,
                    Hovered::default(),
                    children![Text("<-".into())],
            ));
        })),
    )
}

fn page_right() -> impl Bundle {
    (
        Node::default(),
        Children::spawn(SpawnWith(|parent: &mut ChildSpawner| {
            parent.spawn((
                    Node {
                        flex_grow: 0.,
                        aspect_ratio: Some(1.),
                        padding: px(4.).into(),
                        ..default()
                    },
                    Button,
                    Hovered::default(),
                    children![Text("->".into())],
            ));
        })),
    )
}
