use specs::prelude::*;

use super::{colors::*, EntityMoved, EntryTrigger, game_log::GameLog, glyph_index::POW_GLYPH, Hidden, InflictsDamage, Map, Name, particle_system::ParticleBuilder, Position, SingleActivation, SufferDamage };

pub struct TriggerSystem {}

impl<'a> System<'a> for TriggerSystem {
    type SystemData = ( ReadExpect<'a, Map>,
                        WriteStorage<'a, EntityMoved>,
                        ReadStorage<'a, Position>,
                        ReadStorage<'a, EntryTrigger>,
                        WriteStorage<'a, Hidden>,
                        ReadStorage<'a, Name>,
                        Entities<'a>,
                        WriteExpect<'a, GameLog>,
                        ReadStorage<'a, InflictsDamage>,
                        WriteExpect<'a, ParticleBuilder>,
                        WriteStorage<'a, SufferDamage>,
                        ReadStorage<'a, SingleActivation>
                       );
    
    fn run(&mut self, data: Self::SystemData) {
        let (map, mut entity_moved, position, entry_trigger, mut hidden, names, entities, mut log,
             inflicts_damage, mut particle_builder, mut inflict_damage, single_activation) = data;
 
        // Iterate the entities that moved and their final position
        let mut remove_entities: Vec<Entity> = Vec::new();
        for (entity, mut _entity_moved, pos) in (&entities, &mut entity_moved, &position).join() {
            let idx = map.xy_idx(pos.x, pos.y);
            for entity_id in map.tile_content[idx].iter() {
                if entity != *entity_id { // Do not bother to check yourself for being a trap!
                    let maybe_trigger = entry_trigger.get(*entity_id);
                    match maybe_trigger {
                        None => {},
                        Some(_trigger) => {
                            // We triggered it
                            let name = names.get(*entity_id);
                            if let Some(name) = name {
                                log.entries.push(format!("{} triggers!", &name.name));
                            }

                            hidden.remove(*entity_id); // The trap is no longer hidden

                            // I the trap is damage inflicting, do it
                            let damage = inflicts_damage.get(*entity_id);
                            if let Some(damage) = damage {
                                particle_builder.request(pos.x, pos.y, return_u8_rgb(DMG_FG), return_u8_rgb(DEFAULT_BG), rltk::to_cp437(POW_GLYPH), 200.0);
                                SufferDamage::new_damage(&mut inflict_damage, entity, damage.damage);
                            }

                            // If it is single activation, it needs to be removed
                            let sa = single_activation.get(*entity_id);
                            if let Some(_sa) = sa {
                                remove_entities.push(*entity_id);
                            }
                        }
                    }
                }
            }
        }

        // Remove any single activation traps
        for trap in remove_entities.iter() {
            entities.delete(*trap).expect("Unable to delete trap");
        }

        // Remove all entity movement markers
        entity_moved.clear();
    }
}