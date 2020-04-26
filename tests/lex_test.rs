extern crate wavefront_rs;
use std::io::Write;
use wavefront_rs::lexer::*;

#[allow(dead_code)]
fn test_lexer() {
    let lexer = ReadLexer::new();
    let stream = std::fs::read_to_string("/Users/heikoaweber/Desktop/buildings/building.obj");
    let now = std::time::Instant::now();
    let file = std::rc::Rc::new(std::fs::File::create("/Users/heikoaweber/Desktop/output.obj").unwrap());
    lexer.read(&mut std::io::BufReader::new(stream.unwrap().as_bytes()), 
        |x| {
            file.as_ref().write_all(x.to_string().as_ref()).unwrap();
        }).unwrap();
    let nowafter = std::time::Instant::now();
    println!("milliseconds: {}", nowafter.duration_since(now).as_millis());
}
