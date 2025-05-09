use crate::nouns::{Chest, Dwarf, Log};
use crate::object::Object;

mod traits;
mod nouns;
mod object;
mod adjectives;
mod modifiers;
mod descriptions;

fn main() {
    let dwarf: Object<Dwarf> = Object::new();
    let dead_dwarf = dwarf.die();
    let dwarf_skeleton = dead_dwarf.decay();

    let chest: Object<Chest> = Object::new();
    let locked_chest = chest.lock();
    let chest = locked_chest.unlock();

    let log: Object<Log> = Object::new();
    let burning_log = log.ignite();
    let burnt_log = burning_log.extinguish();

    println!("{:?}", log);
    println!("{:?}", burning_log);
    println!("{:?}", burnt_log);
}