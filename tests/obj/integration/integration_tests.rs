use std::{
    fs::File,
    io::{BufReader, BufWriter, Read},
};

use wavefront_rs::obj::{parser::Parser, writer::Writer};

#[test]
fn test_teapot_smoke() {
    let file = File::open("./tests/obj/integration/resources/teapot.obj").unwrap();
    Parser::read_to_end(&mut BufReader::new(file), |_| {}).unwrap();
}

#[test]
fn test_lamp_smoke() {
    let file = File::open("./tests/obj/integration/resources/lamp.obj").unwrap();
    Parser::read_to_end(&mut BufReader::new(file), |_| {}).unwrap();
}

#[test]
fn test_teapot_read_write() {
    let source = std::fs::File::open("./tests/obj/integration/resources/teapot.obj").unwrap();
    let mut source_content = String::new();
    File::open("./tests/obj/integration/resources/teapot.obj")
        .unwrap()
        .read_to_string(&mut source_content)
        .unwrap();
    let mut dest = String::new();
    {
        let writer_arc = std::sync::Arc::new(std::sync::Mutex::new(BufWriter::new(unsafe { dest.as_mut_vec() })));
        Parser::read_to_end(&mut BufReader::new(source), |e| {
            let mut local_writer = writer_arc.lock().unwrap();
            Writer { auto_newline: true }.write(&mut *local_writer, &e).unwrap();
        })
        .unwrap();
    }
    assert_eq!(source_content, dest);
}

#[test]
fn test_lamp_read_write() {
    let source = std::fs::File::open("./tests/obj/integration/resources/lamp.obj").unwrap();
    let mut source_content = String::new();
    File::open("./tests/obj/integration/resources/lamp.obj")
        .unwrap()
        .read_to_string(&mut source_content)
        .unwrap();
    let mut dest = String::new();
    {
        let writer_arc = std::sync::Arc::new(std::sync::Mutex::new(BufWriter::new(unsafe { dest.as_mut_vec() })));
        Parser::read_to_end(&mut BufReader::new(source), |e| {
            let mut local_writer = writer_arc.lock().unwrap();
            Writer { auto_newline: true }.write(&mut *local_writer, &e).unwrap();
        })
        .unwrap();
    }
    assert_eq!(source_content, dest);
}
