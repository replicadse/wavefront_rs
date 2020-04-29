extern crate wavefront_rs;
use wavefront_rs::obj::entity::*;
use float_cmp::*;

#[test]
fn test_into_format_comment() {
    let entity = Entity::Comment{content: "token".to_owned()};
    assert_eq!("# token", Into::<Format>::into(entity));
}

#[test]
fn test_from_objformat_comment() {
    assert_eq!(
        Entity::from(Format::from("# token")), 
        Entity::Comment{content: "token".to_owned()});
}

#[test]
fn test_into_format_object() {
    let entity = Entity::Object{name: "token".to_owned()};
    assert_eq!("o token", Into::<Format>::into(entity));
}

#[test]
fn test_from_objformat_object() {
    assert_eq!(
        Entity::from(Format::from("o token")), 
        Entity::Object{name: "token".to_owned()});
}

#[test]
fn test_into_format_group() {
    let entity = Entity::Group{name: "token".to_owned()};
    assert_eq!("g token", Into::<Format>::into(entity));
}

#[test]
fn test_from_objformat_group() {
    assert_eq!(
        Entity::from(Format::from("g token")), 
        Entity::Group{name: "token".to_owned()});
}

#[test]
fn test_into_format_smoothing_group() {
    let entity = Entity::SmoothingGroup{name: "token".to_owned()};
    assert_eq!("s token", Into::<Format>::into(entity));
}

#[test]
fn test_from_objformat_smoothing_group() {
    assert_eq!(
        Entity::from(Format::from("s token")), 
        Entity::SmoothingGroup{name: "token".to_owned()});
}

#[test]
fn test_into_format_mtllib() {
    let entity = Entity::Mtllib{name: "token".to_owned()};
    assert_eq!("mtllib token", Into::<Format>::into(entity));
}

#[test]
fn test_from_objformat_mtllib() {
    assert_eq!(
        Entity::from(Format::from("mtllib token")), 
        Entity::Mtllib{name: "token".to_owned()});
}

#[test]
fn test_into_format_usemtl() {
    let entity = Entity::Usemtl{name: "token".to_owned()};
    assert_eq!("usemtl token", Into::<Format>::into(entity));
}

#[test]
fn test_from_objformat_usemtl() {
    assert_eq!(
        Entity::from(Format::from("usemtl token")), 
        Entity::Usemtl{name: "token".to_owned()});
}

#[test]
fn test_into_format_vertex_xyzw() {
    let entity = Entity::Vertex{
        x: 0f64,
        y: 1f64,
        z: 2f64,
        w: Some(3f64),
    };
    assert_eq!("v 0 1 2 3", Into::<Format>::into(entity));
}

#[test]
fn test_from_objformat_vertex_xyzw() {
    if let Entity::Vertex{x, y, z, w} = Entity::from(Format::from("v 0.1 1.2 2.3 3.4")) {
        assert!(approx_eq!(f64, 0.1, x, epsilon=1e-5));
        assert!(approx_eq!(f64, 1.2, y, epsilon=1e-5));
        assert!(approx_eq!(f64, 2.3, z, epsilon=1e-5));
        assert!(approx_eq!(f64, 3.4, w.unwrap(), epsilon=1e-5));
    }
    else {
        panic!()
    }
}

#[test]
fn test_into_format_vertex_xyz() {
    let entity = Entity::Vertex{
        x: 0f64,
        y: 1f64,
        z: 2f64,
        w: None,
    };
    assert_eq!("v 0 1 2", Into::<Format>::into(entity));
}

#[test]
fn test_from_objformat_vertex_xyz() {
    if let Entity::Vertex{x, y, z, w} = Entity::from(Format::from("v 0.1 1.2 2.3")) {
        assert!(approx_eq!(f64, 0.1, x, epsilon=1e-5));
        assert!(approx_eq!(f64, 1.2, y, epsilon=1e-5));
        assert!(approx_eq!(f64, 2.3, z, epsilon=1e-5));
        assert_eq!(None, w);
    }
    else {
        panic!()
    }
}

#[test]
fn test_into_format_vertex_normal() {
    let entity = Entity::VertexNormal{
        x: 0f64,
        y: 1f64,
        z: 2f64,
    };
    assert_eq!("vn 0 1 2", Into::<Format>::into(entity));
}

#[test]
fn test_from_objformat_vertex_normal() {
    if let Entity::VertexNormal{x, y, z} = Entity::from(Format::from("vn 0.1 1.2 2.3")) {
        assert!(approx_eq!(f64, 0.1, x, epsilon=1e-5));
        assert!(approx_eq!(f64, 1.2, y, epsilon=1e-5));
        assert!(approx_eq!(f64, 2.3, z, epsilon=1e-5));
    }
    else {
        panic!()
    }
}

#[test]
fn test_into_format_vertex_texture_xy() {
    let entity = Entity::VertexTexture{
        x: 0f64,
        y: 1f64,
        z: None,
    };
    assert_eq!("vt 0 1", Into::<Format>::into(entity));
}

#[test]
fn test_from_objformat_vertex_texture_xy() {
    if let Entity::VertexTexture{x, y, z} = Entity::from(Format::from("vt 0.1 1.2")) {
        assert!(approx_eq!(f64, 0.1, x, epsilon=1e-5));
        assert!(approx_eq!(f64, 1.2, y, epsilon=1e-5));
        assert_eq!(None, z);
    }
    else {
        panic!()
    }
}

#[test]
fn test_into_format_vertex_texture_xyz() {
    let entity = Entity::VertexTexture{
        x: 0f64,
        y: 1f64,
        z: Some(2f64),
    };
    assert_eq!("vt 0 1 2", Into::<Format>::into(entity));
}

#[test]
fn test_from_objformat_vertex_texture_xyz() {
    if let Entity::VertexTexture{x, y, z} = Entity::from(Format::from("vt 0.1 1.2 2.3")) {
        assert!(approx_eq!(f64, 0.1, x, epsilon=1e-5));
        assert!(approx_eq!(f64, 1.2, y, epsilon=1e-5));
        assert!(approx_eq!(f64, 2.3, z.unwrap(), epsilon=1e-5));
    }
    else {
        panic!()
    }
}

#[test]
fn test_into_format_face_vnt_3() {
    let entity = Entity::Face{
        vertices: vec!(
            FaceVertex::new2(0, Some(1), Some(2)),
            FaceVertex::new2(3, Some(4), Some(5)),
            FaceVertex::new2(6, Some(7), Some(8)),
        )
    };
    assert_eq!("f 0/1/2 3/4/5 6/7/8", Into::<Format>::into(entity));
}

#[test]
fn test_from_objformat_face_vnt_3() {
    if let Entity::Face{vertices} = Entity::from(Format::from("f 0/1/2 3/4/5 6/7/8")) {
        assert_eq!(
            vec!(
                FaceVertex::new2(0, Some(1), Some(2)),
                FaceVertex::new2(3, Some(4), Some(5)),
                FaceVertex::new2(6, Some(7), Some(8)),
            ), vertices
        );
    }
    else {
        panic!()
    }
}

#[test]
fn test_into_format_face_vnt_6() {
    let entity = Entity::Face{
        vertices: vec!(
            FaceVertex::new2(0, Some(1), Some(2)),
            FaceVertex::new2(3, Some(4), Some(5)),
            FaceVertex::new2(6, Some(7), Some(8)),
            FaceVertex::new2(9, Some(10), Some(11)),
            FaceVertex::new2(12, Some(13), Some(14)),
            FaceVertex::new2(15, Some(16), Some(17)),
        )
    };
    assert_eq!("f 0/1/2 3/4/5 6/7/8 9/10/11 12/13/14 15/16/17", Into::<Format>::into(entity));
}

#[test]
fn test_from_objformat_face_vnt_6() {
    if let Entity::Face{vertices} = Entity::from(Format::from("f 0/1/2 3/4/5 6/7/8 9/10/11 12/13/14 15/16/17")) {
        assert_eq!(
            vec!(
                FaceVertex::new2(0, Some(1), Some(2)),
                FaceVertex::new2(3, Some(4), Some(5)),
                FaceVertex::new2(6, Some(7), Some(8)),
                FaceVertex::new2(9, Some(10), Some(11)),
                FaceVertex::new2(12, Some(13), Some(14)),
                FaceVertex::new2(15, Some(16), Some(17)),
            ), vertices
        );
    }
    else {
        panic!()
    }
}

#[test]
fn test_into_format_face_vt() {
    let entity = Entity::Face{
        vertices: vec!(
            FaceVertex::new2(0, None, Some(2)),
            FaceVertex::new2(3, None, Some(5)),
            FaceVertex::new2(6, None, Some(8)),
        )
    };
    assert_eq!("f 0//2 3//5 6//8", Into::<Format>::into(entity));
}

#[test]
fn test_from_objformat_face_vt() {
    if let Entity::Face{vertices} = Entity::from(Format::from("f 0//2 3//5 6//8")) {
        assert_eq!(
            vec!(
                FaceVertex::new2(0, None, Some(2)),
                FaceVertex::new2(3, None, Some(5)),
                FaceVertex::new2(6, None, Some(8)),
            ), vertices
        );
    }
    else {
        panic!()
    }
}

#[test]
fn test_into_format_face_vn() {
    let entity = Entity::Face{
        vertices: vec!(
            FaceVertex::new2(0, Some(1), None),
            FaceVertex::new2(3, Some(4), None),
            FaceVertex::new2(6, Some(7), None),
        )
    };
    assert_eq!("f 0/1 3/4 6/7", Into::<Format>::into(entity));
}

#[test]
fn test_from_objformat_face_vn() {
    if let Entity::Face{vertices} = Entity::from(Format::from("f 0/1 3/4 6/7")) {
        assert_eq!(
            vec!(
                FaceVertex::new2(0, Some(1), None),
                FaceVertex::new2(3, Some(4), None),
                FaceVertex::new2(6, Some(7), None),
            ), vertices
        );
    }
    else {
        panic!()
    }
}

#[test]
fn test_into_format_face_v() {
    let entity = Entity::Face{
        vertices: vec!(
            FaceVertex::new2(0, None, None),
            FaceVertex::new2(3, None, None),
            FaceVertex::new2(6, None, None),
        )
    };
    assert_eq!("f 0 3 6", Into::<Format>::into(entity));
}

#[test]
fn test_from_objformat_face_v() {
    if let Entity::Face{vertices} = Entity::from(Format::from("f 0 3 6")) {
        assert_eq!(
            vec!(
                FaceVertex::new2(0, None, None),
                FaceVertex::new2(3, None, None),
                FaceVertex::new2(6, None, None),
            ), vertices
        );
    }
    else {
        panic!()
    }
}

#[test]
fn test_into_format_line() {
    let entity = Entity::Line{
        vertices: vec!(0, 1, 2, 3, 4)
    };
    assert_eq!("l 0 1 2 3 4", Into::<Format>::into(entity));
}

#[test]
fn test_from_objformat_line() {
    if let Entity::Line{vertices} = Entity::from(Format::from("l 0 1 2 3 4")) {
        assert_eq!(vec!(0, 1, 2, 3, 4), vertices);
    }
    else {
        panic!()
    }
}

#[test]
fn test_token_comment() {
    assert_eq!("#", Entity::Comment{content: "".to_owned()}.token());
}

#[test]
fn test_token_object() {
    assert_eq!("o", Entity::Object{name: "".to_owned()}.token());
}

#[test]
fn test_token_group() {
    assert_eq!("g", Entity::Group{name: "".to_owned()}.token());
}

#[test]
fn test_token_smoothing_group() {
    assert_eq!("s", Entity::SmoothingGroup{name: "".to_owned()}.token());
}

#[test]
fn test_token_mtllib() {
    assert_eq!("mtllib", Entity::Mtllib{name: "".to_owned()}.token());
}

#[test]
fn test_token_usemtl() {
    assert_eq!("usemtl", Entity::Usemtl{name: "".to_owned()}.token());
}

#[test]
fn test_token_vertex() {
    assert_eq!("v", Entity::Vertex{x: 0f64, y: 0f64, z: 0f64, w: None}.token());
}

#[test]
fn test_token_vertex_normal() {
    assert_eq!("vn", Entity::VertexNormal{x: 0f64, y: 0f64, z: 0f64}.token());
}

#[test]
fn test_token_vertex_texture() {
    assert_eq!("vt", Entity::VertexTexture{x: 0f64, y: 0f64, z: None}.token());
}

#[test]
fn test_token_face() {
    assert_eq!("f", Entity::Face{vertices: vec!()}.token());
}

#[test]
fn test_token_line() {
    assert_eq!("l", Entity::Line{vertices: vec!()}.token());
}

#[test]
fn test_to_string_comment() {
    let entity = Entity::Comment{content: "token".to_owned()};
    assert_eq!("# token", entity.to_string());
}

#[test]
fn test_to_string_object() {
    let entity = Entity::Object{name: "token".to_owned()};
    assert_eq!("o token", entity.to_string());
}

#[test]
fn test_to_string_group() {
    let entity = Entity::Group{name: "token".to_owned()};
    assert_eq!("g token", entity.to_string());
}

#[test]
fn test_to_string_smoothing_group() {
    let entity = Entity::SmoothingGroup{name: "token".to_owned()};
    assert_eq!("s token", entity.to_string());
}

#[test]
fn test_to_string_mtllib() {
    let entity = Entity::Mtllib{name: "token".to_owned()};
    assert_eq!("mtllib token", entity.to_string());
}

#[test]
fn test_to_string_usemtl() {
    let entity = Entity::Usemtl{name: "token".to_owned()};
    assert_eq!("usemtl token", entity.to_string());
}

#[test]
fn test_to_string_vertex_xyzw() {
    let entity = Entity::Vertex{
        x: 0f64,
        y: 1f64,
        z: 2f64,
        w: Some(3f64),
    };
    assert_eq!("v 0 1 2 3", entity.to_string());
}

#[test]
fn test_to_string_vertex_xyz() {
    let entity = Entity::Vertex{
        x: 0f64,
        y: 1f64,
        z: 2f64,
        w: None,
    };
    assert_eq!("v 0 1 2", entity.to_string());
}

#[test]
fn test_to_string_vertex_normal() {
    let entity = Entity::VertexNormal{
        x: 0f64,
        y: 1f64,
        z: 2f64,
    };
    assert_eq!("vn 0 1 2", entity.to_string());
}

#[test]
fn test_to_string_vertex_texture_xy() {
    let entity = Entity::VertexTexture{
        x: 0f64,
        y: 1f64,
        z: None,
    };
    assert_eq!("vt 0 1", entity.to_string());
}

#[test]
fn test_to_string_vertex_texture_xyz() {
    let entity = Entity::VertexTexture{
        x: 0f64,
        y: 1f64,
        z: Some(2f64),
    };
    assert_eq!("vt 0 1 2", entity.to_string());
}

#[test]
fn test_to_string_face_vnt_3() {
    let entity = Entity::Face{
        vertices: vec!(
            FaceVertex::new2(0, Some(1), Some(2)),
            FaceVertex::new2(3, Some(4), Some(5)),
            FaceVertex::new2(6, Some(7), Some(8)),
        )
    };
    assert_eq!("f 0/1/2 3/4/5 6/7/8", entity.to_string());
}

#[test]
fn test_to_string_face_vnt_6() {
    let entity = Entity::Face{
        vertices: vec!(
            FaceVertex::new2(0, Some(1), Some(2)),
            FaceVertex::new2(3, Some(4), Some(5)),
            FaceVertex::new2(6, Some(7), Some(8)),
            FaceVertex::new2(9, Some(10), Some(11)),
            FaceVertex::new2(12, Some(13), Some(14)),
            FaceVertex::new2(15, Some(16), Some(17)),
        )
    };
    assert_eq!("f 0/1/2 3/4/5 6/7/8 9/10/11 12/13/14 15/16/17", entity.to_string());
}

#[test]
fn test_to_string_face_vt() {
    let entity = Entity::Face{
        vertices: vec!(
            FaceVertex::new2(0, None, Some(2)),
            FaceVertex::new2(3, None, Some(5)),
            FaceVertex::new2(6, None, Some(8)),
        )
    };
    assert_eq!("f 0//2 3//5 6//8", entity.to_string());
}

#[test]
fn test_to_string_face_vn() {
    let entity = Entity::Face{
        vertices: vec!(
            FaceVertex::new2(0, Some(1), None),
            FaceVertex::new2(3, Some(4), None),
            FaceVertex::new2(6, Some(7), None),
        )
    };
    assert_eq!("f 0/1 3/4 6/7", entity.to_string());
}

#[test]
fn test_to_string_face_v() {
    let entity = Entity::Face{
        vertices: vec!(
            FaceVertex::new2(0, None, None),
            FaceVertex::new2(3, None, None),
            FaceVertex::new2(6, None, None),
        )
    };
    assert_eq!("f 0 3 6", entity.to_string());
}

#[test]
fn test_to_string_line() {
    let entity = Entity::Line{
        vertices: vec!(0, 1, 2, 3, 4)
    };
    assert_eq!("l 0 1 2 3 4", entity.to_string());
}
