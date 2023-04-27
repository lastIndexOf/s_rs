use std::fmt::Display;

pub trait Transform {
    fn translate(&mut self, x: f32, y: f32);
}

pub trait TestTrait {}

pub struct Point<T, Y> {
    x: T,
    y: Y,
}

impl Point<f32, f32> {}
impl<T: Display> Point<T, f32> {}

impl Transform for Point<f32, f32> {
    fn translate(self: &mut Self, x: f32, y: f32) {
        // let a = *self;
        (*self).x += x;
    }
}

impl<T: Transform> TestTrait for T {}

pub fn scale_after_translate<T>(arg1: &mut T)
where
    T: Transform,
{
    arg1.translate(3.2, 2.2);
}
