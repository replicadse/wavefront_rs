extern crate wavefront_rs;
use std::io::{BufReader, Write};
use wavefront_rs::lexer::*;
use wavefront_rs::entity::*;
use float_cmp::*;
use std::rc::Rc;

#[allow(dead_code)]
fn test_lexer() {
    let lexer = ReadLexer::new();
    let stream = std::fs::File::open("/Users/heikoaweber/Desktop/buildings/building.obj");
    let now = std::time::Instant::now();
    let file = Rc::new(std::fs::File::create("/Users/heikoaweber/Desktop/output.obj").unwrap());
    lexer.read(&mut std::io::BufReader::new(stream.unwrap()),
        |x| {
            file.as_ref().write_all(x.to_string().as_ref()).unwrap();
        }).unwrap();
    let nowafter = std::time::Instant::now();
    println!("milliseconds: {}", nowafter.duration_since(now).as_millis());
}

#[test]
fn test_comment() {
    let stream = std::io::Cursor::new("# token");
    let lexer = ReadLexer::new();
    let exists = std::cell::Cell::new(false);
    lexer.read(&mut BufReader::new(stream), 
        |x| {
            if let Entity::Comment{content} = x {
                if content == "token" {
                    exists.set(true);
                }
            }
        }).unwrap();
    assert_eq!(true, exists.take());
}

#[test]
fn test_object() {
    let stream = std::io::Cursor::new("o token");
    let lexer = ReadLexer::new();
    let exists = std::cell::Cell::new(false);
    lexer.read(&mut BufReader::new(stream), 
        |x| {
            if let Entity::Object{name} = x {
                if name == "token" {
                    exists.set(true);
                }
            }
        }).unwrap();
    assert_eq!(true, exists.take());
}

#[test]
fn test_group() {
    let stream = std::io::Cursor::new("g token");
    let lexer = ReadLexer::new();
    let exists = std::cell::Cell::new(false);
    lexer.read(&mut BufReader::new(stream), 
        |x| {
            if let Entity::Group{name} = x {
                if name == "token" {
                    exists.set(true);
                }
            }
        }).unwrap();
    assert_eq!(true, exists.take());
}

#[test]
fn test_smoothing_group() {
    let stream = std::io::Cursor::new("s token");
    let lexer = ReadLexer::new();
    let exists = std::cell::Cell::new(false);
    lexer.read(&mut BufReader::new(stream), 
        |x| {
            if let Entity::SmoothingGroup{name} = x {
                if name == "token" {
                    exists.set(true);
                }
            }
        }).unwrap();
    assert_eq!(true, exists.take());
}

#[test]
fn test_mtllib() {
    let stream = std::io::Cursor::new("mtllib token");
    let lexer = ReadLexer::new();
    let exists = std::cell::Cell::new(false);
    lexer.read(&mut BufReader::new(stream), 
        |x| {
            if let Entity::Mtllib{name} = x {
                if name == "token" {
                    exists.set(true);
                }
            }
        }).unwrap();
    assert_eq!(true, exists.take());
}

#[test]
fn test_usemtl() {
    let stream = std::io::Cursor::new("usemtl token");
    let lexer = ReadLexer::new();
    let exists = std::cell::Cell::new(false);
    lexer.read(&mut BufReader::new(stream), 
        |x| {
            if let Entity::Usemtl{name} = x {
                if name == "token" {
                    exists.set(true);
                }
            }
        }).unwrap();
    assert_eq!(true, exists.take());
}

#[test]
fn test_vertex_xyzw() {
    let stream = std::io::Cursor::new("v 0.1 1.2 2.3 3.4");
    let lexer = ReadLexer::new();
    let exists = std::cell::Cell::new(false);
    lexer.read(&mut BufReader::new(stream), 
        |v| {
            if let Entity::Vertex{x, y, z, w} = v {
                assert!(approx_eq!(f64, 0.1, x, epsilon=1e-5));
                assert!(approx_eq!(f64, 1.2, y, epsilon=1e-5));
                assert!(approx_eq!(f64, 2.3, z, epsilon=1e-5));
                assert!(approx_eq!(f64, 3.4, w.unwrap(), epsilon=1e-5));
                exists.set(true);
            };
        }).unwrap();
    assert_eq!(true, exists.take());
}

#[test]
fn test_vertex_xyz() {
    let stream = std::io::Cursor::new("v 0.1 1.2 2.3");
    let lexer = ReadLexer::new();
    let exists = std::cell::Cell::new(false);
    lexer.read(&mut BufReader::new(stream), 
        |v| {
            if let Entity::Vertex{x, y, z, w} = v {
                assert!(approx_eq!(f64, 0.1, x, epsilon=1e-5));
                assert!(approx_eq!(f64, 1.2, y, epsilon=1e-5));
                assert!(approx_eq!(f64, 2.3, z, epsilon=1e-5));
                assert_eq!(None, w);
                exists.set(true);
            };
        }).unwrap();
    assert_eq!(true, exists.take());
}

#[test]
fn test_vertex_normal() {
    let stream = std::io::Cursor::new("vn 0.1 1.2 2.3");
    let lexer = ReadLexer::new();
    let exists = std::cell::Cell::new(false);
    lexer.read(&mut BufReader::new(stream), 
        |v| {
            if let Entity::VertexNormal{x, y, z} = v {
                assert!(approx_eq!(f64, 0.1, x, epsilon=1e-5));
                assert!(approx_eq!(f64, 1.2, y, epsilon=1e-5));
                assert!(approx_eq!(f64, 2.3, z, epsilon=1e-5));
                exists.set(true);
            };
        }).unwrap();
    assert_eq!(true, exists.take());
}

#[test]
fn test_vertex_texture_xyz() {
    let stream = std::io::Cursor::new("vt 0.1 1.2 2.3");
    let lexer = ReadLexer::new();
    let exists = std::cell::Cell::new(false);
    lexer.read(&mut BufReader::new(stream), 
        |v| {
            if let Entity::VertexTexture{x, y, z} = v {
                assert!(approx_eq!(f64, 0.1, x, epsilon=1e-5));
                assert!(approx_eq!(f64, 1.2, y, epsilon=1e-5));
                assert!(approx_eq!(f64, 2.3, z.unwrap(), epsilon=1e-5));
                exists.set(true);
            };
        }).unwrap();
    assert_eq!(true, exists.take());
}

#[test]
fn test_vertex_texture_xy() {
    let stream = std::io::Cursor::new("vt 0.1 1.2");
    let lexer = ReadLexer::new();
    let exists = std::cell::Cell::new(false);
    lexer.read(&mut BufReader::new(stream), 
        |v| {
            if let Entity::VertexTexture{x, y, z} = v {
                assert!(approx_eq!(f64, 0.1, x, epsilon=1e-5));
                assert!(approx_eq!(f64, 1.2, y, epsilon=1e-5));
                assert_eq!(None, z);
                exists.set(true);
            };
        }).unwrap();
    assert_eq!(true, exists.take());
}

#[test]
fn test_face_vnt_3() {
    let stream = std::io::Cursor::new("f 0/1/2 3/4/5 6/7/8");
    let lexer = ReadLexer::new();
    let exists = std::cell::Cell::new(false);
    lexer.read(&mut BufReader::new(stream), 
        |v| {
            if let Entity::Face{vertices} = v {
                assert_eq!(3, vertices.len());
                assert_eq!(FaceVertex::new2(0, Some(1), Some(2)), vertices[0]);
                assert_eq!(FaceVertex::new2(3, Some(4), Some(5)), vertices[1]);
                assert_eq!(FaceVertex::new2(6, Some(7), Some(8)), vertices[2]);
                exists.set(true);
            };
        }).unwrap();
    assert_eq!(true, exists.take());
}

#[test]
fn test_face_vnt_6() {
    let stream = std::io::Cursor::new("f 0/1/2 3/4/5 6/7/8 9/10/11 12/13/14");
    let lexer = ReadLexer::new();
    let exists = std::cell::Cell::new(false);
    lexer.read(&mut BufReader::new(stream), 
        |v| {
            if let Entity::Face{vertices} = v {
                assert_eq!(5, vertices.len());
                assert_eq!(FaceVertex::new2(0, Some(1), Some(2)), vertices[0]);
                assert_eq!(FaceVertex::new2(3, Some(4), Some(5)), vertices[1]);
                assert_eq!(FaceVertex::new2(6, Some(7), Some(8)), vertices[2]);
                assert_eq!(FaceVertex::new2(9, Some(10), Some(11)), vertices[3]);
                assert_eq!(FaceVertex::new2(12, Some(13), Some(14)), vertices[4]);
                exists.set(true);
            };
        }).unwrap();
    assert_eq!(true, exists.take());
}

#[test]
fn test_face_vt() {
    let stream = std::io::Cursor::new("f 0//2 3//5 6//8");
    let lexer = ReadLexer::new();
    let exists = std::cell::Cell::new(false);
    lexer.read(&mut BufReader::new(stream), 
        |v| {
            if let Entity::Face{vertices} = v {
                assert_eq!(3, vertices.len());
                assert_eq!(FaceVertex::new2(0, None, Some(2)), vertices[0]);
                assert_eq!(FaceVertex::new2(3, None, Some(5)), vertices[1]);
                assert_eq!(FaceVertex::new2(6, None, Some(8)), vertices[2]);
                exists.set(true);
            };
        }).unwrap();
    assert_eq!(true, exists.take());
}

#[test]
fn test_face_vn() {
    let stream = std::io::Cursor::new("f 0/1 3/4 6/7");
    let lexer = ReadLexer::new();
    let exists = std::cell::Cell::new(false);
    lexer.read(&mut BufReader::new(stream), 
        |v| {
            if let Entity::Face{vertices} = v {
                assert_eq!(3, vertices.len());
                assert_eq!(FaceVertex::new2(0, Some(1), None), vertices[0]);
                assert_eq!(FaceVertex::new2(3, Some(4), None), vertices[1]);
                assert_eq!(FaceVertex::new2(6, Some(7), None), vertices[2]);
                exists.set(true);
            };
        }).unwrap();
    assert_eq!(true, exists.take());
}

#[test]
fn test_face_v() {
    let stream = std::io::Cursor::new("f 0 3 6");
    let lexer = ReadLexer::new();
    let exists = std::cell::Cell::new(false);
    lexer.read(&mut BufReader::new(stream), 
        |v| {
            if let Entity::Face{vertices} = v {
                assert_eq!(3, vertices.len());
                assert_eq!(FaceVertex::new2(0, None, None), vertices[0]);
                assert_eq!(FaceVertex::new2(3, None, None), vertices[1]);
                assert_eq!(FaceVertex::new2(6, None, None), vertices[2]);
                exists.set(true);
            };
        }).unwrap();
    assert_eq!(true, exists.take());
}

#[test]
fn test_line() {
    let stream = std::io::Cursor::new("l 0 1 2 3 4");
    let lexer = ReadLexer::new();
    let exists = std::cell::Cell::new(false);
    lexer.read(&mut BufReader::new(stream), 
        |v| {
            if let Entity::Line{vertices} = v {
                assert_eq!(5, vertices.len());
                assert_eq!(0, vertices[0]);
                assert_eq!(1, vertices[1]);
                assert_eq!(2, vertices[2]);
                assert_eq!(3, vertices[3]);
                assert_eq!(4, vertices[4]);
                exists.set(true);
            };
        }).unwrap();
    assert_eq!(true, exists.take());
}
