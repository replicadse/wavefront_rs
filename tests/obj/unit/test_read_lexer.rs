extern crate wavefront_rs;
use std::io::BufReader;
use wavefront_rs::obj::read_lexer::*;
use wavefront_rs::obj::entity::*;
use float_cmp::*;

#[test]
fn test_read_to_end_comment() {
    let stream = std::io::Cursor::new("# token");
    let exists = std::cell::Cell::new(false);
    ReadLexer::read_to_end(&mut BufReader::new(stream), 
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
fn test_read_line_comment() {
    let stream = std::io::Cursor::new("# token");
    let exists = std::cell::Cell::new(false);
    if let Ok(Entity::Comment{content}) = ReadLexer::read_line(&mut BufReader::new(stream)) {
        if content == "token" {
            exists.set(true);
        }
    }
    assert_eq!(true, exists.take());
}

#[test]
fn test_read_to_end_object() {
    let stream = std::io::Cursor::new("o token");
    let exists = std::cell::Cell::new(false);
    ReadLexer::read_to_end(&mut BufReader::new(stream), 
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
fn test_read_line_object() {
    let stream = std::io::Cursor::new("o token");
    let exists = std::cell::Cell::new(false);
    if let Ok(Entity::Object{name}) = ReadLexer::read_line(&mut BufReader::new(stream)) {
        if name == "token" {
            exists.set(true);
        }
    }
    assert_eq!(true, exists.take());
}

#[test]
fn test_read_to_end_group() {
    let stream = std::io::Cursor::new("g token");
    let exists = std::cell::Cell::new(false);
    ReadLexer::read_to_end(&mut BufReader::new(stream), 
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
fn test_read_line_group() {
    let stream = std::io::Cursor::new("g token");
    let exists = std::cell::Cell::new(false);
    if let Ok(Entity::Group{name}) = ReadLexer::read_line(&mut BufReader::new(stream)) {
        if name == "token" {
            exists.set(true);
        }
    }
    assert_eq!(true, exists.take());
}

#[test]
fn test_read_to_end_smoothing_group() {
    let stream = std::io::Cursor::new("s token");
    let exists = std::cell::Cell::new(false);
    ReadLexer::read_to_end(&mut BufReader::new(stream), 
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
fn test_read_line_smoothing_group() {
    let stream = std::io::Cursor::new("s token");
    let exists = std::cell::Cell::new(false);
    if let Ok(Entity::SmoothingGroup{name}) = ReadLexer::read_line(&mut BufReader::new(stream)) {
        if name == "token" {
            exists.set(true);
        }
    }
    assert_eq!(true, exists.take());
}

#[test]
fn test_read_to_end_merging_group() {
    let stream = std::io::Cursor::new("mg token");
    let exists = std::cell::Cell::new(false);
    ReadLexer::read_to_end(&mut BufReader::new(stream), 
        |x| {
            if let Entity::MergingGroup{name} = x {
                if name == "token" {
                    exists.set(true);
                }
            }
        }).unwrap();
    assert_eq!(true, exists.take());
}

#[test]
fn test_read_line_merging_group() {
    let stream = std::io::Cursor::new("mg token");
    let exists = std::cell::Cell::new(false);
    if let Ok(Entity::MergingGroup{name}) = ReadLexer::read_line(&mut BufReader::new(stream)) {
        if name == "token" {
            exists.set(true);
        }
    }
    assert_eq!(true, exists.take());
}

#[test]
fn test_read_to_end_mtllib() {
    let stream = std::io::Cursor::new("mtllib token");
    let exists = std::cell::Cell::new(false);
    ReadLexer::read_to_end(&mut BufReader::new(stream), 
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
fn test_read_line_mtllib() {
    let stream = std::io::Cursor::new("mtllib token");
    let exists = std::cell::Cell::new(false);
    if let Ok(Entity::Mtllib{name}) = ReadLexer::read_line(&mut BufReader::new(stream)) {
        if name == "token" {
            exists.set(true);
        }
    }
    assert_eq!(true, exists.take());
}

#[test]
fn test_read_to_end_usemtl() {
    let stream = std::io::Cursor::new("usemtl token");
    let exists = std::cell::Cell::new(false);
    ReadLexer::read_to_end(&mut BufReader::new(stream), 
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
fn test_read_line_usemtl() {
    let stream = std::io::Cursor::new("usemtl token");
    let exists = std::cell::Cell::new(false);
    if let Ok(Entity::Usemtl{name}) = ReadLexer::read_line(&mut BufReader::new(stream)) {
        if name == "token" {
            exists.set(true);
        }
    }
    assert_eq!(true, exists.take());
}

#[test]
fn test_read_to_end_vertex_xyzw() {
    let stream = std::io::Cursor::new("v 0.1 1.2 2.3 3.4");
    let exists = std::cell::Cell::new(false);
    ReadLexer::read_to_end(&mut BufReader::new(stream), 
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
fn test_read_line_vertex_xyzw() {
    let stream = std::io::Cursor::new("v 0.1 1.2 2.3 3.4");
    let exists = std::cell::Cell::new(false);
    if let Ok(Entity::Vertex{x, y, z, w}) = ReadLexer::read_line(&mut BufReader::new(stream)) {
        assert!(approx_eq!(f64, 0.1, x, epsilon=1e-5));
        assert!(approx_eq!(f64, 1.2, y, epsilon=1e-5));
        assert!(approx_eq!(f64, 2.3, z, epsilon=1e-5));
        assert!(approx_eq!(f64, 3.4, w.unwrap(), epsilon=1e-5));
        exists.set(true);
    }
    assert_eq!(true, exists.take());
}

#[test]
fn test_read_to_end_vertex_xyz() {
    let stream = std::io::Cursor::new("v 0.1 1.2 2.3");
    let exists = std::cell::Cell::new(false);
    ReadLexer::read_to_end(&mut BufReader::new(stream), 
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
fn test_read_line_vertex_xyz() {
    let stream = std::io::Cursor::new("v 0.1 1.2 2.3");
    let exists = std::cell::Cell::new(false);
    if let Ok(Entity::Vertex{x, y, z, w}) = ReadLexer::read_line(&mut BufReader::new(stream)) {
        assert!(approx_eq!(f64, 0.1, x, epsilon=1e-5));
        assert!(approx_eq!(f64, 1.2, y, epsilon=1e-5));
        assert!(approx_eq!(f64, 2.3, z, epsilon=1e-5));
        assert_eq!(None, w);
        exists.set(true);
    }
    assert_eq!(true, exists.take());
}

#[test]
fn test_read_to_end_vertex_normal() {
    let stream = std::io::Cursor::new("vn 0.1 1.2 2.3");
    let exists = std::cell::Cell::new(false);
    ReadLexer::read_to_end(&mut BufReader::new(stream), 
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
fn test_read_line_vertex_normal() {
    let stream = std::io::Cursor::new("vn 0.1 1.2 2.3");
    let exists = std::cell::Cell::new(false);
    if let Ok(Entity::VertexNormal{x, y, z}) = ReadLexer::read_line(&mut BufReader::new(stream)) {
        assert!(approx_eq!(f64, 0.1, x, epsilon=1e-5));
        assert!(approx_eq!(f64, 1.2, y, epsilon=1e-5));
        assert!(approx_eq!(f64, 2.3, z, epsilon=1e-5));
        exists.set(true);
    }
    assert_eq!(true, exists.take());
}

#[test]
fn test_read_to_end_vertex_texture_uvw() {
    let stream = std::io::Cursor::new("vt 0.1 1.2 2.3");
    let exists = std::cell::Cell::new(false);
    ReadLexer::read_to_end(&mut BufReader::new(stream), 
        |v| {
            if let Entity::VertexTexture{u, v, w} = v {
                assert!(approx_eq!(f64, 0.1, u, epsilon=1e-5));
                assert!(approx_eq!(f64, 1.2, v.unwrap(), epsilon=1e-5));
                assert!(approx_eq!(f64, 2.3, w.unwrap(), epsilon=1e-5));
                exists.set(true);
            };
        }).unwrap();
    assert_eq!(true, exists.take());
}

#[test]
fn test_read_line_vertex_texture_uvw() {
    let stream = std::io::Cursor::new("vt 0.1 1.2 2.3");
    let exists = std::cell::Cell::new(false);
    if let Ok(Entity::VertexTexture{u, v, w}) = ReadLexer::read_line(&mut BufReader::new(stream)) {
        assert!(approx_eq!(f64, 0.1, u, epsilon=1e-5));
        assert!(approx_eq!(f64, 1.2, v.unwrap(), epsilon=1e-5));
        assert!(approx_eq!(f64, 2.3, w.unwrap(), epsilon=1e-5));
        exists.set(true);
    }
    assert_eq!(true, exists.take());
}

#[test]
fn test_read_to_end_vertex_texture_uv() {
    let stream = std::io::Cursor::new("vt 0.1 1.2");
    let exists = std::cell::Cell::new(false);
    ReadLexer::read_to_end(&mut BufReader::new(stream), 
        |v| {
            if let Entity::VertexTexture{u, v, w} = v {
                assert!(approx_eq!(f64, 0.1, u, epsilon=1e-5));
                assert!(approx_eq!(f64, 1.2, v.unwrap(), epsilon=1e-5));
                assert_eq!(None, w);
                exists.set(true);
            };
        }).unwrap();
    assert_eq!(true, exists.take());
}

#[test]
fn test_read_line_vertex_texture_uv() {
    let stream = std::io::Cursor::new("vt 0.1 1.2");
    let exists = std::cell::Cell::new(false);
    if let Ok(Entity::VertexTexture{u, v, w}) = ReadLexer::read_line(&mut BufReader::new(stream)) {
        assert!(approx_eq!(f64, 0.1, u, epsilon=1e-5));
        assert!(approx_eq!(f64, 1.2, v.unwrap(), epsilon=1e-5));
        assert_eq!(None, w);
        exists.set(true);
    }
    assert_eq!(true, exists.take());
}

#[test]
fn test_read_to_end_vertex_texture_u() {
    let stream = std::io::Cursor::new("vt 0.1");
    let exists = std::cell::Cell::new(false);
    ReadLexer::read_to_end(&mut BufReader::new(stream), 
        |v| {
            if let Entity::VertexTexture{u, v, w} = v {
                assert!(approx_eq!(f64, 0.1, u, epsilon=1e-5));
                assert_eq!(None, v);
                assert_eq!(None, w);
                exists.set(true);
            };
        }).unwrap();
    assert_eq!(true, exists.take());
}

#[test]
fn test_read_line_vertex_texture_u() {
    let stream = std::io::Cursor::new("vt 0.1");
    let exists = std::cell::Cell::new(false);
    if let Ok(Entity::VertexTexture{u, v, w}) = ReadLexer::read_line(&mut BufReader::new(stream)) {
        assert!(approx_eq!(f64, 0.1, u, epsilon=1e-5));
        assert_eq!(None, v);
        assert_eq!(None, w);
        exists.set(true);
    }
    assert_eq!(true, exists.take());
}

#[test]
fn test_read_to_end_vertex_parameter_uvw() {
    let stream = std::io::Cursor::new("vp 0.1 1.2 2.3");
    ReadLexer::read_to_end(&mut BufReader::new(stream), 
        |v| {
            assert_eq!(Entity::VertexParameter{
                u: 0.1f64,
                v: Some(1.2f64),
                w: Some(2.3f64),
            }, v);
        }).unwrap();
}

#[test]
fn test_read_line_vertex_parameter_uvw() {
    let stream = std::io::Cursor::new("vp 0.1 1.2 2.3");
    assert_eq!(Entity::VertexParameter{
        u: 0.1f64,
        v: Some(1.2f64),
        w: Some(2.3f64),
    }, ReadLexer::read_line(&mut BufReader::new(stream)).unwrap());
}

#[test]
fn test_read_to_end_vertex_parameter_uv() {
    let stream = std::io::Cursor::new("vp 0.1 1.2");
    ReadLexer::read_to_end(&mut BufReader::new(stream), 
        |v| {
            assert_eq!(Entity::VertexParameter{
                u: 0.1f64,
                v: Some(1.2f64),
                w: None,
            }, v);
        }).unwrap();
}

#[test]
fn test_read_line_vertex_parameter_uv() {
    let stream = std::io::Cursor::new("vp 0.1 1.2");
    assert_eq!(Entity::VertexParameter{
        u: 0.1f64,
        v: Some(1.2f64),
        w: None,
    }, ReadLexer::read_line(&mut BufReader::new(stream)).unwrap());
}

#[test]
fn test_read_to_end_vertex_parameter_u() {
    let stream = std::io::Cursor::new("vp 0.1");
    ReadLexer::read_to_end(&mut BufReader::new(stream), 
        |v| {
            assert_eq!(Entity::VertexParameter{
                u: 0.1f64,
                v: None,
                w: None,
            }, v);
        }).unwrap();
}

#[test]
fn test_read_line_vertex_parameter_u() {
    let stream = std::io::Cursor::new("vp 0.1");
    assert_eq!(Entity::VertexParameter{
        u: 0.1f64,
        v: None,
        w: None,
    }, ReadLexer::read_line(&mut BufReader::new(stream)).unwrap());
}

#[test]
fn test_read_to_end_face_vnt_3() {
    let stream = std::io::Cursor::new("f 0/1/2 3/4/5 6/7/8");
    let exists = std::cell::Cell::new(false);
    ReadLexer::read_to_end(&mut BufReader::new(stream), 
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
fn test_read_line_face_vnt_3() {
    let stream = std::io::Cursor::new("f 0/1/2 3/4/5 6/7/8");
    let exists = std::cell::Cell::new(false);
    if let Ok(Entity::Face{vertices}) = ReadLexer::read_line(&mut BufReader::new(stream)) {
        assert_eq!(3, vertices.len());
        assert_eq!(FaceVertex::new2(0, Some(1), Some(2)), vertices[0]);
        assert_eq!(FaceVertex::new2(3, Some(4), Some(5)), vertices[1]);
        assert_eq!(FaceVertex::new2(6, Some(7), Some(8)), vertices[2]);
        exists.set(true);
    }
    assert_eq!(true, exists.take());
}

#[test]
fn test_read_to_end_face_vnt_6() {
    let stream = std::io::Cursor::new("f 0/1/2 3/4/5 6/7/8 9/10/11 12/13/14");
    let exists = std::cell::Cell::new(false);
    ReadLexer::read_to_end(&mut BufReader::new(stream), 
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
fn test_read_line_face_vnt_6() {
    let stream = std::io::Cursor::new("f 0/1/2 3/4/5 6/7/8 9/10/11 12/13/14");
    let exists = std::cell::Cell::new(false);
    if let Ok(Entity::Face{vertices}) = ReadLexer::read_line(&mut BufReader::new(stream)) {
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
fn test_read_to_end_face_vt() {
    let stream = std::io::Cursor::new("f 0//2 3//5 6//8");
    let exists = std::cell::Cell::new(false);
    ReadLexer::read_to_end(&mut BufReader::new(stream), 
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
fn test_read_line_face_vt() {
    let stream = std::io::Cursor::new("f 0//2 3//5 6//8");
    let exists = std::cell::Cell::new(false);
    if let Ok(Entity::Face{vertices}) = ReadLexer::read_line(&mut BufReader::new(stream)) {
        assert_eq!(3, vertices.len());
        assert_eq!(FaceVertex::new2(0, None, Some(2)), vertices[0]);
        assert_eq!(FaceVertex::new2(3, None, Some(5)), vertices[1]);
        assert_eq!(FaceVertex::new2(6, None, Some(8)), vertices[2]);
        exists.set(true);
    }
    assert_eq!(true, exists.take());
}

#[test]
fn test_read_to_end_face_vn() {
    let stream = std::io::Cursor::new("f 0/1 3/4 6/7");
    let exists = std::cell::Cell::new(false);
    ReadLexer::read_to_end(&mut BufReader::new(stream), 
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
fn test_read_line_face_vn() {
    let stream = std::io::Cursor::new("f 0/1 3/4 6/7");
    let exists = std::cell::Cell::new(false);
    if let Ok(Entity::Face{vertices}) = ReadLexer::read_line(&mut BufReader::new(stream)) {
        assert_eq!(3, vertices.len());
        assert_eq!(FaceVertex::new2(0, Some(1), None), vertices[0]);
        assert_eq!(FaceVertex::new2(3, Some(4), None), vertices[1]);
        assert_eq!(FaceVertex::new2(6, Some(7), None), vertices[2]);
        exists.set(true);
    }
    assert_eq!(true, exists.take());
}

#[test]
fn test_read_to_end_face_v() {
    let stream = std::io::Cursor::new("f 0 3 6");
    let exists = std::cell::Cell::new(false);
    ReadLexer::read_to_end(&mut BufReader::new(stream), 
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
fn test_read_line_face_v() {
    let stream = std::io::Cursor::new("f 0 3 6");
    let exists = std::cell::Cell::new(false);
    if let Ok(Entity::Face{vertices}) = ReadLexer::read_line(&mut BufReader::new(stream)) {
        assert_eq!(3, vertices.len());
        assert_eq!(FaceVertex::new2(0, None, None), vertices[0]);
        assert_eq!(FaceVertex::new2(3, None, None), vertices[1]);
        assert_eq!(FaceVertex::new2(6, None, None), vertices[2]);
        exists.set(true);
    }
    assert_eq!(true, exists.take());
}

#[test]
fn test_read_to_end_line() {
    let stream = std::io::Cursor::new("l 0 1 2 3 4");
    let exists = std::cell::Cell::new(false);
    ReadLexer::read_to_end(&mut BufReader::new(stream), 
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

#[test]
fn test_read_line_line() {
    let stream = std::io::Cursor::new("l 0 1 2 3 4");
    let exists = std::cell::Cell::new(false);
    if let Ok(Entity::Line{vertices}) = ReadLexer::read_line(&mut BufReader::new(stream)) {
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
