#![no_std]

use core::marker::PhantomData;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
pub use uuid::{Timestamp, Uuid};

#[derive(Debug, Clone, Copy)]
pub enum Error {
    WrongVersion { expected: usize, actual: usize },
}

/// [`Id`] is a typed wrapper around a [`Uuid`].
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Id<T, Version>(
    Uuid,
    #[cfg_attr(feature = "serde", serde(skip))] PhantomData<(T, Version)>,
);

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


#[cfg(feature = "v1")]
mod v1 {
    use crate::{Error, Id, Timestamp, Uuid};
    use core::marker::PhantomData;
    struct V1;

    impl<T> Id<T, V1> {
        #[allow(clippy::new_without_default)]
        pub fn new(ts: Timestamp, node_id: &[u8; 6]) -> Self {
            Self(Uuid::new_v1(ts, node_id), PhantomData::default())
        }

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
}

#[cfg(feature = "v3")]
mod v3 {
    use crate::{Error, Id, Uuid};
    use core::marker::PhantomData;
    struct V3;

    impl<T> Id<T, V3> {
        #[allow(clippy::new_without_default)]
        pub fn new(namespace: &Uuid, name: &[u8]) -> Self {
            Self(Uuid::new_v3(namespace, name), PhantomData::default())
        }

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
}

#[cfg(feature = "v4")]
mod v4 {
    use crate::{Error, Id, Uuid};
    use core::marker::PhantomData;
    struct V4;

    impl<T> Id<T, V4> {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self(Uuid::new_v4(), PhantomData::default())
        }

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
}

#[cfg(feature = "v5")]
mod v5 {
    use crate::{Error, Id, Uuid};
    use core::marker::PhantomData;
    struct V5;

    impl<T> Id<T, V5> {
        #[allow(clippy::new_without_default)]
        pub fn new(namespace: &Uuid, name: &[u8]) -> Self {
            Self(Uuid::new_v5(namespace, name), PhantomData::default())
        }

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
}

#[cfg(all(uuid_unstable, feature = "v6"))]
mod v6 {
    use crate::{Error, Id, Timestamp, Uuid};
    use core::marker::PhantomData;
    struct V6;

    impl<T> Id<T, V6> {
        #[allow(clippy::new_without_default)]
        pub fn new(ts: Timestamp, node_id: &[u8; 6]) -> Self {
            Self(Uuid::new_v1(ts, node_id), PhantomData::default())
        }

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
    use crate::{Error, Id, Timestamp, Uuid};
    use core::marker::PhantomData;
    struct V7;

    impl<T> Id<T, V7> {
        #[allow(clippy::new_without_default)]
        pub fn new(ts: Timestamp) -> Self {
            Self(Uuid::new_v7(ts), PhantomData::default())
        }

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
    struct V8;

    impl<T> Id<T, V8> {
        #[allow(clippy::new_without_default)]
        pub fn new(buf: [u8; 16]) -> Self {
            Self(Uuid::new_v8(buf), PhantomData::default())
        }

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
