use std::marker::PhantomData;

type Noun<U: 'static> = PhantomData<U>;

macro_rules! define_tag {
    ($name:ident) => {
        pub struct $name<T>(Noun<T>);
    };
}
macro_rules! define_double_tag {
    ($name:ident) => {
        pub struct $name<T, A>(Noun<T>, Noun<A>);
    };
}

define_tag!(Corpse);
define_tag!(Skeleton);
define_tag!(Locked);
define_tag!(Open);
define_tag!(Burning);
define_tag!(Burnt);

define_double_tag!(Doing);