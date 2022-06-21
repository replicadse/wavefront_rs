extern crate wavefront_rs;
use std::io::BufReader;
use wavefront_rs::obj::entity::*;
use wavefront_rs::obj::parser::*;

#[test]
fn test_read_to_end_comment() {
    let stream = std::io::Cursor::new("# token");
    let exists = std::cell::Cell::new(false);
    Parser::read_to_end(&mut BufReader::new(stream), |x| {
        assert_eq!(
            Entity::Comment {
                content: "token".to_owned()
            },
            x
        );
        exists.set(true);
    })
    .unwrap();
    assert_eq!(true, exists.take());
}

#[test]
fn test_read_line_comment() {
    let stream = std::io::Cursor::new("# token");
    assert_eq!(
        Entity::Comment {
            content: "token".to_owned()
        },
        Parser::read_line(&mut BufReader::new(stream)).unwrap()
    );
}

#[test]
fn test_read_to_end_object() {
    let stream = std::io::Cursor::new("o token");
    let exists = std::cell::Cell::new(false);
    Parser::read_to_end(&mut BufReader::new(stream), |x| {
        assert_eq!(
            Entity::Object {
                name: "token".to_owned()
            },
            x
        );
        exists.set(true);
    })
    .unwrap();
    assert_eq!(true, exists.take());
}

#[test]
fn test_read_line_object() {
    let stream = std::io::Cursor::new("o token");
    assert_eq!(
        Entity::Object {
            name: "token".to_owned()
        },
        Parser::read_line(&mut BufReader::new(stream)).unwrap()
    );
}

#[test]
fn test_read_to_end_group() {
    let stream = std::io::Cursor::new("g token");
    let exists = std::cell::Cell::new(false);
    Parser::read_to_end(&mut BufReader::new(stream), |x| {
        assert_eq!(
            Entity::Group {
                name: "token".to_owned()
            },
            x
        );
        exists.set(true);
    })
    .unwrap();
    assert_eq!(true, exists.take());
}

#[test]
fn test_read_line_group() {
    let stream = std::io::Cursor::new("g token");
    assert_eq!(
        Entity::Group {
            name: "token".to_owned()
        },
        Parser::read_line(&mut BufReader::new(stream)).unwrap()
    );
}

#[test]
fn test_read_to_end_smoothing_group() {
    let stream = std::io::Cursor::new("s token");
    let exists = std::cell::Cell::new(false);
    Parser::read_to_end(&mut BufReader::new(stream), |x| {
        assert_eq!(
            Entity::SmoothingGroup {
                name: "token".to_owned()
            },
            x
        );
        exists.set(true);
    })
    .unwrap();
    assert_eq!(true, exists.take());
}

#[test]
fn test_read_line_smoothing_group() {
    let stream = std::io::Cursor::new("s token");
    assert_eq!(
        Entity::SmoothingGroup {
            name: "token".to_owned()
        },
        Parser::read_line(&mut BufReader::new(stream)).unwrap()
    );
}

#[test]
fn test_read_to_end_merging_group() {
    let stream = std::io::Cursor::new("mg token");
    let exists = std::cell::Cell::new(false);
    Parser::read_to_end(&mut BufReader::new(stream), |x| {
        assert_eq!(
            Entity::MergingGroup {
                name: "token".to_owned()
            },
            x
        );
        exists.set(true);
    })
    .unwrap();
    assert_eq!(true, exists.take());
}

#[test]
fn test_read_line_merging_group() {
    let stream = std::io::Cursor::new("mg token");
    assert_eq!(
        Entity::MergingGroup {
            name: "token".to_owned()
        },
        Parser::read_line(&mut BufReader::new(stream)).unwrap()
    );
}

#[test]
fn test_read_to_end_mtllib() {
    let stream = std::io::Cursor::new("mtllib token");
    let exists = std::cell::Cell::new(false);
    Parser::read_to_end(&mut BufReader::new(stream), |x| {
        assert_eq!(
            Entity::Mtllib {
                name: "token".to_owned()
            },
            x
        );
        exists.set(true);
    })
    .unwrap();
    assert_eq!(true, exists.take());
}

#[test]
fn test_read_line_mtllib() {
    let stream = std::io::Cursor::new("mtllib token");
    assert_eq!(
        Entity::Mtllib {
            name: "token".to_owned()
        },
        Parser::read_line(&mut BufReader::new(stream)).unwrap()
    );
}

#[test]
fn test_read_to_end_usemtl() {
    let stream = std::io::Cursor::new("usemtl token");
    let exists = std::cell::Cell::new(false);
    Parser::read_to_end(&mut BufReader::new(stream), |x| {
        assert_eq!(
            Entity::Usemtl {
                name: "token".to_owned()
            },
            x
        );
        exists.set(true);
    })
    .unwrap();
    assert_eq!(true, exists.take());
}

#[test]
fn test_read_line_usemtl() {
    let stream = std::io::Cursor::new("usemtl token");
    assert_eq!(
        Entity::Usemtl {
            name: "token".to_owned()
        },
        Parser::read_line(&mut BufReader::new(stream)).unwrap()
    );
}

#[test]
fn test_read_to_end_vertex_xyzw() {
    let stream = std::io::Cursor::new("v 0.1 1.2 2.3 3.4");
    let exists = std::cell::Cell::new(false);
    Parser::read_to_end(&mut BufReader::new(stream), |x| {
        assert_eq!(
            Entity::Vertex {
                x: 0.1f64,
                y: 1.2f64,
                z: 2.3f64,
                w: Some(3.4f64),
            },
            x
        );
        exists.set(true);
    })
    .unwrap();
    assert_eq!(true, exists.take());
}

#[test]
fn test_read_line_vertex_xyzw() {
    let stream = std::io::Cursor::new("v 0.1 1.2 2.3 3.4");
    assert_eq!(
        Entity::Vertex {
            x: 0.1f64,
            y: 1.2f64,
            z: 2.3f64,
            w: Some(3.4f64),
        },
        Parser::read_line(&mut BufReader::new(stream)).unwrap()
    );
}

#[test]
fn test_read_to_end_vertex_xyz() {
    let stream = std::io::Cursor::new("v 0.1 1.2 2.3");
    let exists = std::cell::Cell::new(false);
    Parser::read_to_end(&mut BufReader::new(stream), |x| {
        assert_eq!(
            Entity::Vertex {
                x: 0.1f64,
                y: 1.2f64,
                z: 2.3f64,
                w: None,
            },
            x
        );
        exists.set(true);
    })
    .unwrap();
    assert_eq!(true, exists.take());
}

#[test]
fn test_read_line_vertex_xyz() {
    let stream = std::io::Cursor::new("v 0.1 1.2 2.3");
    assert_eq!(
        Entity::Vertex {
            x: 0.1f64,
            y: 1.2f64,
            z: 2.3f64,
            w: None,
        },
        Parser::read_line(&mut BufReader::new(stream)).unwrap()
    );
}

#[test]
fn test_read_to_end_vertex_normal() {
    let stream = std::io::Cursor::new("vn 0.1 1.2 2.3");
    let exists = std::cell::Cell::new(false);
    Parser::read_to_end(&mut BufReader::new(stream), |x| {
        assert_eq!(
            Entity::VertexNormal {
                x: 0.1f64,
                y: 1.2f64,
                z: 2.3f64,
            },
            x
        );
        exists.set(true);
    })
    .unwrap();
    assert_eq!(true, exists.take());
}

#[test]
fn test_read_line_vertex_normal() {
    let stream = std::io::Cursor::new("vn 0.1 1.2 2.3");
    assert_eq!(
        Entity::VertexNormal {
            x: 0.1f64,
            y: 1.2f64,
            z: 2.3f64,
        },
        Parser::read_line(&mut BufReader::new(stream)).unwrap()
    );
}

#[test]
fn test_read_to_end_vertex_texture_uvw() {
    let stream = std::io::Cursor::new("vt 0.1 1.2 2.3");
    let exists = std::cell::Cell::new(false);
    Parser::read_to_end(&mut BufReader::new(stream), |x| {
        assert_eq!(
            Entity::VertexTexture {
                u: 0.1f64,
                v: Some(1.2f64),
                w: Some(2.3f64),
            },
            x
        );
        exists.set(true);
    })
    .unwrap();
    assert_eq!(true, exists.take());
}

#[test]
fn test_read_line_vertex_texture_uvw() {
    let stream = std::io::Cursor::new("vt 0.1 1.2 2.3");
    assert_eq!(
        Entity::VertexTexture {
            u: 0.1f64,
            v: Some(1.2f64),
            w: Some(2.3f64),
        },
        Parser::read_line(&mut BufReader::new(stream)).unwrap()
    );
}

#[test]
fn test_read_to_end_vertex_texture_uv() {
    let stream = std::io::Cursor::new("vt 0.1 1.2");
    let exists = std::cell::Cell::new(false);
    Parser::read_to_end(&mut BufReader::new(stream), |x| {
        assert_eq!(
            Entity::VertexTexture {
                u: 0.1f64,
                v: Some(1.2f64),
                w: None,
            },
            x
        );
        exists.set(true);
    })
    .unwrap();
    assert_eq!(true, exists.take());
}

#[test]
fn test_read_line_vertex_texture_uv() {
    let stream = std::io::Cursor::new("vt 0.1 1.2");
    assert_eq!(
        Entity::VertexTexture {
            u: 0.1f64,
            v: Some(1.2f64),
            w: None,
        },
        Parser::read_line(&mut BufReader::new(stream)).unwrap()
    );
}

#[test]
fn test_read_to_end_vertex_texture_u() {
    let stream = std::io::Cursor::new("vt 0.1");
    let exists = std::cell::Cell::new(false);
    Parser::read_to_end(&mut BufReader::new(stream), |x| {
        assert_eq!(
            Entity::VertexTexture {
                u: 0.1f64,
                v: None,
                w: None,
            },
            x
        );
        exists.set(true);
    })
    .unwrap();
    assert_eq!(true, exists.take());
}

#[test]
fn test_read_line_vertex_texture_u() {
    let stream = std::io::Cursor::new("vt 0.1");
    assert_eq!(
        Entity::VertexTexture {
            u: 0.1f64,
            v: None,
            w: None,
        },
        Parser::read_line(&mut BufReader::new(stream)).unwrap()
    );
}

#[test]
fn test_read_to_end_vertex_parameter_uvw() {
    let stream = std::io::Cursor::new("vp 0.1 1.2 2.3");
    let exists = std::cell::Cell::new(false);
    Parser::read_to_end(&mut BufReader::new(stream), |x| {
        assert_eq!(
            Entity::VertexParameter {
                u: 0.1f64,
                v: Some(1.2f64),
                w: Some(2.3f64),
            },
            x
        );
        exists.set(true);
    })
    .unwrap();
    assert_eq!(true, exists.take());
}

#[test]
fn test_read_line_vertex_parameter_uvw() {
    let stream = std::io::Cursor::new("vp 0.1 1.2 2.3");
    assert_eq!(
        Entity::VertexParameter {
            u: 0.1f64,
            v: Some(1.2f64),
            w: Some(2.3f64),
        },
        Parser::read_line(&mut BufReader::new(stream)).unwrap()
    );
}

#[test]
fn test_read_to_end_vertex_parameter_uv() {
    let stream = std::io::Cursor::new("vp 0.1 1.2");
    let exists = std::cell::Cell::new(false);
    Parser::read_to_end(&mut BufReader::new(stream), |x| {
        assert_eq!(
            Entity::VertexParameter {
                u: 0.1f64,
                v: Some(1.2f64),
                w: None,
            },
            x
        );
        exists.set(true);
    })
    .unwrap();
    assert_eq!(true, exists.take());
}

#[test]
fn test_read_line_vertex_parameter_uv() {
    let stream = std::io::Cursor::new("vp 0.1 1.2");
    assert_eq!(
        Entity::VertexParameter {
            u: 0.1f64,
            v: Some(1.2f64),
            w: None,
        },
        Parser::read_line(&mut BufReader::new(stream)).unwrap()
    );
}

#[test]
fn test_read_to_end_vertex_parameter_u() {
    let stream = std::io::Cursor::new("vp 0.1");
    let exists = std::cell::Cell::new(false);
    Parser::read_to_end(&mut BufReader::new(stream), |x| {
        assert_eq!(
            Entity::VertexParameter {
                u: 0.1f64,
                v: None,
                w: None,
            },
            x
        );
        exists.set(true);
    })
    .unwrap();
    assert_eq!(true, exists.take());
}

#[test]
fn test_read_line_vertex_parameter_u() {
    let stream = std::io::Cursor::new("vp 0.1");
    assert_eq!(
        Entity::VertexParameter {
            u: 0.1f64,
            v: None,
            w: None,
        },
        Parser::read_line(&mut BufReader::new(stream)).unwrap()
    );
}

#[test]
fn test_read_to_end_face_vnt_3() {
    let stream = std::io::Cursor::new("f 0/1/2 3/4/5 6/7/8");
    let exists = std::cell::Cell::new(false);
    Parser::read_to_end(&mut BufReader::new(stream), |x| {
        assert_eq!(
            Entity::Face {
                vertices: vec!(
                    FaceVertex::new_vtn(0, Some(1), Some(2)),
                    FaceVertex::new_vtn(3, Some(4), Some(5)),
                    FaceVertex::new_vtn(6, Some(7), Some(8)),
                )
            },
            x
        );
        exists.set(true);
    })
    .unwrap();
    assert_eq!(true, exists.take());
}

#[test]
fn test_read_line_face_vnt_3() {
    let stream = std::io::Cursor::new("f 0/1/2 3/4/5 6/7/8");
    assert_eq!(
        Entity::Face {
            vertices: vec!(
                FaceVertex::new_vtn(0, Some(1), Some(2)),
                FaceVertex::new_vtn(3, Some(4), Some(5)),
                FaceVertex::new_vtn(6, Some(7), Some(8)),
            )
        },
        Parser::read_line(&mut BufReader::new(stream)).unwrap()
    );
}

#[test]
fn test_read_to_end_face_vnt_5() {
    let stream = std::io::Cursor::new("f 0/1/2 3/4/5 6/7/8 9/10/11 12/13/14");
    let exists = std::cell::Cell::new(false);
    Parser::read_to_end(&mut BufReader::new(stream), |x| {
        assert_eq!(
            Entity::Face {
                vertices: vec!(
                    FaceVertex::new_vtn(0, Some(1), Some(2)),
                    FaceVertex::new_vtn(3, Some(4), Some(5)),
                    FaceVertex::new_vtn(6, Some(7), Some(8)),
                    FaceVertex::new_vtn(9, Some(10), Some(11)),
                    FaceVertex::new_vtn(12, Some(13), Some(14)),
                )
            },
            x
        );
        exists.set(true);
    })
    .unwrap();
    assert_eq!(true, exists.take());
}

#[test]
fn test_read_line_face_vnt_5() {
    let stream = std::io::Cursor::new("f 0/1/2 3/4/5 6/7/8 9/10/11 12/13/14");
    assert_eq!(
        Entity::Face {
            vertices: vec!(
                FaceVertex::new_vtn(0, Some(1), Some(2)),
                FaceVertex::new_vtn(3, Some(4), Some(5)),
                FaceVertex::new_vtn(6, Some(7), Some(8)),
                FaceVertex::new_vtn(9, Some(10), Some(11)),
                FaceVertex::new_vtn(12, Some(13), Some(14)),
            )
        },
        Parser::read_line(&mut BufReader::new(stream)).unwrap()
    );
}

#[test]
fn test_read_to_end_face_vt() {
    let stream = std::io::Cursor::new("f 0//2 3//5 6//8");
    let exists = std::cell::Cell::new(false);
    Parser::read_to_end(&mut BufReader::new(stream), |x| {
        assert_eq!(
            Entity::Face {
                vertices: vec!(
                    FaceVertex::new_vtn(0, None, Some(2)),
                    FaceVertex::new_vtn(3, None, Some(5)),
                    FaceVertex::new_vtn(6, None, Some(8)),
                )
            },
            x
        );
        exists.set(true);
    })
    .unwrap();
    assert_eq!(true, exists.take());
}

#[test]
fn test_read_line_face_vt() {
    let stream = std::io::Cursor::new("f 0//2 3//5 6//8");
    assert_eq!(
        Entity::Face {
            vertices: vec!(
                FaceVertex::new_vtn(0, None, Some(2)),
                FaceVertex::new_vtn(3, None, Some(5)),
                FaceVertex::new_vtn(6, None, Some(8)),
            )
        },
        Parser::read_line(&mut BufReader::new(stream)).unwrap()
    );
}

#[test]
fn test_read_to_end_face_vn() {
    let stream = std::io::Cursor::new("f 0/1 3/4 6/7");
    let exists = std::cell::Cell::new(false);
    Parser::read_to_end(&mut BufReader::new(stream), |x| {
        assert_eq!(
            Entity::Face {
                vertices: vec!(
                    FaceVertex::new_vtn(0, Some(1), None),
                    FaceVertex::new_vtn(3, Some(4), None),
                    FaceVertex::new_vtn(6, Some(7), None),
                )
            },
            x
        );
        exists.set(true);
    })
    .unwrap();
    assert_eq!(true, exists.take());
}

#[test]
fn test_read_line_face_vn() {
    let stream = std::io::Cursor::new("f 0/1 3/4 6/7");
    assert_eq!(
        Entity::Face {
            vertices: vec!(
                FaceVertex::new_vtn(0, Some(1), None),
                FaceVertex::new_vtn(3, Some(4), None),
                FaceVertex::new_vtn(6, Some(7), None),
            )
        },
        Parser::read_line(&mut BufReader::new(stream)).unwrap()
    );
}

#[test]
fn test_read_to_end_face_v() {
    let stream = std::io::Cursor::new("f 0 3 6");
    let exists = std::cell::Cell::new(false);
    Parser::read_to_end(&mut BufReader::new(stream), |x| {
        assert_eq!(
            Entity::Face {
                vertices: vec!(
                    FaceVertex::new_vtn(0, None, None),
                    FaceVertex::new_vtn(3, None, None),
                    FaceVertex::new_vtn(6, None, None),
                )
            },
            x
        );
        exists.set(true);
    })
    .unwrap();
    assert_eq!(true, exists.take());
}

#[test]
fn test_read_line_face_v() {
    let stream = std::io::Cursor::new("f 0 3 6");
    assert_eq!(
        Entity::Face {
            vertices: vec!(
                FaceVertex::new_vtn(0, None, None),
                FaceVertex::new_vtn(3, None, None),
                FaceVertex::new_vtn(6, None, None),
            )
        },
        Parser::read_line(&mut BufReader::new(stream)).unwrap()
    );
}

#[test]
fn test_read_to_end_line() {
    let stream = std::io::Cursor::new("l 0 1 2 3 4");
    let exists = std::cell::Cell::new(false);
    Parser::read_to_end(&mut BufReader::new(stream), |x| {
        assert_eq!(
            Entity::Line {
                vertices: vec!(0, 1, 2, 3, 4)
            },
            x
        );
        exists.set(true);
    })
    .unwrap();
    assert_eq!(true, exists.take());
}

#[test]
fn test_read_line_line() {
    let stream = std::io::Cursor::new("l 0 1 2 3 4");
    assert_eq!(
        Entity::Line {
            vertices: vec!(0, 1, 2, 3, 4)
        },
        Parser::read_line(&mut BufReader::new(stream)).unwrap()
    );
}

#[test]
fn test_read_to_end_point() {
    let stream = std::io::Cursor::new("p 0 1 2 3 4");
    let exists = std::cell::Cell::new(false);
    Parser::read_to_end(&mut BufReader::new(stream), |x| {
        assert_eq!(
            Entity::Point {
                vertices: vec!(0, 1, 2, 3, 4)
            },
            x
        );
        exists.set(true);
    })
    .unwrap();
    assert_eq!(true, exists.take());
}

#[test]
fn test_read_line_point() {
    let stream = std::io::Cursor::new("p 0 1 2 3 4");
    assert_eq!(
        Entity::Point {
            vertices: vec!(0, 1, 2, 3, 4)
        },
        Parser::read_line(&mut BufReader::new(stream)).unwrap()
    );
}
