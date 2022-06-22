use std::{
    fs::File,
    io::{
        BufReader,
        BufWriter,
        Read,
        Write,
    },
};

use wavefront_rs::mtl::{
    parser::Parser,
    writer::Writer,
};

#[test]
fn test_example_smoke() {
    let file = File::open("./tests/mtl/integration/resources/example.mtl").unwrap();
    Parser::read_to_end(&mut BufReader::new(file), |_| {}).unwrap();
}

#[test]
fn test_example_read_write() {
    let source = std::fs::File::open("./tests/mtl/integration/resources/example.mtl").unwrap();
    let mut source_content = String::new();
    File::open("./tests/mtl/integration/resources/example.mtl")
        .unwrap()
        .read_to_string(&mut source_content)
        .unwrap();
    let mut dest = String::new();
    {
        let writer_arc = std::sync::Arc::new(std::sync::Mutex::new(BufWriter::new(unsafe { dest.as_mut_vec() })));
        Parser::read_to_end(&mut BufReader::new(source), |e| {
            let mut local_writer = writer_arc.lock().unwrap();
            Writer::write(&mut *local_writer, &e).unwrap();
            local_writer.write_all(b"\n").unwrap();
        })
        .unwrap();
    }
    assert_eq!(source_content, dest);
}
