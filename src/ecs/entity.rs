use core::fmt;
use std::{u16, usize};

use super::types::{Entity, Signature, MAX_ENTITIES};

#[derive(Debug)]
pub struct EntityManagerError;

impl fmt::Display for EntityManagerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error: Maximun number of entities reached.")
    }
}

impl std::error::Error for EntityManagerError {}

pub struct EntityManager {
    active_entities_count: u16,
    available_entities: u16,
    entity_signatures: [Signature; MAX_ENTITIES],
}

impl EntityManager {
    pub fn new() -> Self {
        EntityManager {
            active_entities_count: 0,
            available_entities: u16::MAX,
            entity_signatures: [Signature::default(); MAX_ENTITIES],
        }
    }

    pub fn create_entity(&mut self) -> Result<Entity, EntityManagerError> {
        if self.active_entities_count as usize >= MAX_ENTITIES {
            return Err(EntityManagerError);
        }

        let entity = self.available_entities.trailing_zeros() as Entity;
        self.available_entities &= !(1 << entity);
        self.active_entities_count += 1;
        Ok(entity)
    }

    pub fn destroy_entity(&mut self, entity: Entity) {
        self.available_entities |= 1 << entity;
        self.active_entities_count += 1;
    }

    pub fn set_signature(&mut self, entity: Entity, signature: Signature) {
        self.entity_signatures[entity as usize] = signature;
    }

    pub fn get_signature(self, entity: Entity) -> Signature {
        self.entity_signatures[entity as usize]
    }
}
