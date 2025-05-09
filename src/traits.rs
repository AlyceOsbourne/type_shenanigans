pub trait Named {
    fn name(&self) -> String;
}
pub trait Mineable {
    fn hardness(&self) -> u8;
}
pub trait Living {
    fn health(&self) -> f32;
}
pub trait Rots {
    fn rot_timer(&self) -> f32;
}
pub trait Burnable {
    fn burn_time(&self) -> f32;
}
pub trait Lockable {}