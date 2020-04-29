extern crate wavefront_rs;
use wavefront_rs::obj::string_lexer::*;
use wavefront_rs::obj::entity::*;
use float_cmp::*;

#[test]
fn test_read_line_comment() {
    let exists = std::cell::Cell::new(false);
    if let Ok(Entity::Comment{content}) = StringLexer::read_line("# token") {
        if content == "token" {
            exists.set(true);
        }
    }
    assert_eq!(true, exists.take());
}

#[test]
fn test_read_line_object() {
    let exists = std::cell::Cell::new(false);
    if let Ok(Entity::Object{name}) = StringLexer::read_line("o token") {
        if name == "token" {
            exists.set(true);
        }
    }
    assert_eq!(true, exists.take());
}

#[test]
fn test_read_line_group() {
    let exists = std::cell::Cell::new(false);
    if let Ok(Entity::Group{name}) = StringLexer::read_line("g token") {
        if name == "token" {
            exists.set(true);
        }
    }
    assert_eq!(true, exists.take());
}

#[test]
fn test_read_line_smoothing_group() {
    let exists = std::cell::Cell::new(false);
    if let Ok(Entity::SmoothingGroup{name}) = StringLexer::read_line("s token") {
        if name == "token" {
            exists.set(true);
        }
    }
    assert_eq!(true, exists.take());
}

#[test]
fn test_read_line_mtllib() {
    let exists = std::cell::Cell::new(false);
    if let Ok(Entity::Mtllib{name}) = StringLexer::read_line("mtllib token") {
        if name == "token" {
            exists.set(true);
        }
    }
    assert_eq!(true, exists.take());
}

#[test]
fn test_read_line_usemtl() {
    let exists = std::cell::Cell::new(false);
    if let Ok(Entity::Usemtl{name}) = StringLexer::read_line("usemtl token") {
        if name == "token" {
            exists.set(true);
        }
    }
    assert_eq!(true, exists.take());
}

#[test]
fn test_read_line_vertex_xyzw() {
    let exists = std::cell::Cell::new(false);
    if let Ok(Entity::Vertex{x, y, z, w}) = StringLexer::read_line("v 0.1 1.2 2.3 3.4") {
        assert!(approx_eq!(f64, 0.1, x, epsilon=1e-5));
        assert!(approx_eq!(f64, 1.2, y, epsilon=1e-5));
        assert!(approx_eq!(f64, 2.3, z, epsilon=1e-5));
        assert!(approx_eq!(f64, 3.4, w.unwrap(), epsilon=1e-5));
        exists.set(true);
    }
    assert_eq!(true, exists.take());
}

#[test]
fn test_read_line_vertex_xyz() {
    let exists = std::cell::Cell::new(false);
    if let Ok(Entity::Vertex{x, y, z, w}) = StringLexer::read_line("v 0.1 1.2 2.3") {
        assert!(approx_eq!(f64, 0.1, x, epsilon=1e-5));
        assert!(approx_eq!(f64, 1.2, y, epsilon=1e-5));
        assert!(approx_eq!(f64, 2.3, z, epsilon=1e-5));
        assert_eq!(None, w);
        exists.set(true);
    }
    assert_eq!(true, exists.take());
}

#[test]
fn test_read_line_vertex_normal() {
    let exists = std::cell::Cell::new(false);
    if let Ok(Entity::VertexNormal{x, y, z}) = StringLexer::read_line("vn 0.1 1.2 2.3") {
        assert!(approx_eq!(f64, 0.1, x, epsilon=1e-5));
        assert!(approx_eq!(f64, 1.2, y, epsilon=1e-5));
        assert!(approx_eq!(f64, 2.3, z, epsilon=1e-5));
        exists.set(true);
    }
    assert_eq!(true, exists.take());
}

#[test]
fn test_read_line_vertex_texture_xyz() {
    let exists = std::cell::Cell::new(false);
    if let Ok(Entity::VertexTexture{x, y, z}) = StringLexer::read_line("vt 0.1 1.2 2.3") {
        assert!(approx_eq!(f64, 0.1, x, epsilon=1e-5));
        assert!(approx_eq!(f64, 1.2, y, epsilon=1e-5));
        assert!(approx_eq!(f64, 2.3, z.unwrap(), epsilon=1e-5));
        exists.set(true);
    }
    assert_eq!(true, exists.take());
}

#[test]
fn test_read_line_vertex_texture_xy() {
    let exists = std::cell::Cell::new(false);
    if let Ok(Entity::VertexTexture{x, y, z}) = StringLexer::read_line("vt 0.1 1.2") {
        assert!(approx_eq!(f64, 0.1, x, epsilon=1e-5));
        assert!(approx_eq!(f64, 1.2, y, epsilon=1e-5));
        assert_eq!(None, z);
        exists.set(true);
    }
    assert_eq!(true, exists.take());
}

#[test]
fn test_read_line_face_vnt_3() {
    let exists = std::cell::Cell::new(false);
    if let Ok(Entity::Face{vertices}) = StringLexer::read_line("f 0/1/2 3/4/5 6/7/8") {
        assert_eq!(3, vertices.len());
        assert_eq!(FaceVertex::new2(0, Some(1), Some(2)), vertices[0]);
        assert_eq!(FaceVertex::new2(3, Some(4), Some(5)), vertices[1]);
        assert_eq!(FaceVertex::new2(6, Some(7), Some(8)), vertices[2]);
        exists.set(true);
    }
    assert_eq!(true, exists.take());
}

#[test]
fn test_read_line_face_vnt_6() {
    let exists = std::cell::Cell::new(false);
    if let Ok(Entity::Face{vertices}) = StringLexer::read_line("f 0/1/2 3/4/5 6/7/8 9/10/11 12/13/14") {
        assert_eq!(5, vertices.len());
        assert_eq!(FaceVertex::new2(0, Some(1), Some(2)), vertices[0]);
        assert_eq!(FaceVertex::new2(3, Some(4), Some(5)), vertices[1]);
        assert_eq!(FaceVertex::new2(6, Some(7), Some(8)), vertices[2]);
        assert_eq!(FaceVertex::new2(9, Some(10), Some(11)), vertices[3]);
        assert_eq!(FaceVertex::new2(12, Some(13), Some(14)), vertices[4]);
        exists.set(true);
    }
    assert_eq!(true, exists.take());
}

#[test]
fn test_read_line_face_vt() {
    let exists = std::cell::Cell::new(false);
    if let Ok(Entity::Face{vertices}) = StringLexer::read_line("f 0//2 3//5 6//8") {
        assert_eq!(3, vertices.len());
        assert_eq!(FaceVertex::new2(0, None, Some(2)), vertices[0]);
        assert_eq!(FaceVertex::new2(3, None, Some(5)), vertices[1]);
        assert_eq!(FaceVertex::new2(6, None, Some(8)), vertices[2]);
        exists.set(true);
    }
    assert_eq!(true, exists.take());
}

#[test]
fn test_read_line_face_vn() {
    let exists = std::cell::Cell::new(false);
    if let Ok(Entity::Face{vertices}) = StringLexer::read_line("f 0/1 3/4 6/7") {
        assert_eq!(3, vertices.len());
        assert_eq!(FaceVertex::new2(0, Some(1), None), vertices[0]);
        assert_eq!(FaceVertex::new2(3, Some(4), None), vertices[1]);
        assert_eq!(FaceVertex::new2(6, Some(7), None), vertices[2]);
        exists.set(true);
    }
    assert_eq!(true, exists.take());
}

#[test]
fn test_read_line_face_v() {
    let exists = std::cell::Cell::new(false);
    if let Ok(Entity::Face{vertices}) = StringLexer::read_line("f 0 3 6") {
        assert_eq!(3, vertices.len());
        assert_eq!(FaceVertex::new2(0, None, None), vertices[0]);
        assert_eq!(FaceVertex::new2(3, None, None), vertices[1]);
        assert_eq!(FaceVertex::new2(6, None, None), vertices[2]);
        exists.set(true);
    }
    assert_eq!(true, exists.take());
}

#[test]
fn test_read_line_line() {
    let exists = std::cell::Cell::new(false);
    if let Ok(Entity::Line{vertices}) = StringLexer::read_line("l 0 1 2 3 4") {
        assert_eq!(5, vertices.len());
        assert_eq!(0, vertices[0]);
        assert_eq!(1, vertices[1]);
        assert_eq!(2, vertices[2]);
        assert_eq!(3, vertices[3]);
        assert_eq!(4, vertices[4]);
        exists.set(true);
    }
    assert_eq!(true, exists.take());
}
