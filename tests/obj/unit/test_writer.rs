extern crate wavefront_rs;
use std::io::BufWriter;
use wavefront_rs::obj::entity::*;
use wavefront_rs::obj::format_writer::FormatWriter;

#[test]
fn test_write_comment() {
    let entity = Entity::Comment {
        content: "token".to_owned(),
    };
    let mut result = String::new();
    FormatWriter::write(&mut BufWriter::new(unsafe { result.as_mut_vec() }), &entity);
    assert_eq!("# token", result);
}

#[test]
fn test_write_object() {
    let entity = Entity::Object {
        name: "token".to_owned(),
    };
    let mut result = String::new();
    FormatWriter::write(&mut BufWriter::new(unsafe { result.as_mut_vec() }), &entity);
    assert_eq!("o token", result);
}

#[test]
fn test_write_group() {
    let entity = Entity::Group {
        name: "token".to_owned(),
    };
    let mut result = String::new();
    FormatWriter::write(&mut BufWriter::new(unsafe { result.as_mut_vec() }), &entity);
    assert_eq!("g token", result);
}

#[test]
fn test_write_smoothing_group() {
    let entity = Entity::SmoothingGroup {
        name: "token".to_owned(),
    };
    let mut result = String::new();
    FormatWriter::write(&mut BufWriter::new(unsafe { result.as_mut_vec() }), &entity);
    assert_eq!("s token", result);
}

#[test]
fn test_write_merging_group() {
    let entity = Entity::MergingGroup {
        name: "token".to_owned(),
    };
    let mut result = String::new();
    FormatWriter::write(&mut BufWriter::new(unsafe { result.as_mut_vec() }), &entity);
    assert_eq!("mg token", result);
}

#[test]
fn test_write_mtllib() {
    let entity = Entity::Mtllib {
        name: "token".to_owned(),
    };
    let mut result = String::new();
    FormatWriter::write(&mut BufWriter::new(unsafe { result.as_mut_vec() }), &entity);
    assert_eq!("mtllib token", result);
}

#[test]
fn test_write_usemtl() {
    let entity = Entity::Usemtl {
        name: "token".to_owned(),
    };
    let mut result = String::new();
    FormatWriter::write(&mut BufWriter::new(unsafe { result.as_mut_vec() }), &entity);
    assert_eq!("usemtl token", result);
}

#[test]
fn test_write_vertex_xyzw() {
    let entity = Entity::Vertex {
        x: 0f64,
        y: 1f64,
        z: 2f64,
        w: Some(3f64),
    };
    let mut result = String::new();
    FormatWriter::write(&mut BufWriter::new(unsafe { result.as_mut_vec() }), &entity);
    assert_eq!("v 0 1 2 3", result);
}

#[test]
fn test_write_vertex_xyz() {
    let entity = Entity::Vertex {
        x: 0f64,
        y: 1f64,
        z: 2f64,
        w: None,
    };
    let mut result = String::new();
    FormatWriter::write(&mut BufWriter::new(unsafe { result.as_mut_vec() }), &entity);
    assert_eq!("v 0 1 2", result);
}

#[test]
fn test_write_vertex_normal() {
    let entity = Entity::VertexNormal {
        x: 0f64,
        y: 1f64,
        z: 2f64,
    };
    let mut result = String::new();
    FormatWriter::write(&mut BufWriter::new(unsafe { result.as_mut_vec() }), &entity);
    assert_eq!("vn 0 1 2", result);
}

#[test]
fn test_write_vertex_texture_u() {
    let entity = Entity::VertexTexture {
        u: 0.1f64,
        v: None,
        w: None,
    };
    let mut result = String::new();
    FormatWriter::write(&mut BufWriter::new(unsafe { result.as_mut_vec() }), &entity);
    assert_eq!("vt 0.1", result);
}

#[test]
fn test_write_vertex_texture_uv() {
    let entity = Entity::VertexTexture {
        u: 0.1f64,
        v: Some(1.2f64),
        w: None,
    };
    let mut result = String::new();
    FormatWriter::write(&mut BufWriter::new(unsafe { result.as_mut_vec() }), &entity);
    assert_eq!("vt 0.1 1.2", result);
}

#[test]
fn test_write_vertex_texture_uvw() {
    let entity = Entity::VertexTexture {
        u: 0.1f64,
        v: Some(1.2f64),
        w: Some(2.3f64),
    };
    let mut result = String::new();
    FormatWriter::write(&mut BufWriter::new(unsafe { result.as_mut_vec() }), &entity);
    assert_eq!("vt 0.1 1.2 2.3", result);
}

#[test]
fn test_write_vertex_parameter_uvw() {
    let entity = Entity::VertexParameter {
        u: 0.1f64,
        v: Some(1.2f64),
        w: Some(2.3f64),
    };
    let mut result = String::new();
    FormatWriter::write(&mut BufWriter::new(unsafe { result.as_mut_vec() }), &entity);
    assert_eq!("vp 0.1 1.2 2.3", result);
}

#[test]
fn test_write_vertex_parameter_uv() {
    let entity = Entity::VertexParameter {
        u: 0.1f64,
        v: Some(1.2f64),
        w: None,
    };
    let mut result = String::new();
    FormatWriter::write(&mut BufWriter::new(unsafe { result.as_mut_vec() }), &entity);
    assert_eq!("vp 0.1 1.2", result);
}

#[test]
fn test_write_vertex_parameter_u() {
    let entity = Entity::VertexParameter {
        u: 0.1f64,
        v: None,
        w: None,
    };
    let mut result = String::new();
    FormatWriter::write(&mut BufWriter::new(unsafe { result.as_mut_vec() }), &entity);
    assert_eq!("vp 0.1", result);
}

#[test]
fn test_write_face_vnt_3() {
    let entity = Entity::Face {
        vertices: vec![
            FaceVertex::new2(0, Some(1), Some(2)),
            FaceVertex::new2(3, Some(4), Some(5)),
            FaceVertex::new2(6, Some(7), Some(8)),
        ],
    };
    let mut result = String::new();
    FormatWriter::write(&mut BufWriter::new(unsafe { result.as_mut_vec() }), &entity);
    assert_eq!("f 0/1/2 3/4/5 6/7/8", result);
}

#[test]
fn test_write_face_vnt_6() {
    let entity = Entity::Face {
        vertices: vec![
            FaceVertex::new2(0, Some(1), Some(2)),
            FaceVertex::new2(3, Some(4), Some(5)),
            FaceVertex::new2(6, Some(7), Some(8)),
            FaceVertex::new2(9, Some(10), Some(11)),
            FaceVertex::new2(12, Some(13), Some(14)),
            FaceVertex::new2(15, Some(16), Some(17)),
        ],
    };
    let mut result = String::new();
    FormatWriter::write(&mut BufWriter::new(unsafe { result.as_mut_vec() }), &entity);
    assert_eq!("f 0/1/2 3/4/5 6/7/8 9/10/11 12/13/14 15/16/17", result);
}

#[test]
fn test_write_face_vt() {
    let entity = Entity::Face {
        vertices: vec![
            FaceVertex::new2(0, None, Some(2)),
            FaceVertex::new2(3, None, Some(5)),
            FaceVertex::new2(6, None, Some(8)),
        ],
    };
    let mut result = String::new();
    FormatWriter::write(&mut BufWriter::new(unsafe { result.as_mut_vec() }), &entity);
    assert_eq!("f 0//2 3//5 6//8", result);
}

#[test]
fn test_write_face_vn() {
    let entity = Entity::Face {
        vertices: vec![
            FaceVertex::new2(0, Some(1), None),
            FaceVertex::new2(3, Some(4), None),
            FaceVertex::new2(6, Some(7), None),
        ],
    };
    let mut result = String::new();
    FormatWriter::write(&mut BufWriter::new(unsafe { result.as_mut_vec() }), &entity);
    assert_eq!("f 0/1 3/4 6/7", result);
}

#[test]
fn test_write_face_v() {
    let entity = Entity::Face {
        vertices: vec![
            FaceVertex::new2(0, None, None),
            FaceVertex::new2(3, None, None),
            FaceVertex::new2(6, None, None),
        ],
    };
    let mut result = String::new();
    FormatWriter::write(&mut BufWriter::new(unsafe { result.as_mut_vec() }), &entity);
    assert_eq!("f 0 3 6", result);
}

#[test]
fn test_write_line() {
    let entity = Entity::Line {
        vertices: vec![0, 1, 2, 3, 4],
    };
    let mut result = String::new();
    FormatWriter::write(&mut BufWriter::new(unsafe { result.as_mut_vec() }), &entity);
    assert_eq!("l 0 1 2 3 4", result);
}
