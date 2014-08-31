#![license = "MIT"]
#![deny(missing_doc)]
#![deny(warnings)]

//! Exposes `Typeable`, which exposes the `get_type` method, which gives
//! the `TypeId` of any 'static type.

use std::intrinsics::TypeId;

/// Universal mixin trait for adding a `get_type` method.
pub trait Typeable: 'static {
    /// Get the `TypeId` of this object.
    fn get_type(&self) -> TypeId { TypeId::of::<Self>() }
}

impl<T: 'static> Typeable for T {}

