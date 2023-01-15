//! `Id` is a typed wrapper around a `uuid::Uuid`.
//!
//! Use it to add type safety and prevent confusion between different kinds of Uuid.
//!
//! # Example
//! Represent different types of Id to prevent mixups or invalid states. If describing
//! a unique resource's relationship to another, for example the `Role` a `User` has,
//! the relationship can be expressed as follows:
//! ```rust
//! # mod submodule {
//! # struct User;
//! # struct Role;
//! // Subtype the Id type to specify the version of the Id, instead
//! // of repeating yourself everywhere.
//! type Id<T> = typed_uuid::Id<T, typed_uuid::V4>;
//!
//! struct Relation {
//!     user: Id<User>,
//!     role: Id<Role>,
//! }
//! # }
//! ```
//! `Id`s with different `T` parameter types are incompatible, and cannot be compared.
//!
//! Attempting to assign an `Id<User>` to a variable of type `Id<Role>` is a compilation error.
//! ```rust,compile_fail
//! # mod submodule {
//! # struct User;
//! # struct Role;
//! # type Id<T> = typed_uuid::Id<T, typed_uuid::V4>;
//! # fn do_thing() {
//! let user = Id::<User>::new();
//! let role = Id::<Role>::new();
//!
//! // Compilation fails here, can't compare Id<User> and Id<Role>
//! assert_eq!(user, role);
//! # }
//! # }
//! ```
//!
//! But `Id`s of the same type work:
//! ```rust
//! # mod submodule {
//! # struct User;
//! # type Id<T> = typed_uuid::Id<T, typed_uuid::V4>;
//! # fn do_thing() {
//! let mut first = Id::<User>::new();
//! let second = Id::<User>::new();
//!
//! first = second;
//! assert_eq!(first, second);
//! # }
//! # }
//! ```
//! # Usage
//! When depending on this library, you need to explicitly select the versions of the uuid, you will be using, as well as optionally `serde` support:
//! ```toml
//! [dependencies.typed-uuid]
//! version = "*"
//! default-features = false
//! features = ["v4", "serde"]
//! ```
#![no_std]
#![deny(
    bad_style,
    dead_code,
    improper_ctypes,
    non_shorthand_field_patterns,
    no_mangle_generic_items,
    overflowing_literals,
    path_statements,
    patterns_in_fns_without_body,
    private_in_public,
    unconditional_recursion,
    unused,
    unused_allocation,
    unused_comparisons,
    unused_parens,
    while_true,
    missing_debug_implementations,
    missing_docs,
    trivial_casts,
    trivial_numeric_casts,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]
#![forbid(unsafe_code)]

use core::{marker::PhantomData, ops::Deref};
pub use uuid;
use uuid::Uuid;

/// Errors which might occur when using [`Id`].
#[derive(Debug, Clone, Copy)]
pub enum Error {
    /// Attempted to create an [`Id<T, Version>`] where the generic [`Uuid`] being converted from
    /// was of a different Uuid version, than the one specified in the [`Id`] type.
    WrongVersion {
        /// Expected version, this is equivalent to the `Version` field of the [`Id`] type
        expected: usize,
        /// Actual version of the provided [`Uuid`]
        actual: usize,
    },
}

/// Typed wrapper around a [`Uuid`], supports same versions of Uuid as the `uuid` crate trough the `Version` parameter.
#[derive(Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Id<T, Version>(
    Uuid,
    #[cfg_attr(feature = "serde", serde(skip))] PhantomData<(T, Version)>,
);

impl<T, Version> Copy for Id<T, Version> {}

impl<T, Version> Clone for Id<T, Version> {
    fn clone(&self) -> Self {
        Self(self.0, self.1)
    }
}

impl<T, Version> core::fmt::Debug for Id<T, Version> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("Id").field(&self.0).finish()
    }
}

impl<T, Version> core::fmt::Display for Id<T, Version> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<T, Version> core::hash::Hash for Id<T, Version> {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl<T, Version> AsRef<Uuid> for Id<T, Version> {
    fn as_ref(&self) -> &Uuid {
        &self.0
    }
}

impl<T, Version> Deref for Id<T, Version> {
    type Target = Uuid;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T, Version> PartialEq<Id<T, Version>> for Id<T, Version> {
    fn eq(&self, other: &Id<T, Version>) -> bool {
        self.0 == other.0
    }
}

impl<T, Version> PartialEq<Uuid> for Id<T, Version> {
    fn eq(&self, other: &Uuid) -> bool {
        &self.0 == other
    }
}

#[cfg(feature = "v1")]
pub use v1::V1;

#[cfg(feature = "v3")]
pub use v3::V3;

#[cfg(feature = "v4")]
pub use v4::V4;

#[cfg(feature = "v5")]
pub use v5::V5;

#[cfg(all(unstable_uuid, feature = "v6"))]
pub use v6::V6;

#[cfg(all(unstable_uuid, feature = "v7"))]
pub use v7::V7;

#[cfg(all(unstable_uuid, feature = "v8"))]
pub use v8::V8;

#[cfg(feature = "v1")]
mod v1 {
    use crate::{Error, Id};
    use core::marker::PhantomData;
    use uuid::{Timestamp, Uuid};

    /// Denotes that the contained Uuid is of type V1
    #[derive(Debug)]
    pub struct V1;

    impl<T> Id<T, V1> {
        /// Construct a new typed v1 Uuid
        #[allow(clippy::new_without_default)]
        pub fn new(ts: Timestamp, node_id: &[u8; 6]) -> Self {
            Self(Uuid::new_v1(ts, node_id), PhantomData::default())
        }

        /// Attempt to coerce a generic [`Uuid`] into a typed [`Id`]
        ///
        /// Returns `Err(Error::WrongVersion)` if the generic Uuid version
        /// is not v1
        pub fn from_generic_uuid(uuid: Uuid) -> Result<Self, Error> {
            if uuid.get_version_num() == 1 {
                Ok(Id(uuid, PhantomData::default()))
            } else {
                Err(Error::WrongVersion {
                    expected: 1,
                    actual: uuid.get_version_num(),
                })
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::V1;
        use crate::Id;
        use uuid::Timestamp;

        #[test]
        fn new() {
            let context = uuid::timestamp::context::Context::new_random();
            let _ = Id::<u32, V1>::new(Timestamp::now(&context), &[0u8; 6]);
        }
    }
}

#[cfg(feature = "v3")]
mod v3 {
    use crate::{Error, Id, Uuid};
    use core::marker::PhantomData;

    /// Denotes that the contained Uuid is of type V3
    #[derive(Debug)]
    pub struct V3;

    impl<T> Id<T, V3> {
        /// Construct a new typed v3 Uuid
        #[allow(clippy::new_without_default)]
        pub fn new(namespace: &Uuid, name: &[u8]) -> Self {
            Self(Uuid::new_v3(namespace, name), PhantomData::default())
        }

        /// Attempt to coerce a generic [`Uuid`] into a typed [`Id`]
        ///
        /// Returns `Err(Error::WrongVersion)` if the generic Uuid version
        /// is not v3
        pub fn from_generic_uuid(uuid: Uuid) -> Result<Self, Error> {
            if uuid.get_version_num() == 3 {
                Ok(Id(uuid, PhantomData::default()))
            } else {
                Err(Error::WrongVersion {
                    expected: 3,
                    actual: uuid.get_version_num(),
                })
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::V3;
        use crate::Id;
        use uuid::Uuid;

        #[test]
        fn new() {
            let _ = Id::<u32, V3>::new(&Uuid::NAMESPACE_DNS, &[0u8; 6]);
        }
    }
}

#[cfg(feature = "v4")]
mod v4 {
    use crate::{Error, Id, Uuid};
    use core::marker::PhantomData;

    /// Denotes that the contained Uuid is of type V4
    #[derive(Debug)]
    pub struct V4;

    impl<T> Id<T, V4> {
        /// Construct a new typed v4 Uuid
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self(Uuid::new_v4(), PhantomData::default())
        }

        /// Attempt to coerce a generic [`Uuid`] into a typed [`Id`]
        ///
        /// Returns `Err(Error::WrongVersion)` if the generic Uuid version
        /// is not v4
        pub fn from_generic_uuid(uuid: Uuid) -> Result<Self, Error> {
            if uuid.get_version_num() == 4 {
                Ok(Id(uuid, PhantomData::default()))
            } else {
                Err(Error::WrongVersion {
                    expected: 4,
                    actual: uuid.get_version_num(),
                })
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::V4;
        use crate::Id;

        #[test]
        fn new() {
            let _ = Id::<u32, V4>::new();
        }
    }
}

#[cfg(feature = "v5")]
mod v5 {
    use crate::{Error, Id, Uuid};
    use core::marker::PhantomData;

    /// Denotes that the contained Uuid is of type V5
    #[derive(Debug)]
    pub struct V5;

    impl<T> Id<T, V5> {
        /// Construct a new typed v5 Uuid
        #[allow(clippy::new_without_default)]
        pub fn new(namespace: &Uuid, name: &[u8]) -> Self {
            Self(Uuid::new_v5(namespace, name), PhantomData::default())
        }

        /// Attempt to coerce a generic [`Uuid`] into a typed [`Id`]
        ///
        /// Returns `Err(Error::WrongVersion)` if the generic Uuid version
        /// is not v5
        pub fn from_generic_uuid(uuid: Uuid) -> Result<Self, Error> {
            if uuid.get_version_num() == 5 {
                Ok(Id(uuid, PhantomData::default()))
            } else {
                Err(Error::WrongVersion {
                    expected: 5,
                    actual: uuid.get_version_num(),
                })
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::V5;
        use crate::Id;
        use uuid::Uuid;

        #[test]
        fn new() {
            let _ = Id::<u32, V5>::new(&Uuid::NAMESPACE_DNS, &[0u8; 6]);
        }
    }
}

#[cfg(all(uuid_unstable, feature = "v6"))]
mod v6 {
    use crate::{Error, Id};
    use core::marker::PhantomData;
    use uuid::{Timestamp, Uuid};

    /// Denotes that the contained Uuid is of type V6
    #[derive(Debug)]
    pub struct V6;

    impl<T> Id<T, V6> {
        /// Construct a new typed v6 Uuid
        #[allow(clippy::new_without_default)]
        pub fn new(ts: Timestamp, node_id: &[u8; 6]) -> Self {
            Self(Uuid::new_v1(ts, node_id), PhantomData::default())
        }

        /// Attempt to coerce a generic [`Uuid`] into a typed [`Id`]
        ///
        /// Returns `Err(Error::WrongVersion)` if the generic Uuid version
        /// is not v6
        pub fn from_generic_uuid(uuid: Uuid) -> Result<Self, Error> {
            if uuid.get_version_num() == 6 {
                Ok(Id(uuid, PhantomData::default()))
            } else {
                Err(Error::WrongVersion {
                    expected: 6,
                    actual: uuid.get_version_num(),
                })
            }
        }
    }
}

#[cfg(all(uuid_unstable, feature = "v7"))]
mod v7 {
    use crate::{Error, Id};
    use core::marker::PhantomData;
    use uuid::{Timestamp, Uuid};

    /// Denotes that the contained Uuid is of type V7
    #[derive(Debug)]
    pub struct V7;

    impl<T> Id<T, V7> {
        /// Construct a new typed v7 Uuid
        #[allow(clippy::new_without_default)]
        pub fn new(ts: Timestamp) -> Self {
            Self(Uuid::new_v7(ts), PhantomData::default())
        }

        /// Attempt to coerce a generic [`Uuid`] into a typed [`Id`]
        ///
        /// Returns `Err(Error::WrongVersion)` if the generic Uuid version
        /// is not v7
        pub fn from_generic_uuid(uuid: Uuid) -> Result<Self, Error> {
            if uuid.get_version_num() == 7 {
                Ok(Id(uuid, PhantomData::default()))
            } else {
                Err(Error::WrongVersion {
                    expected: 7,
                    actual: uuid.get_version_num(),
                })
            }
        }
    }
}

#[cfg(all(uuid_unstable, feature = "v8"))]
mod v8 {
    use crate::{Error, Id, Uuid};
    use core::marker::PhantomData;

    /// Denotes that the contained Uuid is of type V8
    #[derive(Debug)]
    pub struct V8;

    impl<T> Id<T, V8> {
        /// Construct a new typed v8 Uuid
        #[allow(clippy::new_without_default)]
        pub fn new(buf: [u8; 16]) -> Self {
            Self(Uuid::new_v8(buf), PhantomData::default())
        }

        /// Attempt to coerce a generic [`Uuid`] into a typed [`Id`]
        ///
        /// Returns `Err(Error::WrongVersion)` if the generic Uuid version
        /// is not v8
        pub fn from_generic_uuid(uuid: Uuid) -> Result<Self, Error> {
            if uuid.get_version_num() == 8 {
                Ok(Id(uuid, PhantomData::default()))
            } else {
                Err(Error::WrongVersion {
                    expected: 8,
                    actual: uuid.get_version_num(),
                })
            }
        }
    }
}

/*
impl<T, Version: UntypedVersion> From<[u8; 16]> for Id<T, Version> {
    fn from(value: [u8; 16]) -> Self {
        Id::<T, Version>(Uuid::from_bytes(value), PhantomData::default())
    }
}
*/
