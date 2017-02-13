use std::io::Cursor;
use serial::Serial;

use super::*;

#[test]
fn msg_serial_size() {
    let msg = Msg::Unknown(0, 0, vec![0; 20]);
    let mut cursor = Cursor::new(Vec::new());
    msg.serialize(&mut cursor).unwrap();
    let buf: Vec<u8> = cursor.into_inner();
    assert_eq!(buf.len(), 24);
}

#[test]
fn msg_serial_padding() {
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

#[test]
fn welcome_size() {
    let msg = Msg::LoginWelcome(0, Default::default());
    let mut cursor = Cursor::new(Vec::new());
    msg.serialize(&mut cursor).unwrap();
    let buf: Vec<u8> = cursor.into_inner();
    assert_eq!(buf.len(), 0x004C);
}

#[test]
fn welcome_content() {
    let msg = Msg::LoginWelcome(0, Default::default());
    let mut cursor = Cursor::new(Vec::new());
    msg.serialize(&mut cursor).unwrap();
    let buf: Vec<u8> = cursor.into_inner();
    assert_eq!(&buf[..4], &[0x17, 0x00, 0x4C, 0x00][..4]);
}

#[test]
fn redirect4_size() {
    let msg = Msg::Redirect4(Default::default());
    let mut cursor = Cursor::new(Vec::new());
    msg.serialize(&mut cursor).unwrap();
    let buf: Vec<u8> = cursor.into_inner();
    assert_eq!(buf.len(), 0x000C);
}

#[test]
fn redirect4_contents() {
    let msg = Msg::Redirect4(Default::default());
    let mut cursor = Cursor::new(Vec::new());
    msg.serialize(&mut cursor).unwrap();
    let buf: Vec<u8> = cursor.into_inner();
    assert_eq!(&buf[..],
               &[0x19, 0x00, 0x0C, 0x00, 127, 0, 0, 1, 80, 0, 0, 0]);
}

#[test]
fn redirect6_size() {
    let msg = Msg::Redirect6(Default::default());
    let mut cursor = Cursor::new(Vec::new());
    msg.serialize(&mut cursor).unwrap();
    let buf: Vec<u8> = cursor.into_inner();
    assert_eq!(buf.len(), 0x0018);
}

#[test]
fn hlcheck_size() {
    let msg = Msg::HlCheck(Default::default());
    let mut cursor = Cursor::new(Vec::new());
    msg.serialize(&mut cursor).unwrap();
    let buf: Vec<u8> = cursor.into_inner();
    assert_eq!(buf.len(), 224);
}
