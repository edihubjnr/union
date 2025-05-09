use macros::model;
use unionlabs_primitives::Bytes;

use crate::{
    cosmos::ics23::{hash_op::HashOp, length_op::LengthOp},
    errors::UnknownEnumVariant,
};

#[model(proto(raw(protos::cosmos::ics23::v1::LeafOp), into, from))]
#[cfg_attr(feature = "bincode", derive(bincode::Encode, bincode::Decode))]
pub struct LeafOp {
    pub hash: HashOp,
    pub prehash_key: HashOp,
    pub prehash_value: HashOp,
    pub length: LengthOp,
    pub prefix: Bytes,
}

impl From<LeafOp> for protos::cosmos::ics23::v1::LeafOp {
    fn from(value: LeafOp) -> Self {
        Self {
            hash: value.hash.into(),
            prehash_key: value.prehash_key.into(),
            prehash_value: value.prehash_value.into(),
            length: value.length.into(),
            prefix: value.prefix.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, thiserror::Error)]
pub enum TryFromLeafOpError {
    #[error("error decoding hash")]
    Hash(#[source] UnknownEnumVariant<i32>),
    #[error("error decoding prehash_key")]
    PrehashKey(#[source] UnknownEnumVariant<i32>),
    #[error("error decoding prehash_value")]
    PrehashValue(#[source] UnknownEnumVariant<i32>),
    #[error("error decoding length")]
    Length(#[source] UnknownEnumVariant<i32>),
}

impl TryFrom<protos::cosmos::ics23::v1::LeafOp> for LeafOp {
    type Error = TryFromLeafOpError;

    fn try_from(value: protos::cosmos::ics23::v1::LeafOp) -> Result<Self, Self::Error> {
        Ok(Self {
            hash: value.hash.try_into().map_err(TryFromLeafOpError::Hash)?,
            prehash_key: value
                .prehash_key
                .try_into()
                .map_err(TryFromLeafOpError::PrehashKey)?,
            prehash_value: value
                .prehash_value
                .try_into()
                .map_err(TryFromLeafOpError::PrehashValue)?,
            length: value
                .length
                .try_into()
                .map_err(TryFromLeafOpError::Length)?,
            prefix: value.prefix.into(),
        })
    }
}
