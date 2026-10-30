// Copyright 2026 AsterSQL.

#[cfg(feature = "protobuf-codec")]
#[test]
fn s3_encryption_context_roundtrips_as_field_18() {
    use protobuf::Message;
    let context = "eyJrIjoidiJ9";
    let mut s3 = kvproto::brpb::S3::default();
    s3.set_sse_kms_encryption_context(context.to_owned());
    let bytes = s3.write_to_bytes().unwrap();
    let mut expected = vec![0x92, 0x01, context.len() as u8];
    expected.extend_from_slice(context.as_bytes());
    assert_eq!(bytes, expected);
    let decoded: kvproto::brpb::S3 = protobuf::parse_from_bytes(&bytes).unwrap();
    assert_eq!(decoded.get_sse_kms_encryption_context(), context);
    assert!(kvproto::brpb::S3::default()
        .write_to_bytes()
        .unwrap()
        .is_empty());
}

#[cfg(feature = "prost-codec")]
#[test]
fn s3_encryption_context_prost_roundtrips_as_field_18() {
    use prost::Message;
    let context = "eyJrIjoidiJ9";
    let s3 = kvproto::backup::S3 {
        sse_kms_encryption_context: context.to_owned(),
        ..Default::default()
    };
    let bytes = s3.encode_to_vec();
    let mut expected = vec![0x92, 0x01, context.len() as u8];
    expected.extend_from_slice(context.as_bytes());
    assert_eq!(bytes, expected);
    let decoded = kvproto::backup::S3::decode(bytes.as_slice()).unwrap();
    assert_eq!(decoded.sse_kms_encryption_context, context);
    assert!(kvproto::backup::S3::default().encode_to_vec().is_empty());
}
