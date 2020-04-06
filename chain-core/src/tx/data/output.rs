#[cfg(not(feature = "mesalock_sgx"))]
use std::fmt;
#[cfg(not(feature = "mesalock_sgx"))]
use std::str::FromStr;

use parity_scale_codec::{Decode, Encode, Error, Input, Output};
#[cfg(not(feature = "mesalock_sgx"))]
use serde::de;
#[cfg(not(feature = "mesalock_sgx"))]
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::common::Timespec;
use crate::init::coin::Coin;
use crate::tx::data::address::ExtendedAddr;

/// Tx Output composed of an address and a coin value
#[derive(Debug, PartialEq, Eq, Clone)]
#[cfg_attr(not(feature = "mesalock_sgx"), derive(Serialize, Deserialize))]
pub struct TxOut {
    #[cfg_attr(
        not(feature = "mesalock_sgx"),
        serde(serialize_with = "serialize_address")
    )]
    #[cfg_attr(
        not(feature = "mesalock_sgx"),
        serde(deserialize_with = "deserialize_address")
    )]
    pub address: ExtendedAddr,
    pub value: Coin,
    pub valid_from: Option<Timespec>,
}

impl Encode for TxOut {
    fn encode_to<EncOut: Output>(&self, dest: &mut EncOut) {
        dest.push(&self.address);
        dest.push(&self.value);
        dest.push(&self.valid_from);
    }

    fn size_hint(&self) -> usize {
        let v = self.valid_from.size_hint();
        4 + 33 + v
    }
}

impl Decode for TxOut {
    fn decode<DecIn: Input>(input: &mut DecIn) -> Result<Self, Error> {
        // note: ExtendedAddr is enum;
        // if there is some need for extending TxOut -- e.g. locking against
        // more complex conditions (more complete merkelized abstract syntax tree)
        // it can be done with new variants on ExtendedAddr
        // or for backwards compatibility, there could be an extra Option<...> field in TxOut,
        // but one needs to careful that "None" isn't required to be encoded
        let address = ExtendedAddr::decode(input)?;
        let value = Coin::decode(input)?;
        let valid_from: Option<Timespec> = Option::decode(input)?;
        Ok(TxOut {
            address,
            value,
            valid_from,
        })
    }
}

#[cfg(not(feature = "mesalock_sgx"))]
fn serialize_address<S>(
    address: &ExtendedAddr,
    serializer: S,
) -> std::result::Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&address.to_string())
}

#[cfg(not(feature = "mesalock_sgx"))]
fn deserialize_address<'de, D>(deserializer: D) -> std::result::Result<ExtendedAddr, D::Error>
where
    D: Deserializer<'de>,
{
    struct StrVisitor;

    impl<'de> de::Visitor<'de> for StrVisitor {
        type Value = ExtendedAddr;

        fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter.write_str("transfer address in bech32 format")
        }

        #[inline]
        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            ExtendedAddr::from_str(value).map_err(|err| de::Error::custom(err.to_string()))
        }
    }

    deserializer.deserialize_str(StrVisitor)
}

#[cfg(not(feature = "mesalock_sgx"))]
impl fmt::Display for TxOut {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} -> {}", self.address, self.value)
    }
}

impl TxOut {
    /// creates a TX output (mainly for testing/tools)
    pub fn new(address: ExtendedAddr, value: Coin) -> Self {
        TxOut {
            address,
            value,
            valid_from: None,
        }
    }

    /// creates a TX output with timelock
    pub fn new_with_timelock(address: ExtendedAddr, value: Coin, valid_from: Timespec) -> Self {
        TxOut {
            address,
            value,
            valid_from: Some(valid_from),
        }
    }
}
