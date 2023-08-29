use std::ops::Deref;

use godot::prelude::GdRef;

pub struct MyGdRef<'a, T> {
    inner: GdRef<'a, T>,
}

impl<'a, T> MyGdRef<'a, T> {
    pub fn new(inner: GdRef<'a, T>) -> Self {
        Self { inner }
    }
}

impl<'a, T> Deref for MyGdRef<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.inner.deref()
    }
}

unsafe impl<'a, T> owning_ref::StableAddress for MyGdRef<'a, T> {}
