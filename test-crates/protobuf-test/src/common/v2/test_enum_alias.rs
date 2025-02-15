use protobuf::Enum;
use protobuf_test_common::*;

use super::test_enum_alias_pb::*;

#[test]
fn test_enum() {
    assert_eq!(10, EnumWithAlias::A.value());
    assert_eq!(10, EnumWithAlias::A_AGAIN.value());
    assert_eq!(
        &[
            EnumWithAlias::UNKNOWN,
            EnumWithAlias::A,
            EnumWithAlias::B,
            EnumWithAlias::A_AGAIN,
        ],
        EnumWithAlias::values()
    );
    assert_eq!(EnumWithAlias::A, EnumWithAlias::A_AGAIN);
}

#[test]
fn test_enum_in_message() {
    let mut m = TestEnumWithAlias::new();
    m.set_en(EnumWithAlias::A);
    test_serialize_deserialize_with_dynamic("08 0a", &m);
}
