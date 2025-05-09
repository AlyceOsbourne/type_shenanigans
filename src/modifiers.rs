use crate::adjectives::{Burning, Burnt, Corpse, Locked, Skeleton};
use crate::object::Object;
use crate::traits::{Burnable, Living, Lockable, Rots};
impl <T> Object<T> where Object<T>: Living {
    pub fn die(&self) -> Object<Corpse<T>> {
        Object::new()
    }
}
impl <T> Object<Corpse<T>> where Object<Corpse<T>>: Rots {
    pub fn decay(&self) -> Object<Skeleton<T>> {
        Object::new()
    }
}
impl <T> Object<T> where Object<T>: Lockable {
    pub fn lock(&self) -> Object<Locked<T>> {
        Object::new()
    }
}
impl <T> Object<Locked<T>> where Object<T>: Lockable {
    pub fn unlock(&self) -> Object<T>{
        Object::new()
    }
}
impl <T> Object<T> where Object<T>: Burnable {
    pub(crate) fn ignite(&self) -> Object<Burning<T>> {
        Object::new()
    }
}
impl <T> Object<Burning<T>> where Object<T>: Burnable {
    pub(crate) fn extinguish(&self) -> Object<Burnt<T>> {
        Object::new()
    }
}
