//! # How to use serde with rust-protobuf
//!
//! rust-protobuf 3 no longer directly supports serde.
//!
//! Practically, serde is needed mostly to be able to serialize and deserialize JSON,
//! and **rust-protobuf supports JSON directly**, and more correctly according to
//! official protobuf to JSON mapping. For that reason,
//! native serde support was removed from rust-protobuf.
//!
//! This crate is an example how to inject serde annotations into generated code.
//!
//! Annotations are configured from `build.rs`.

use std::fmt::Formatter;
use std::marker::PhantomData;

use protobuf::EnumFull;
use protobuf::EnumOrUnknown;

include!(concat!(env!("OUT_DIR"), "/protos/mod.rs"));

fn serialize_enum_or_unknown<E: EnumFull, S: serde::Serializer>(
    e: &Option<EnumOrUnknown<E>>,
    s: S,
) -> Result<S::Ok, S::Error> {
    if let Some(e) = e {
        match e.enum_value() {
            Ok(v) => s.serialize_str(v.descriptor().name()),
            Err(v) => s.serialize_i32(v),
        }
    } else {
        s.serialize_unit()
    }
}

fn deserialize_enum_or_unknown<'de, E: EnumFull, D: serde::Deserializer<'de>>(
    d: D,
) -> Result<Option<EnumOrUnknown<E>>, D::Error> {
    struct DeserializeEnumVisitor<E: EnumFull>(PhantomData<E>);

    impl<'de, E: EnumFull> serde::de::Visitor<'de> for DeserializeEnumVisitor<E> {
        type Value = Option<EnumOrUnknown<E>>;

        fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
            write!(formatter, "a string, an integer or none")
        }

        fn visit_str<R>(self, v: &str) -> Result<Self::Value, R>
        where
            R: serde::de::Error,
        {
            match E::enum_descriptor_static().value_by_name(v) {
                Some(v) => Ok(Some(EnumOrUnknown::from_i32(v.value()))),
                None => Err(serde::de::Error::custom(format!(
                    "unknown enum value: {}",
                    v
                ))),
            }
        }

        fn visit_i32<R>(self, v: i32) -> Result<Self::Value, R>
        where
            R: serde::de::Error,
        {
            Ok(Some(EnumOrUnknown::from_i32(v)))
        }

        fn visit_unit<R>(self) -> Result<Self::Value, R>
        where
            R: serde::de::Error,
        {
            Ok(None)
        }
    }

    d.deserialize_any(DeserializeEnumVisitor(PhantomData))
}

#[cfg(test)]
mod test {
    use crate::customize_example::Fruit;
    use crate::customize_example::Shape;

    #[test]
    fn test() {
        let mut fruit = Fruit::new();
        fruit.set_name("Orange".to_owned());
        fruit.set_weight(1.5);
        fruit.set_shape(Shape::CIRCLE);

        // Serde works.
        // Note rust-protobuf has built in support for JSON,
        // which follows protobuf-JSON serialization more correctly than default serde-json.
        // This example here is for the demonstration of generation of custom derives.
        let json = serde_json::to_string(&fruit).unwrap();
        assert_eq!(
            "{\"name\":\"Orange\",\"weight\":1.5,\"shape\":\"CIRCLE\"}",
            json
        );

        // TODO: add deserialization test
    }
}
