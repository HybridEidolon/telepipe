use std::io::Cursor;
use serial::Serial;

use super::*;

#[test]
fn test_msg_serial_size() {
    let msg = Msg::Unknown(0, 0, vec![0; 20]);
    let mut cursor = Cursor::new(Vec::new());
    msg.serialize(&mut cursor).unwrap();
    let buf: Vec<u8> = cursor.into_inner();
    assert_eq!(buf.len(), 24);
}

#[test]
fn test_msg_serial_padding() {
    let msg = Msg::Unknown(0, 0, vec![0; 21]);
    let mut cursor = Cursor::new(Vec::new());
    msg.serialize(&mut cursor).unwrap();
    let buf: Vec<u8> = cursor.into_inner();
    assert_eq!(buf.len(), 28);

    let msg = Msg::Unknown(0, 0, vec![0; 22]);
    let mut cursor = Cursor::new(Vec::new());
    msg.serialize(&mut cursor).unwrap();
    let buf: Vec<u8> = cursor.into_inner();
    assert_eq!(buf.len(), 28);

    let msg = Msg::Unknown(0, 0, vec![0; 23]);
    let mut cursor = Cursor::new(Vec::new());
    msg.serialize(&mut cursor).unwrap();
    let buf: Vec<u8> = cursor.into_inner();
    assert_eq!(buf.len(), 28);

    let msg = Msg::Unknown(0, 0, vec![0; 24]);
    let mut cursor = Cursor::new(Vec::new());
    msg.serialize(&mut cursor).unwrap();
    let buf: Vec<u8> = cursor.into_inner();
    assert_eq!(buf.len(), 28);
}
