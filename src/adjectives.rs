use std::marker::PhantomData;

type Noun<U: 'static> = PhantomData<U>;

macro_rules! define_tag {
    ($name:ident) => {
        pub struct $name<T>(Noun<T>);
    };
}

define_tag!(Corpse);
define_tag!(Skeleton);
define_tag!(Locked);
define_tag!(Burning);
define_tag!(Burnt);

struct Doing<T, A> (Noun<T>, Noun<T>);