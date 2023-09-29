use serde::de::MapAccess;

use crate::Error;

pub trait FromBool {
    fn from_bool(v: bool) -> Result<Self, Error>
    where
        Self: Sized;
}

pub trait FromF64 {
    fn from_f64(v: f64) -> Result<Self, Error>
    where
        Self: Sized;
}

pub trait FromI64 {
    fn from_i64(v: i64) -> Result<Self, Error>
    where
        Self: Sized;
}

pub trait FromNumber {
    fn from_f64(v: f64) -> Result<Self, Error>
    where
        Self: Sized;
    fn from_i64(v: i64) -> Result<Self, Error>
    where
        Self: Sized;
    fn from_u64(v: u64) -> Result<Self, Error>
    where
        Self: Sized;
}

pub trait FromU64 {
    fn from_u64(v: u64) -> Result<Self, Error>
    where
        Self: Sized;
}

pub trait FromMap {
    fn is_map_visitor() -> bool {
        true
    }
    fn from_map<'a, A>(map: A) -> Result<Self, A::Error>
    where
        A: MapAccess<'a>,
        Self: Sized;
}
