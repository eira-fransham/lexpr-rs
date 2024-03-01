//! Serialization to and from `lexpr::Value`.

pub use lexpr::{Cons, Value};

pub use de::{from_value, Deserializer};
// We need to re-export all the sub-serializers used for associated types in `Serializer`'s
// `serde::ser::Serializer` impl.
pub use ser::{
    to_value, SerializeList, SerializeMap, SerializeStruct, SerializeStructVariant,
    SerializeTupleVariant, SerializeVector, Serializer,
};

mod de;
mod ser;
