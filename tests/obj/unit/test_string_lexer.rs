extern crate wavefront_rs;
use wavefront_rs::obj::entity::*;
use wavefront_rs::obj::string_lexer::*;

#[test]
fn test_read_line_comment() {
    let exists = std::cell::Cell::new(false);
    if let Ok(Entity::Comment { content }) = StringLexer::read_line("# token") {
        if content == "token" {
            exists.set(true);
        }
    }
    assert_eq!(true, exists.take());
}

#[test]
fn test_read_line_object() {
    let exists = std::cell::Cell::new(false);
    if let Ok(Entity::Object { name }) = StringLexer::read_line("o token") {
        if name == "token" {
            exists.set(true);
        }
    }
    assert_eq!(true, exists.take());
}

#[test]
fn test_read_line_group() {
    let exists = std::cell::Cell::new(false);
    if let Ok(Entity::Group { name }) = StringLexer::read_line("g token") {
        if name == "token" {
            exists.set(true);
        }
    }
    assert_eq!(true, exists.take());
}

#[test]
fn test_read_line_smoothing_group() {
    let exists = std::cell::Cell::new(false);
    if let Ok(Entity::SmoothingGroup { name }) = StringLexer::read_line("s token") {
        if name == "token" {
            exists.set(true);
        }
    }
    assert_eq!(true, exists.take());
}

#[test]
fn test_read_line_merging_group() {
    let exists = std::cell::Cell::new(false);
    if let Ok(Entity::MergingGroup { name }) = StringLexer::read_line("mg token") {
        if name == "token" {
            exists.set(true);
        }
    }
    assert_eq!(true, exists.take());
}

#[test]
fn test_read_line_mtllib() {
    let exists = std::cell::Cell::new(false);
    if let Ok(Entity::Mtllib { name }) = StringLexer::read_line("mtllib token") {
        if name == "token" {
            exists.set(true);
        }
    }
    assert_eq!(true, exists.take());
}

#[test]
fn test_read_line_usemtl() {
    let exists = std::cell::Cell::new(false);
    if let Ok(Entity::Usemtl { name }) = StringLexer::read_line("usemtl token") {
        if name == "token" {
            exists.set(true);
        }
    }
    assert_eq!(true, exists.take());
}

#[test]
fn test_read_line_vertex_xyzw() {
    assert_eq!(
        Entity::Vertex {
            x: 0.1f64,
            y: 1.2f64,
            z: 2.3f64,
            w: Some(3.4f64),
        },
        StringLexer::read_line("v 0.1 1.2 2.3 3.4").unwrap()
    );
}

#[test]
fn test_read_line_vertex_xyz() {
    assert_eq!(
        Entity::Vertex {
            x: 0.1f64,
            y: 1.2f64,
            z: 2.3f64,
            w: None,
        },
        StringLexer::read_line("v 0.1 1.2 2.3").unwrap()
    );
}

#[test]
fn test_read_line_vertex_normal() {
    assert_eq!(
        Entity::VertexNormal {
            x: 0.1f64,
            y: 1.2f64,
            z: 2.3f64,
        },
        StringLexer::read_line("vn 0.1 1.2 2.3").unwrap()
    );
}

#[test]
fn test_read_line_vertex_texture_uvw() {
    assert_eq!(
        Entity::VertexTexture {
            u: 0.1f64,
            v: Some(1.2f64),
            w: Some(2.3f64),
        },
        StringLexer::read_line("vt 0.1 1.2 2.3").unwrap()
    );
}

#[test]
fn test_read_line_vertex_texture_uv() {
    assert_eq!(
        Entity::VertexTexture {
            u: 0.1f64,
            v: Some(1.2f64),
            w: None,
        },
        StringLexer::read_line("vt 0.1 1.2").unwrap()
    );
}

#[test]
fn test_read_line_vertex_texture_u() {
    assert_eq!(
        Entity::VertexTexture {
            u: 0.1f64,
            v: None,
            w: None
        },
        StringLexer::read_line("vt 0.1").unwrap()
    );
}

#[test]
fn test_read_line_vertex_parameter_uvw() {
    assert_eq!(
        Entity::VertexParameter {
            u: 0.1f64,
            v: Some(1.2f64),
            w: Some(2.3f64),
        },
        StringLexer::read_line("vp 0.1 1.2 2.3").unwrap()
    );
}

#[test]
fn test_read_line_vertex_parameter_uv() {
    assert_eq!(
        Entity::VertexParameter {
            u: 0.1f64,
            v: Some(1.2f64),
            w: None,
        },
        StringLexer::read_line("vp 0.1 1.2").unwrap()
    );
}

#[test]
fn test_read_line_vertex_parameter_u() {
    assert_eq!(
        Entity::VertexParameter {
            u: 0.1f64,
            v: None,
            w: None
        },
        StringLexer::read_line("vp 0.1").unwrap()
    );
}

#[test]
fn test_read_line_face_vnt_3() {
    assert_eq!(
        Entity::Face {
            vertices: vec!(
                FaceVertex::new2(0, Some(1), Some(2)),
                FaceVertex::new2(3, Some(4), Some(5)),
                FaceVertex::new2(6, Some(7), Some(8)),
            )
        },
        StringLexer::read_line("f 0/1/2 3/4/5 6/7/8").unwrap()
    );
}

#[test]
fn test_read_line_face_vnt_5() {
    assert_eq!(
        Entity::Face {
            vertices: vec!(
                FaceVertex::new2(0, Some(1), Some(2)),
                FaceVertex::new2(3, Some(4), Some(5)),
                FaceVertex::new2(6, Some(7), Some(8)),
                FaceVertex::new2(9, Some(10), Some(11)),
                FaceVertex::new2(12, Some(13), Some(14)),
            )
        },
        StringLexer::read_line("f 0/1/2 3/4/5 6/7/8 9/10/11 12/13/14").unwrap()
    );
}

#[test]
fn test_read_line_face_vt() {
    assert_eq!(
        Entity::Face {
            vertices: vec!(
                FaceVertex::new2(0, None, Some(2)),
                FaceVertex::new2(3, None, Some(5)),
                FaceVertex::new2(6, None, Some(8)),
            )
        },
        StringLexer::read_line("f 0//2 3//5 6//8").unwrap()
    );
}

#[test]
fn test_read_line_face_vn() {
    assert_eq!(
        Entity::Face {
            vertices: vec!(
                FaceVertex::new2(0, Some(1), None),
                FaceVertex::new2(3, Some(4), None),
                FaceVertex::new2(6, Some(7), None),
            )
        },
        StringLexer::read_line("f 0/1 3/4 6/7").unwrap()
    );
}

#[test]
fn test_read_line_face_v() {
    assert_eq!(
        Entity::Face {
            vertices: vec!(
                FaceVertex::new2(0, None, None),
                FaceVertex::new2(3, None, None),
                FaceVertex::new2(6, None, None),
            )
        },
        StringLexer::read_line("f 0 3 6").unwrap()
    );
}

#[test]
fn test_read_line_line() {
    assert_eq!(
        Entity::Line {
            vertices: vec!(0, 1, 2, 3, 4)
        },
        StringLexer::read_line("l 0 1 2 3 4").unwrap()
    );
}
