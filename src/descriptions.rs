use crate::adjectives::Corpse;
use crate::nouns::{Chest, Dwarf, Log};
use crate::object::Object;
use crate::traits::{Burnable, Living, Lockable, Mineable, Openable, Rots};

// her we describe what it means to be an object.

impl Living for Object<Dwarf> {
    fn health(&self) -> f32 {
        10.0
    }
}

impl Mineable for Object<Log> {
    fn hardness(&self) -> u8 {
        1
    }
}

impl Burnable for Object<Log> {
    fn burn_time(&self) -> f32 {
        100.0
    }
}

impl Lockable for Object<Chest> {}
impl Openable for Object<Chest> {}
impl <T> Rots for Object<Corpse<T>>{
    fn rot_timer(&self) -> f32 {
        100.0
    }
}

