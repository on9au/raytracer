//! color

use std::ops::{Deref, DerefMut};

use crate::vec3::Vec3;

pub struct Color(Vec3);

impl DerefMut for Color {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Deref for Color {
    type Target = Vec3;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Color {
    pub fn new(inner: impl Into<Vec3>) -> Self {
        let inner = inner.into();
        Self(inner)
    }
}
