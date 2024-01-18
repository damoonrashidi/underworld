pub trait Hittable {
    fn on_hit(&mut self, dmg: usize);
}
