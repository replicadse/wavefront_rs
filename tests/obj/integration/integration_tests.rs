use std::io::{BufReader, BufWriter, Read, Write};
use std::fs::File;

extern crate wavefront_rs;
use wavefront_rs::obj::read_lexer::ReadLexer;
use wavefront_rs::obj::format_writer::FormatWriter;

#[test]
fn test_teapot_smoke() {
    let file = File::open("./tests/obj/integration/resources/teapot.obj").unwrap();
    ReadLexer::read_to_end(&mut BufReader::new(file), |_| {}).unwrap();
}

#[test]
fn test_lamp_smoke() {
    let file = File::open("./tests/obj/integration/resources/lamp.obj").unwrap();
    ReadLexer::read_to_end(&mut BufReader::new(file), |_| {}).unwrap();
}

#[test]
fn test_teapot_read_write() {
    let source = std::fs::File::open("./tests/obj/integration/resources/teapot.obj").unwrap();
    let mut source_content = String::new();
    File::open("./tests/obj/integration/resources/teapot.obj").unwrap()
        .read_to_string(&mut source_content).unwrap();
    let mut dest = String::new();
    {
        let writer_arc = std::sync::Arc::new(
            std::sync::Mutex::new(
                BufWriter::new(
                    unsafe { dest.as_mut_vec() })));
        ReadLexer::read_to_end(&mut BufReader::new(source), |e| {
            let mut local_writer = writer_arc.lock().unwrap();
            FormatWriter::write(&mut *local_writer, &e);
            local_writer.write_all(b"\n").unwrap();
        }).unwrap();
    }
    assert_eq!(source_content, dest);
}

#[test]
fn test_lamp_read_write() {
    let source = std::fs::File::open("./tests/obj/integration/resources/lamp.obj").unwrap();
    let mut source_content = String::new();
    File::open("./tests/obj/integration/resources/lamp.obj").unwrap()
        .read_to_string(&mut source_content).unwrap();
    let mut dest = String::new();
    {
        let writer_arc = std::sync::Arc::new(
            std::sync::Mutex::new(
                BufWriter::new(
                    unsafe { dest.as_mut_vec() })));
        ReadLexer::read_to_end(&mut BufReader::new(source), |e| {
            let mut local_writer = writer_arc.lock().unwrap();
            FormatWriter::write(&mut *local_writer, &e);
            local_writer.write_all(b"\n").unwrap();
        }).unwrap();
    }
    assert_eq!(source_content, dest);
}
