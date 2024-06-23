/// Contains low-level structs for directly executing Cloud Filter operations.
///
/// The [command][crate::command] API is exposed through various higher-level structs, like
/// [Request][crate::Request] and [Placeholder][crate::Placeholder]. Thus, it is not necessary to
/// create and call these structs manually unless you need more granular access.
pub mod command;
mod error;
/// Contains traits extending common structs from the [std][std].
pub mod ext;
pub mod filter;
pub mod metadata;
pub mod placeholder;
pub mod placeholder_file;
pub mod request;
pub mod root;
pub mod usn;
pub mod utility;

pub use error::CloudErrorKind;
pub use filter::{info, ticket, SyncFilter};
pub use placeholder::Placeholder;
pub use placeholder_file::{BatchCreate, PlaceholderFile};
pub use request::{Process, Request};
pub use root::{
    active_roots, is_supported, Connection, HydrationPolicy, HydrationType, PopulationType,
    ProtectionMode, Registration, SecurityId, Session, SupportedAttributes, SyncRootId,
    SyncRootIdBuilder,
};
pub use usn::Usn;
pub use utility::{ReadAt, WriteAt};

pub use nt_time;

mod sealed {
    pub trait Sealed {}
}
