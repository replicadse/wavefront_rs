extern crate wavefront_rs;
use wavefront_rs::entity::*;

#[test]
fn test_comment() {
    let entity = Entity::Comment{content: "token".to_owned()};
    assert_eq!("# token", entity.to_string());
}

#[test]
fn test_object() {
    let entity = Entity::Object{name: "token".to_owned()};
    assert_eq!("o token", entity.to_string());
}

#[test]
fn test_group() {
    let entity = Entity::Group{name: "token".to_owned()};
    assert_eq!("g token", entity.to_string());
}

#[test]
fn test_smoothing_group() {
    let entity = Entity::SmoothingGroup{name: "token".to_owned()};
    assert_eq!("s token", entity.to_string());
}

#[test]
fn test_mtllib() {
    let entity = Entity::Mtllib{name: "token".to_owned()};
    assert_eq!("mtllib token", entity.to_string());
}

#[test]
fn test_usemtl() {
    let entity = Entity::Usemtl{name: "token".to_owned()};
    assert_eq!("usemtl token", entity.to_string());
}

#[test]
fn test_vertex_xyzw() {
    let entity = Entity::Vertex{
        x: 0f64,
        y: 1f64,
        z: 2f64,
        w: Some(3f64),
    };
    assert_eq!("v 0 1 2 3", entity.to_string());
}

#[test]
fn test_vertex_xyz() {
    let entity = Entity::Vertex{
        x: 0f64,
        y: 1f64,
        z: 2f64,
        w: None,
    };
    assert_eq!("v 0 1 2", entity.to_string());
}

#[test]
fn test_vertex_normal() {
    let entity = Entity::VertexNormal{
        x: 0f64,
        y: 1f64,
        z: 2f64,
    };
    assert_eq!("vn 0 1 2", entity.to_string());
}

#[test]
fn test_vertex_texture_xy() {
    let entity = Entity::VertexTexture{
        x: 0f64,
        y: 1f64,
        z: None,
    };
    assert_eq!("vt 0 1", entity.to_string());
}

#[test]
fn test_vertex_texture_xyz() {
    let entity = Entity::VertexTexture{
        x: 0f64,
        y: 1f64,
        z: Some(2f64),
    };
    assert_eq!("vt 0 1 2", entity.to_string());
}

#[test]
fn test_face_vnt_3() {
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
fn test_face_vnt_6() {
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
fn test_face_vt() {
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
fn test_face_vn() {
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
fn test_face_v() {
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
fn test_line() {
    let entity = Entity::Line{
        vertices: vec!(0, 1, 2, 3, 4)
    };
    assert_eq!("l 0 1 2 3 4", entity.to_string());
}
