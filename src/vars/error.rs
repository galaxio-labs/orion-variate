use derive_more::From;
use orion_error::{OrionError, StructError, UnifiedReason};
use serde_derive::Serialize;

#[derive(Clone, Debug, Serialize, PartialEq, From, OrionError)]
pub enum VarsReason {
    #[orion_error(identity = "biz.unknow", code = 502)]
    UnKnow,
    #[orion_error(identity = "biz.format", code = 501)]
    Format,
    #[orion_error(transparent)]
    General(UnifiedReason),
}

pub type VarsResult<T> = Result<T, StructError<VarsReason>>;

impl VarsReason {
    /// Convert a raw std error (that doesn't implement `RawStdError`) into
    /// a `StructError<VarsReason>`, using its Display output as detail.
    pub fn raw_source_err(e: impl std::fmt::Display) -> StructError<VarsReason> {
        StructError::from(VarsReason::Format).with_detail(e.to_string())
    }
}
