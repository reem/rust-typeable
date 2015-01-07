#![deny(missing_docs)]
#![deny(warnings)]

//! Exposes `Typeable`, which exposes the `get_type` method, which gives
//! the `TypeId` of any 'static type.

use std::intrinsics::TypeId;

/// Universal mixin trait for adding a `get_type` method.
///
pub trait Typeable: 'static {
    /// Get the `TypeId` of this object.
    fn get_type(&self) -> TypeId where Self: Sized { TypeId::of::<Self>() }
}

impl<T: 'static> Typeable for T {}

