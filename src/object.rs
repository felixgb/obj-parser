use std::str::from_utf8;
use nom::*;
use nom::IResult::*;
use std::io::prelude::*;
use std::fs::File;

pub fn parse_file_to_object(path: &str) -> Result<Object, String> {
    let mut f = File::open(&path).unwrap();
    let mut inp = String::new();
    f.read_to_string(&mut inp).unwrap();
    parse_object(&inp)
}

pub fn parse_object(inp: &str) -> Result<Object, String> {
    match toplevel(inp.as_bytes()) {
        Done(_, parsed) => Ok(parse_res_to_object(parsed)),
        Error(err) => Err(err.to_string()),
        Incomplete(needed) => Err("incomplete!".to_string()),
    }
}

pub enum ParseResult {
    Vertex(f32, f32, f32, f32),
    TextCoord(f32, f32, f32),
    VertexNormal(f32, f32, f32),
    ParamSpaceVert(f32, Option<f32>, Option<f32>),
    Face(Vec<FaceVertex>),
    Comment,
}

fn parse_res_to_object(parse_results: Vec<ParseResult>) -> Object {
    let mut vertices = Vec::new();
    let mut text_coords = Vec::new();
    let mut vertex_normals = Vec::new();
    let mut param_space_verts = Vec::new();
    let mut faces = Vec::new();

    for p in parse_results {
        match p {
            ParseResult::Vertex(x, y, z, w) => vertices.push(Vertex { x: x, y: y, z: z, w: w }),
            ParseResult::TextCoord(u, v, w) => text_coords.push(TextCoord { u: u, v: v, w: w }),
            ParseResult::VertexNormal(x, y, z) => vertex_normals.push(VertexNormal { x: x, y: y, z: z }),
            ParseResult::ParamSpaceVert(u, v, w) => param_space_verts.push(ParamSpaceVert { u: u, v: v, w: w }),
            ParseResult::Face(is) => faces.push(Face { vertex_indices: is }),
            ParseResult::Comment => (),
        }
    }
    Object {
        vertices: vertices,
        text_coords: text_coords,
        vertex_normals: vertex_normals,
        param_space_verts: param_space_verts,
        faces: faces,
    }
}

#[derive(PartialEq, PartialOrd, Debug)]
pub struct Vertex {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32, // w is optional in the file format, defaults to 1.0
}

#[derive(Debug)]
pub struct TextCoord {
    pub u: f32,
    pub v: f32,
    pub w: f32,
}

#[derive(Debug)]
pub struct VertexNormal {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Debug)]
pub struct ParamSpaceVert {
    pub u: f32,
    pub v: Option<f32>,
    pub w: Option<f32>,
}

#[derive(PartialEq, PartialOrd, Debug)]
pub struct FaceVertex {
    pub vertex_index: usize,
    pub text_coord_index: Option<usize>,
    pub vert_normal_index: Option<usize>,
}

#[derive(PartialEq, PartialOrd, Debug)]
pub struct Face {
    pub vertex_indices: Vec<FaceVertex>,
}

#[derive(Debug)]
pub struct Object {
    pub vertices: Vec<Vertex>,
    pub text_coords: Vec<TextCoord>,
    pub vertex_normals: Vec<VertexNormal>,
    pub param_space_verts: Vec<ParamSpaceVert>,
    pub faces: Vec<Face>,
}

// this should be built in??? be_f32 does not work?? this parses sci notation as well
fn parse_float(input: &[u8]) -> IResult<&[u8], f32> {
    let (i, name) = try_parse!(input,
                               recognize!(chain!(
                                   tag!("-")? ~
                                       take_while!(is_digit) ~
                                       tag!(".")? ~
                                       take_while!(is_digit)? ~
                                       opt!( chain!(
                                               tag!("e") ~
                                               tag!("-")? ~
                                               take_while!(is_digit)
                                               ,
                                               || {}
                                               )
                                           )
                                       ,
                                   || {}
                               )));
    let num: &str = from_utf8(name).unwrap();
    Done(i, num.parse::<f32>().unwrap())
}

fn parse_usize(input: &[u8]) -> IResult<&[u8], usize> {
    let (i, name) = try_parse!(input,
                               recognize!(chain!(
                                   tag!("-")? ~
                                       take_while!(is_digit)?,
                                   || {}
                               )));
    let num: &str = from_utf8(name).unwrap();
    Done(i, num.parse::<usize>().unwrap())
}

named!(pub comment <ParseResult>,
       chain!(
           alt!(tag!("#") | tag!("s") | tag!("g")) ~
           take_until!("\n")
           ,
           || { ParseResult::Comment }
           )
       );

named!(toplevel <Vec<ParseResult> >,
       many1!(obj_line)
       );

named!(obj_line <ParseResult>,
       chain!(
           ln: alt!( comment | vertex | text_coord | vertex_normal | param_space_vert | face ) ~
           newline
           ,
           || ( ln )
           )
       );

named!(vertex <ParseResult>,
       chain!(
           tag!("v") ~
           space ~
           x: parse_float ~
           space ~
           y: parse_float ~
           space ~
           z: parse_float ~
           w: opt!(chain!(
                   space ~
                   w: parse_float
                   ,
                   || { w }
                   )
               )
       ,
       || { ParseResult::Vertex(x, y, z, w.unwrap_or(1.0))  }
      )
      );

named!(text_coord <ParseResult>,
       chain!(
           tag!("vt") ~
           space ~
           u: parse_float ~
           space ~
           v: parse_float ~
           w: opt!(chain!(
                   space ~
                   w: parse_float
                   ,
                   || { w }
                   )
               )
       ,
       || { ParseResult::TextCoord(u, v, w.unwrap_or(0.0)) }
      )
      );


named!(vertex_normal <ParseResult>,
       chain!(
           tag!("vn") ~
           space ~
           x: parse_float ~
           space ~
           y: parse_float ~
           space ~
           z: parse_float
       ,
       || { ParseResult::VertexNormal(x, y, z) }
      )
      );

named!(param_space_vert <ParseResult>,
       chain!(
           tag!("vp") ~
           space ~
           u: parse_float ~
           v: opt!(chain!(
                   space ~
                   v: parse_float
                   ,
                   || { v }
                   )
               ) ~
           w: opt!(chain!(
                   space ~
                   w: parse_float
                   ,
                   || { w }
                   )
               )
       ,
       || { ParseResult::ParamSpaceVert(u, v, w) }
      )
      );

named!(face_vertex <FaceVertex>,
       chain!(
           vi: parse_usize ~
           tag!("/") ~
           tci: parse_usize ~
           tag!("/") ~
           vni: parse_usize
           ,
           || { FaceVertex {
               vertex_index: vi,
               text_coord_index: Some(tci),
               vert_normal_index: Some(vni),
           }
           }
           )
       );

named!(face <ParseResult>,
       chain!(
           tag!("f") ~
           space ~
           fs: separated_list!(space, face_vertex)
           ,
           || { ParseResult::Face(fs) }
           )
       );
