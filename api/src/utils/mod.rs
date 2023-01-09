use chrono::NaiveDateTime;
use serde::Serializer;

pub mod env;
pub mod argon2;
pub mod jwt;

pub fn vec_to_option_vec<T>(vec: Vec<T>) -> Vec<Option<T>> {
    let mut new_vec = Vec::with_capacity(vec.len());

    for item in vec {
        new_vec.push(Some(item));
    }
    new_vec
}

pub fn serialize_timestamp<S>(datetime: &NaiveDateTime, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer
{
    serializer.serialize_i64(datetime.timestamp())
}
