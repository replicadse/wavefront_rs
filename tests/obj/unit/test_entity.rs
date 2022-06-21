use wavefront_rs::obj::entity::*;

#[test]
fn test_into_format_comment() {
    let entity = Entity::Comment {
        content: "token".to_owned(),
    };
    assert_eq!("# token", Into::<Format>::into(entity));
}

#[test]
fn test_from_objformat_comment() {
    assert_eq!(
        Entity::from(Format::from("# token")),
        Entity::Comment {
            content: "token".to_owned()
        }
    );
}

#[test]
fn test_into_format_object() {
    let entity = Entity::Object {
        name: "token".to_owned(),
    };
    assert_eq!("o token", Into::<Format>::into(entity));
}

#[test]
fn test_from_objformat_object() {
    assert_eq!(
        Entity::from(Format::from("o token")),
        Entity::Object {
            name: "token".to_owned()
        }
    );
}

#[test]
fn test_into_format_group() {
    let entity = Entity::Group {
        name: "token".to_owned(),
    };
    assert_eq!("g token", Into::<Format>::into(entity));
}

#[test]
fn test_from_objformat_group() {
    assert_eq!(
        Entity::from(Format::from("g token")),
        Entity::Group {
            name: "token".to_owned()
        }
    );
}

#[test]
fn test_into_format_smoothing_group() {
    let entity = Entity::SmoothingGroup {
        name: "token".to_owned(),
    };
    assert_eq!("s token", Into::<Format>::into(entity));
}

#[test]
fn test_from_objformat_smoothing_group() {
    assert_eq!(
        Entity::from(Format::from("s token")),
        Entity::SmoothingGroup {
            name: "token".to_owned()
        }
    );
}

#[test]
fn test_into_format_merging_group() {
    let entity = Entity::MergingGroup {
        name: "token".to_owned(),
    };
    assert_eq!("mg token", Into::<Format>::into(entity));
}

#[test]
fn test_from_objformat_merging_group() {
    assert_eq!(
        Entity::from(Format::from("mg token")),
        Entity::MergingGroup {
            name: "token".to_owned()
        }
    );
}

#[test]
fn test_into_format_mtllib() {
    let entity = Entity::Mtllib {
        name: "token".to_owned(),
    };
    assert_eq!("mtllib token", Into::<Format>::into(entity));
}

#[test]
fn test_from_objformat_mtllib() {
    assert_eq!(
        Entity::from(Format::from("mtllib token")),
        Entity::Mtllib {
            name: "token".to_owned()
        }
    );
}

#[test]
fn test_into_format_usemtl() {
    let entity = Entity::Usemtl {
        name: "token".to_owned(),
    };
    assert_eq!("usemtl token", Into::<Format>::into(entity));
}

#[test]
fn test_from_objformat_usemtl() {
    assert_eq!(
        Entity::from(Format::from("usemtl token")),
        Entity::Usemtl {
            name: "token".to_owned()
        }
    );
}

#[test]
fn test_into_format_vertex_xyzw() {
    let entity = Entity::Vertex {
        x: 0f64,
        y: 1f64,
        z: 2f64,
        w: Some(3f64),
    };
    assert_eq!("v 0 1 2 3", Into::<Format>::into(entity));
}

#[test]
fn test_from_objformat_vertex_xyzw() {
    assert_eq!(
        Entity::Vertex {
            x: 0.1,
            y: 1.2,
            z: 2.3,
            w: Some(3.4),
        },
        Entity::from(Format::from("v 0.1 1.2 2.3 3.4"))
    );
}

#[test]
fn test_into_format_vertex_xyz() {
    let entity = Entity::Vertex {
        x: 0f64,
        y: 1f64,
        z: 2f64,
        w: None,
    };
    assert_eq!("v 0 1 2", Into::<Format>::into(entity));
}

#[test]
fn test_from_objformat_vertex_xyz() {
    assert_eq!(
        Entity::Vertex {
            x: 0.1f64,
            y: 1.2f64,
            z: 2.3f64,
            w: None,
        },
        Entity::from(Format::from("v 0.1 1.2 2.3"))
    );
}

#[test]
fn test_into_format_vertex_normal() {
    let entity = Entity::VertexNormal {
        x: 0f64,
        y: 1f64,
        z: 2f64,
    };
    assert_eq!("vn 0 1 2", Into::<Format>::into(entity));
}

#[test]
fn test_from_objformat_vertex_normal() {
    assert_eq!(
        Entity::VertexNormal {
            x: 0.1f64,
            y: 1.2f64,
            z: 2.3f64,
        },
        Entity::from(Format::from("vn 0.1 1.2 2.3"))
    );
}

#[test]
fn test_into_format_vertex_texture_u() {
    let entity = Entity::VertexTexture {
        u: 0f64,
        v: None,
        w: None,
    };
    assert_eq!("vt 0", Into::<Format>::into(entity));
}

#[test]
fn test_from_objformat_vertex_texture_u() {
    assert_eq!(
        Entity::VertexTexture {
            u: 0.1f64,
            v: None,
            w: None,
        },
        Entity::from(Format::from("vt 0.1"))
    );
}

#[test]
fn test_into_format_vertex_texture_uv() {
    let entity = Entity::VertexTexture {
        u: 0f64,
        v: Some(1f64),
        w: None,
    };
    assert_eq!("vt 0 1", Into::<Format>::into(entity));
}

#[test]
fn test_from_objformat_vertex_texture_uv() {
    assert_eq!(
        Entity::VertexTexture {
            u: 0.1f64,
            v: Some(1.2f64),
            w: None,
        },
        Entity::from(Format::from("vt 0.1 1.2"))
    );
}

#[test]
fn test_into_format_vertex_texture_uvw() {
    let entity = Entity::VertexTexture {
        u: 0f64,
        v: Some(1f64),
        w: Some(2f64),
    };
    assert_eq!("vt 0 1 2", Into::<Format>::into(entity));
}

#[test]
fn test_from_objformat_vertex_texture_uvw() {
    assert_eq!(
        Entity::VertexTexture {
            u: 0.1f64,
            v: Some(1.2f64),
            w: Some(2.3f64),
        },
        Entity::from(Format::from("vt 0.1 1.2 2.3"))
    );
}

#[test]
fn test_into_format_vertex_parameter_u() {
    let entity = Entity::VertexParameter {
        u: 0.1f64,
        v: None,
        w: None,
    };
    assert_eq!("vp 0.1", Into::<Format>::into(entity));
}

#[test]
fn test_from_objformat_vertex_parameter_u() {
    assert_eq!(
        Entity::VertexParameter {
            u: 0.1f64,
            v: None,
            w: None,
        },
        Entity::from(Format::from("vp 0.1"))
    );
}

#[test]
fn test_into_format_vertex_parameter_uv() {
    let entity = Entity::VertexParameter {
        u: 0.1f64,
        v: Some(1.2f64),
        w: None,
    };
    assert_eq!("vp 0.1 1.2", Into::<Format>::into(entity));
}

#[test]
fn test_from_objformat_vertex_parameter_uv() {
    assert_eq!(
        Entity::VertexParameter {
            u: 0.1f64,
            v: Some(1.2f64),
            w: None,
        },
        Entity::from(Format::from("vp 0.1 1.2"))
    );
}

#[test]
fn test_into_format_vertex_parameter_uvw() {
    let entity = Entity::VertexParameter {
        u: 0.1f64,
        v: Some(1.2f64),
        w: Some(2.3f64),
    };
    assert_eq!("vp 0.1 1.2 2.3", Into::<Format>::into(entity));
}

#[test]
fn test_from_objformat_vertex_parameter_uvw() {
    assert_eq!(
        Entity::VertexParameter {
            u: 0.1f64,
            v: Some(1.2f64),
            w: Some(2.3f64),
        },
        Entity::from(Format::from("vp 0.1 1.2 2.3"))
    );
}

#[test]
fn test_into_format_face_vtn_3() {
    let entity = Entity::Face {
        vertices: vec![
            FaceVertex::new_vtn(0, Some(1), Some(2)),
            FaceVertex::new_vtn(3, Some(4), Some(5)),
            FaceVertex::new_vtn(6, Some(7), Some(8)),
        ],
    };
    assert_eq!("f 0/1/2 3/4/5 6/7/8", Into::<Format>::into(entity));
}

#[test]
fn test_from_objformat_face_vtn_3() {
    assert_eq!(
        Entity::Face {
            vertices: vec!(
                FaceVertex::new_vtn(0, Some(1), Some(2)),
                FaceVertex::new_vtn(3, Some(4), Some(5)),
                FaceVertex::new_vtn(6, Some(7), Some(8)),
            )
        },
        Entity::from(Format::from("f 0/1/2 3/4/5 6/7/8"))
    );
}

#[test]
fn test_into_format_face_vtn_6() {
    let entity = Entity::Face {
        vertices: vec![
            FaceVertex::new_vtn(0, Some(1), Some(2)),
            FaceVertex::new_vtn(3, Some(4), Some(5)),
            FaceVertex::new_vtn(6, Some(7), Some(8)),
            FaceVertex::new_vtn(9, Some(10), Some(11)),
            FaceVertex::new_vtn(12, Some(13), Some(14)),
            FaceVertex::new_vtn(15, Some(16), Some(17)),
        ],
    };
    assert_eq!(
        "f 0/1/2 3/4/5 6/7/8 9/10/11 12/13/14 15/16/17",
        Into::<Format>::into(entity)
    );
}

#[test]
fn test_from_objformat_face_vtn_5() {
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
        Entity::from(Format::from("f 0/1/2 3/4/5 6/7/8 9/10/11 12/13/14"))
    );
}

#[test]
fn test_into_format_face_vt() {
    let entity = Entity::Face {
        vertices: vec![
            FaceVertex::new_vtn(0, Some(1), None),
            FaceVertex::new_vtn(3, Some(4), None),
            FaceVertex::new_vtn(6, Some(7), None),
        ],
    };
    assert_eq!("f 0/1 3/4 6/7", Into::<Format>::into(entity));
}

#[test]
fn test_from_objformat_face_vt() {
    assert_eq!(
        Entity::Face {
            vertices: vec!(
                FaceVertex::new_vtn(0, None, Some(2)),
                FaceVertex::new_vtn(3, None, Some(5)),
                FaceVertex::new_vtn(6, None, Some(8)),
            )
        },
        Entity::from(Format::from("f 0//2 3//5 6//8"))
    );
}

#[test]
fn test_into_format_face_vn() {
    let entity = Entity::Face {
        vertices: vec![
            FaceVertex::new_vtn(0, Some(1), None),
            FaceVertex::new_vtn(3, Some(4), None),
            FaceVertex::new_vtn(6, Some(7), None),
        ],
    };
    assert_eq!("f 0/1 3/4 6/7", Into::<Format>::into(entity));
}

#[test]
fn test_from_objformat_face_vn() {
    assert_eq!(
        Entity::Face {
            vertices: vec!(
                FaceVertex::new_vtn(0, Some(1), None),
                FaceVertex::new_vtn(3, Some(4), None),
                FaceVertex::new_vtn(6, Some(7), None),
            )
        },
        Entity::from(Format::from("f 0/1 3/4 6/7"))
    );
}

#[test]
fn test_into_format_face_v() {
    let entity = Entity::Face {
        vertices: vec![
            FaceVertex::new_vtn(0, None, None),
            FaceVertex::new_vtn(3, None, None),
            FaceVertex::new_vtn(6, None, None),
        ],
    };
    assert_eq!("f 0 3 6", Into::<Format>::into(entity));
}

#[test]
fn test_from_objformat_face_v() {
    assert_eq!(
        Entity::Face {
            vertices: vec!(
                FaceVertex::new_vtn(0, None, None),
                FaceVertex::new_vtn(3, None, None),
                FaceVertex::new_vtn(6, None, None),
            )
        },
        Entity::from(Format::from("f 0 3 6"))
    );
}

#[test]
fn test_into_format_line() {
    let entity = Entity::Line {
        vertices: vec![0, 1, 2, 3, 4],
    };
    assert_eq!("l 0 1 2 3 4", Into::<Format>::into(entity));
}

#[test]
fn test_from_objformat_line() {
    assert_eq!(
        Entity::Line {
            vertices: vec!(0, 1, 2, 3, 4)
        },
        Entity::from(Format::from("l 0 1 2 3 4"))
    );
}

#[test]
fn test_into_format_point() {
    let entity = Entity::Point {
        vertices: vec![0, 1, 2, 3, 4],
    };
    assert_eq!("p 0 1 2 3 4", Into::<Format>::into(entity));
}

#[test]
fn test_from_objformat_point() {
    assert_eq!(
        Entity::Point {
            vertices: vec!(0, 1, 2, 3, 4)
        },
        Entity::from(Format::from("p 0 1 2 3 4"))
    );
}

#[test]
fn test_token_comment() {
    assert_eq!(
        "#",
        Entity::Comment {
            content: "".to_owned()
        }
        .token()
    );
}

#[test]
fn test_token_object() {
    assert_eq!(
        "o",
        Entity::Object {
            name: "".to_owned()
        }
        .token()
    );
}

#[test]
fn test_token_group() {
    assert_eq!(
        "g",
        Entity::Group {
            name: "".to_owned()
        }
        .token()
    );
}

#[test]
fn test_token_smoothing_group() {
    assert_eq!(
        "s",
        Entity::SmoothingGroup {
            name: "".to_owned()
        }
        .token()
    );
}

#[test]
fn test_token_merging_group() {
    assert_eq!(
        "mg",
        Entity::MergingGroup {
            name: "".to_owned()
        }
        .token()
    );
}

#[test]
fn test_token_mtllib() {
    assert_eq!(
        "mtllib",
        Entity::Mtllib {
            name: "".to_owned()
        }
        .token()
    );
}

#[test]
fn test_token_usemtl() {
    assert_eq!(
        "usemtl",
        Entity::Usemtl {
            name: "".to_owned()
        }
        .token()
    );
}

#[test]
fn test_token_vertex() {
    assert_eq!(
        "v",
        Entity::Vertex {
            x: 0f64,
            y: 0f64,
            z: 0f64,
            w: None
        }
        .token()
    );
}

#[test]
fn test_token_vertex_normal() {
    assert_eq!(
        "vn",
        Entity::VertexNormal {
            x: 0f64,
            y: 0f64,
            z: 0f64
        }
        .token()
    );
}

#[test]
fn test_token_vertex_texture() {
    assert_eq!(
        "vt",
        Entity::VertexTexture {
            u: 0f64,
            v: None,
            w: None
        }
        .token()
    );
}

#[test]
fn test_token_vertex_parameter() {
    assert_eq!(
        "vp",
        Entity::VertexParameter {
            u: 0f64,
            v: None,
            w: None
        }
        .token()
    );
}

#[test]
fn test_token_face() {
    assert_eq!("f", Entity::Face { vertices: vec!() }.token());
}

#[test]
fn test_token_line() {
    assert_eq!("l", Entity::Line { vertices: vec!() }.token());
}

#[test]
fn test_token_point() {
    assert_eq!("p", Entity::Point { vertices: vec!() }.token());
}

#[test]
fn test_to_string_comment() {
    let entity = Entity::Comment {
        content: "token".to_owned(),
    };
    assert_eq!("# token", entity.to_string());
}

#[test]
fn test_to_string_object() {
    let entity = Entity::Object {
        name: "token".to_owned(),
    };
    assert_eq!("o token", entity.to_string());
}

#[test]
fn test_to_string_group() {
    let entity = Entity::Group {
        name: "token".to_owned(),
    };
    assert_eq!("g token", entity.to_string());
}

#[test]
fn test_to_string_smoothing_group() {
    let entity = Entity::SmoothingGroup {
        name: "token".to_owned(),
    };
    assert_eq!("s token", entity.to_string());
}

#[test]
fn test_to_string_merging_group() {
    let entity = Entity::MergingGroup {
        name: "token".to_owned(),
    };
    assert_eq!("mg token", entity.to_string());
}

#[test]
fn test_to_string_mtllib() {
    let entity = Entity::Mtllib {
        name: "token".to_owned(),
    };
    assert_eq!("mtllib token", entity.to_string());
}

#[test]
fn test_to_string_usemtl() {
    let entity = Entity::Usemtl {
        name: "token".to_owned(),
    };
    assert_eq!("usemtl token", entity.to_string());
}

#[test]
fn test_to_string_vertex_xyzw() {
    let entity = Entity::Vertex {
        x: 0f64,
        y: 1f64,
        z: 2f64,
        w: Some(3f64),
    };
    assert_eq!("v 0 1 2 3", entity.to_string());
}

#[test]
fn test_to_string_vertex_xyz() {
    let entity = Entity::Vertex {
        x: 0f64,
        y: 1f64,
        z: 2f64,
        w: None,
    };
    assert_eq!("v 0 1 2", entity.to_string());
}

#[test]
fn test_to_string_vertex_normal() {
    let entity = Entity::VertexNormal {
        x: 0f64,
        y: 1f64,
        z: 2f64,
    };
    assert_eq!("vn 0 1 2", entity.to_string());
}

#[test]
fn test_to_string_vertex_texture_u() {
    let entity = Entity::VertexTexture {
        u: 0.1f64,
        v: None,
        w: None,
    };
    assert_eq!("vt 0.1", entity.to_string());
}

#[test]
fn test_to_string_vertex_texture_uv() {
    let entity = Entity::VertexTexture {
        u: 0.1f64,
        v: Some(1.2f64),
        w: None,
    };
    assert_eq!("vt 0.1 1.2", entity.to_string());
}

#[test]
fn test_to_string_vertex_texture_uvw() {
    let entity = Entity::VertexTexture {
        u: 0f64,
        v: Some(1f64),
        w: Some(2f64),
    };
    assert_eq!("vt 0 1 2", entity.to_string());
}

#[test]
fn test_to_string_vertex_parameter_uvw() {
    let entity = Entity::VertexParameter {
        u: 0.1f64,
        v: Some(1.2f64),
        w: Some(2.3f64),
    };
    assert_eq!("vp 0.1 1.2 2.3", entity.to_string());
}

#[test]
fn test_to_string_vertex_parameter_uv() {
    let entity = Entity::VertexParameter {
        u: 0.1f64,
        v: Some(1.2f64),
        w: None,
    };
    assert_eq!("vp 0.1 1.2", entity.to_string());
}

#[test]
fn test_to_string_vertex_parameter_u() {
    let entity = Entity::VertexParameter {
        u: 0.1f64,
        v: None,
        w: None,
    };
    assert_eq!("vp 0.1", entity.to_string());
}

#[test]
fn test_to_string_face_vtn_3() {
    let entity = Entity::Face {
        vertices: vec![
            FaceVertex::new_vtn(0, Some(1), Some(2)),
            FaceVertex::new_vtn(3, Some(4), Some(5)),
            FaceVertex::new_vtn(6, Some(7), Some(8)),
        ],
    };
    assert_eq!("f 0/1/2 3/4/5 6/7/8", entity.to_string());
}

#[test]
fn test_to_string_face_vtn_6() {
    let entity = Entity::Face {
        vertices: vec![
            FaceVertex::new_vtn(0, Some(1), Some(2)),
            FaceVertex::new_vtn(3, Some(4), Some(5)),
            FaceVertex::new_vtn(6, Some(7), Some(8)),
            FaceVertex::new_vtn(9, Some(10), Some(11)),
            FaceVertex::new_vtn(12, Some(13), Some(14)),
            FaceVertex::new_vtn(15, Some(16), Some(17)),
        ],
    };
    assert_eq!(
        "f 0/1/2 3/4/5 6/7/8 9/10/11 12/13/14 15/16/17",
        entity.to_string()
    );
}

#[test]
fn test_to_string_face_vt() {
    let entity = Entity::Face {
        vertices: vec![
            FaceVertex::new_vtn(0, None, Some(2)),
            FaceVertex::new_vtn(3, None, Some(5)),
            FaceVertex::new_vtn(6, None, Some(8)),
        ],
    };
    assert_eq!("f 0//2 3//5 6//8", entity.to_string());
}

#[test]
fn test_to_string_face_vn() {
    let entity = Entity::Face {
        vertices: vec![
            FaceVertex::new_vtn(0, None, Some(2)),
            FaceVertex::new_vtn(3, None, Some(5)),
            FaceVertex::new_vtn(6, None, Some(8)),
        ],
    };
    assert_eq!("f 0//2 3//5 6//8", entity.to_string());
}

#[test]
fn test_to_string_face_v() {
    let entity = Entity::Face {
        vertices: vec![
            FaceVertex::new_vtn(0, None, None),
            FaceVertex::new_vtn(3, None, None),
            FaceVertex::new_vtn(6, None, None),
        ],
    };
    assert_eq!("f 0 3 6", entity.to_string());
}

#[test]
fn test_to_string_line() {
    let entity = Entity::Line {
        vertices: vec![0, 1, 2, 3, 4],
    };
    assert_eq!("l 0 1 2 3 4", entity.to_string());
}

#[test]
fn test_to_string_point() {
    let entity = Entity::Point {
        vertices: vec![0, 1, 2, 3, 4],
    };
    assert_eq!("p 0 1 2 3 4", entity.to_string());
}
