use std::ops::Deref;

use godot::prelude::*;

pub struct MyGd<T: GodotClass> {
    inner: Gd<T>,
}

impl<T: GodotClass> MyGd<T> {
    pub fn new(inner: Gd<T>) -> Self {
        Self { inner }
    }
}

impl<T: GodotClass> Deref for MyGd<T> {
    type Target = Gd<T>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

unsafe impl<T: GodotClass> owning_ref::StableAddress for MyGd<T> {}

pub struct MyGdRef<'a, T: GodotClass> {
    inner: GdRef<'a, T>,
}

impl<'a, T: GodotClass> MyGdRef<'a, T> {
    pub fn new(inner: GdRef<'a, T>) -> Self {
        Self { inner }
    }
}

impl<'a, T: GodotClass> Deref for MyGdRef<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.inner.deref()
    }
}

unsafe impl<'a, T: GodotClass> owning_ref::StableAddress for MyGdRef<'a, T> {}
