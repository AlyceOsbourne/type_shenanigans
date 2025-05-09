use crate::adjectives::{Burning, Burnt, Corpse, Locked, Open, Skeleton};
use crate::object::Object;
use crate::traits::{Burnable, Living, Lockable, Openable, Rots};

macro_rules! transform {
    (
        $(#[$meta:meta])*
        $fn_name:ident: $from:ty => $to:ty | $bound:ty: $trait_bound:path
    ) => {
        $(#[$meta])*
        impl<T> Object<$from>
        where
            Object<$bound>: $trait_bound,
        {
            pub fn $fn_name(&self) -> Object<$to> {
                Object::new()
            }
        }
    };
}

transform!(die: T => Corpse<T> | T: Living);

transform!(decay: Corpse<T> => Skeleton<T> | Corpse<T>: Rots);

transform!(lock: T => Locked<T> | T: Lockable);

transform!(unlock: Locked<T> => T | T: Lockable);

transform!(ignite: T => Burning<T> | T: Burnable);

transform!(extinguish: Burning<T> => Burnt<T> | T: Burnable);

transform!(open: T => Open<T> | T: Openable);

transform!(close: Open<T> => T | T: Openable);
