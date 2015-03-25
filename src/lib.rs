#![feature(core)]
#![deny(missing_docs)]
#![deny(warnings)]

//! Exposes `Typeable`, which exposes the `get_type` method, which gives
//! the `TypeId` of any 'static type.

use std::any::TypeId;

/// Universal mixin trait for adding a `get_type` method.
///
pub trait Typeable: 'static {
    /// Get the `TypeId` of this object.
    #[inline(always)]
    fn get_type(&self) -> TypeId { TypeId::of::<Self>() }
}

impl<T: 'static> Typeable for T {}

