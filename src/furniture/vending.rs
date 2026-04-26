use avian3d::prelude::{Collider, RigidBody};
use bevy::{ecs::{lifecycle::HookContext, world::DeferredWorld}, prelude::*};

use crate::{Drink, Food};

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
#[require(
    crate::Interactable,
)]
#[component(on_add = on_vending_machine_add)]
pub enum VendingMachine{
    Food,
    Drink,
}

fn on_vending_machine_add(
    mut world: DeferredWorld,
    context: HookContext,
) {
    world.commands()
        .entity(context.entity)
        .observe(vending_machine_interaction_observer);
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct VendTarget;

pub struct VendingPlugin;
impl Plugin for VendingPlugin {
    fn build(&self, app: &mut App) {
       app
           .register_type::<VendingMachine>();
    }
}

#[allow(clippy::too_many_arguments)]
fn vending_machine_interaction_observer(
    trigger: On<crate::InteractionEvent>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    vend_target_query: Query<&GlobalTransform, With<VendTarget>>,
    vending_machine_query: Query<&VendingMachine>,
    children_query: Query<&Children>,
    parent_query: Query<&ChildOf>,
) {
    trace!("OBSERVER: vending_machine_interaction_observer");
    if let Ok(parent) = parent_query.get(trigger.entity)
    && let Ok(vending_machine) = vending_machine_query.get(trigger.entity)
    && let Ok(vending_children) = children_query.get(parent.0) {
        for child in vending_children.iter() {
            if let Ok(vending_transform) = vend_target_query.get(child) {
                let mesh = meshes.add(Capsule3d::new(0.1, 0.1));
                let material = materials.add(Color::WHITE);

                println!("{:?}", vending_machine);

                match vending_machine {
                    VendingMachine::Food => {
                        commands.spawn((
                                Food,
                                RigidBody::Dynamic,
                                Collider::capsule(0.1, 0.1),
                                Transform::from_translation(vending_transform.translation()),
                                Mesh3d(mesh),
                                MeshMaterial3d(material),
                        ));
                    },
                    VendingMachine::Drink => {
                        commands.spawn((
                                Drink,
                                RigidBody::Dynamic,
                                Collider::capsule(0.1, 0.1),
                                Transform::from_translation(vending_transform.translation()),
                                Mesh3d(mesh),
                                MeshMaterial3d(material),
                        ));
                    }
                }
            }
        }
    }
}
