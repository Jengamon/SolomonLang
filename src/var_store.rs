use interp::{SolomonObject};
use std::sync::mpsc::{Sender, Receiver, channel};
use std::collections::HashMap;
use fnv::FnvHashMap;
use std::convert::AsRef;
//use bus::{Bus, BusReader};

enum VarMessage {
    NewObject(SolomonObject),
    //RequestAccess,
    Destroyed,
    // etc.
}

// TEMPORARY MOVE, maybe make dynamic in future?
const MAX_GENERATIONS: usize = 100;
pub type Generation = usize;

pub struct VariableRef(Sender<VarMessage>, String, usize);

pub struct VariableStore {
    generation: Generation,
    // Sender Receiver Object RefCount
    cells: Vec<(Sender<VarMessage>, Receiver<VarMessage>, SolomonObject, usize)>,
    store: FnvHashMap<String, [Option<usize>; 100]>
}

impl VariableStore {
    pub fn new() -> VariableStore {
        VariableStore {
            generation: 0,
            cells: vec![],
            store: FnvHashMap::default()
        }
    }

    pub fn new_variable<S: AsRef<str>>(&mut self, name: S) -> VariableRef {
        let (tx, rx) = channel();
        let mut generation_array = [None; MAX_GENERATIONS];

        // TODO Replace w/ algorithm that finds the smallest available index, and places it there.
        let idx = self.cells.len();
        self.cells.push((tx.clone(), rx, SolomonObject::Null, 1));

        generation_array[self.generation] = Some(idx);
        self.store.insert(name.as_ref().into(), generation_array);
        VariableRef(tx, name.as_ref().into(), self.generation)
    }

    pub fn reference<S: AsRef<str>>(&mut self, name: S) -> Result<VariableRef, String> {
        match (0..(self.generation + 1)).rev().map(|x| self._reference(name.as_ref(), x)).find(|x| x.is_some()) {
            Some(x) => Ok(x.unwrap()),
            None => Err(format!("No reference found for {}", name.as_ref()))
        }
    }

    fn _reference<S: AsRef<str>>(&mut self, name: S, gen: Generation) -> Option<VariableRef> {
        match self.store.get(name.as_ref()) {
            Some(v) => match v[gen] {
                Some(idx) => {
                    let ref s = self.cells[idx];
                    Some(VariableRef(s.0.clone(), name.as_ref().into(), self.generation))
                },
                None => None
            },
            _ => None
        }
    }
}
