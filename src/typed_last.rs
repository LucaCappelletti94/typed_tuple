//! Submodule providing the `TypedLast` trait for accessing the last element of
//! a tuple by type.

use crate::prelude::*;

typed_tuple_macros::define_typed_last_trait!();

typed_tuple_macros::impl_typed_last_trait!();
