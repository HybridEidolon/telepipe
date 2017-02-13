use std::io::Cursor;

use super::*;

/// Multi-byte integers should serialize as little endian.
#[test]
fn little_endian_integers() {
    let v = 1u16;
    let mut cursor = Cursor::new(Vec::new());
    v.serialize(&mut cursor).unwrap();
    let buf = cursor.into_inner();
    assert_eq!(&buf, &[1u8, 0]);

    let v = 1u32;
    let mut cursor = Cursor::new(Vec::new());
    v.serialize(&mut cursor).unwrap();
    let buf = cursor.into_inner();
    assert_eq!(&buf, &[1u8, 0, 0, 0]);

    let v = 1u64;
    let mut cursor = Cursor::new(Vec::new());
    v.serialize(&mut cursor).unwrap();
    let buf = cursor.into_inner();
    assert_eq!(&buf, &[1u8, 0, 0, 0, 0, 0, 0, 0]);

    let v = -2i16;
    let mut cursor = Cursor::new(Vec::new());
    v.serialize(&mut cursor).unwrap();
    let buf = cursor.into_inner();
    assert_eq!(&buf, &[254u8, 255]);

    let v = -2i32;
    let mut cursor = Cursor::new(Vec::new());
    v.serialize(&mut cursor).unwrap();
    let buf = cursor.into_inner();
    assert_eq!(&buf, &[254u8, 255, 255, 255]);

    let v = -2i64;
    let mut cursor = Cursor::new(Vec::new());
    v.serialize(&mut cursor).unwrap();
    let buf = cursor.into_inner();
    assert_eq!(&buf, &[254u8, 255, 255, 255, 255, 255, 255, 255]);
}
