use crate::nouns::{Chest, Dwarf, Goblin, Log};
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

    let goblin: Object<Goblin> = Object::new();
    let dead_goblin = goblin.die();
    let goblin_skeleton = dead_goblin.decay();

    let chest: Object<Chest> = Object::new();
    let locked_chest = chest.lock();
    let chest = locked_chest.unlock();
    let open_chest = chest.open();
    let chest = open_chest.close();

    let log: Object<Log> = Object::new();
    let burning_log = log.ignite();
    let burnt_log = burning_log.extinguish();
}