use std::borrow::Cow;

use bevy::{color::palettes::css::CRIMSON, ecs::system::IntoObserverSystem, prelude::*};

pub fn ui_root(name: impl Into<Cow<'static, str>>) -> impl Bundle {
    (
        Name::new(name),
        Node {
            position_type: PositionType::Absolute,
            width: percent(100.0),
            height: percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(20.0),
            ..default()
        },
        BackgroundColor(CRIMSON.into()),
        // Don't block picking events for other UI roots.
        Pickable::IGNORE,
    )
}

pub fn header(text: impl Into<String>) -> impl Bundle {
    (
        Name::new("Header"),
        Text(text.into()),
    )
}

pub fn label(text: impl Into<String>) -> impl Bundle {
    label_base(text, 24.0)
}

pub fn label_small(text: impl Into<String>) -> impl Bundle {
    label_base(text, 12.0)
}

pub fn label_base(text: impl Into<String>, font_size: f32) -> impl Bundle {
    (
        Name::new("Label"),
        Text(text.into())
    )
}

pub fn button<E, B, M, I>(text: impl Into<String>, action: I) -> impl Bundle 
where
    E: EntityEvent,
    B: Bundle,
    I: IntoObserverSystem<E, B, M>,
{
    button_base(
        text,
        action,
        (
            Node {
                width: Val::Px(380.0),
                height: Val::Px(80.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            BorderRadius::MAX,
        ),
    )
}

pub fn button_base<E, B, M, I>(
    text: impl Into<String>,
    action: I,
    button_bundle: impl Bundle,
) -> impl Bundle
where
    E: EntityEvent,
    B: Bundle,
    I: IntoObserverSystem<E, B, M>,
{
    let text = text.into();
    let action_observer = IntoObserverSystem::into_system(action);
    (
        Name::new("Button"),
        Node::default(),
        Children::spawn(SpawnWith(|parent: &mut ChildSpawner| {
            parent
                .spawn((
                    Name::new("Button Inner"),
                    Button,
                    children![(
                        Name::new("Button Text"),
                        Text(text),
                        TextFont::from_font_size(40.0),
                        // Don't bubble picking events from the text up to the button.
                        Pickable::IGNORE,
                    )],
            ))
            .insert(button_bundle)
            .observe(action_observer);
        })),
    )
}

pub fn large_button<E, B, M, I>(text: impl Into<String>, action: I) -> impl Bundle
where
    E: EntityEvent,
    B: Bundle,
    I: IntoObserverSystem<E, B, M>,
{
    button_base(
        text,
        action,
        (
            Node {
                width: Val::Px(380.0),
                height: Val::Px(80.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            BorderRadius::MAX,
        )
    )
}

pub fn small_button<E, B, M, I>(text: impl Into<String>, action: I) -> impl Bundle
where
    E: EntityEvent,
    B: Bundle,
    I: IntoObserverSystem<E, B, M>,
{
    button_base(
        text,
        action,
        Node {
            width: Val::Px(30.0),
            height: Val::Px(30.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
    )
}

pub fn plus_minus_bar<E, B, M, I1, I2>(
    label_marker: impl Component,
    lower: I1,
    raise: I2,
) -> impl Bundle
where
    E: EntityEvent,
    B: Bundle,
    I1: IntoObserverSystem<E, B, M>,
    I2: IntoObserverSystem<E, B, M>,
{
    (
        Node {
            justify_self: JustifySelf::Start,
            ..default()
        },
        children![
            small_button("-", lower),
            small_button("+", raise),
            (
                Node {
                    padding: UiRect::horizontal(Val::Px(10.0)),
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                children![(label(""), label_marker)],
            ),
        ],
    )
}

pub fn menu_base() -> impl Bundle {
    (
        Name::new("Menu"),
        Node {
            position_type: PositionType::Absolute,
            width: Val::Percent(80.),
            height: Val::Percent(80.),
            left: Val::Percent(10.),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_self: AlignSelf::Center,
            flex_wrap: FlexWrap::Wrap,
            display: Display::None,
            ..default()
        },
        BackgroundColor(CRIMSON.into()),
    )
}

pub fn menu_item() -> impl Bundle {
    (
        Node { ..default() },
        TextColor(Color::WHITE),
    )
}
