use rand::random_range;
use std::collections::HashMap;

use super::IdentifierGenerator;
use crate::consts::{AdjectiveMap, NounMap};

#[derive(Debug, Clone)]
pub struct HRGenerator(HashMap<String, u16>);
impl HRGenerator {
    pub fn new() -> Self {
        HRGenerator(HashMap::<String, u16>::new())
    }
}

impl IdentifierGenerator for HRGenerator {
    fn new_id(&mut self) -> String {
        log::info!("generating new human-readable identifier");
        let adjective_ix: u8 = random_range(0..10);
        let noun_ix: u8 = random_range(0..10);

        let adjective: String = AdjectiveMap::get(adjective_ix);
        let noun: String = NounMap::get(noun_ix);

        let adjective_noun_combination: String = format!("{adjective} {noun}");
        let key = adjective_noun_combination.clone();

        let count_of_combination: Option<u16> = self.0.get(&key).copied();
        match count_of_combination {
            Some(count) => {
                let inc_count: u16 = count + 1;
                self.0.insert(key, inc_count);
                let id: String = format!("{0} {count}", adjective_noun_combination.clone());
                log::info!("{id}");
                id
            }
            None => {
                self.0.insert(adjective_noun_combination.clone(), 1);
                log::info!("{adjective_noun_combination}");
                adjective_noun_combination
            }
        }
    }

    fn new() -> Self {
        Self::new()
    }
}
