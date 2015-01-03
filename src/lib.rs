#![deny(missing_docs)]
#![deny(warnings)]

//! Exposes `Typeable`, which exposes the `get_type` method, which gives
//! the `TypeId` of any 'static type.

use std::intrinsics::TypeId;

/// Universal mixin trait for adding a `get_type` method.
///
// FIXME: Remove the `Sized` bound here, it should be unnecessary.
// Tracking: rust-lang/rust#20492
pub trait Typeable: Sized + 'static {
    /// Get the `TypeId` of this object.
    fn get_type(&self) -> TypeId { TypeId::of::<Self>() }
}

impl<T: 'static> Typeable for T {}

