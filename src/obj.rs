use std::fs::File;
use std::io::Read;
use crate::world;
use crate::shader::get_phong;
use crate::world::triangle::Triangle;
use wavefront_obj::obj::Primitive;
use wavefront_obj::obj::parse as obj_parse;
use wavefront_obj::mtl::parse as mtl_parse;
use wavefront_obj::mtl::Material;
use crate::error::Error;
use na::Vector3;
use image::Rgba;
use image::Pixel;

/// FileParser struct
/// Can parse obj and mtl wavefront files
/// After parsing multiple files with this struct, get the elements field
pub struct FileParser {
    pub elements: std::vec::Vec<std::boxed::Box<world::Interceptable>>,
    materials: Vec<Material>,
}

impl FileParser {
    /// Create a FileParser to parse wavefront files (obj and mtl)
    pub fn new() -> Self {
        FileParser {
            elements: Vec::new(),
            materials: Vec::new(),
        }
    }

    /// Parse a wavefront obj or a wavefront mtl file
    /// Decide the parser to use in function of the extension
    pub fn parse(&mut self, path: String) -> Result<(), Error> {
        // read file content to String
        let mut file = File::open(&path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        // choose the parser
        if path.contains(".obj") {
            self.parse_obj(contents)?;
            Ok(())
        } else if path.contains(".mtl") {
            self.parse_mtl(contents)?;
            Ok(())
        } else {
            Err(Error::from("File is not a wavefront obj file or a mnt file"))
        }
    }

    /// Parse a wavefront obj file (only supports a subset from the subset that the crate
    /// wavefront_obj support.
    fn parse_obj(&mut self, contents: String) -> Result<(), Error> {
        let obj_set = obj_parse(contents)?;

        for object in obj_set.objects {
            for geometry in object.geometry {
                for shape in geometry.shapes {
                    let primitive = &shape.primitive;
                    match primitive {
                        Primitive::Triangle(u, v, w) => {
                            let vertices_a = object.vertices[u.0];
                            let a = Vector3::new(vertices_a.x, vertices_a.y, vertices_a.z);

                            let vertices_b = object.vertices[v.0];
                            let b = Vector3::new(vertices_b.x, vertices_b.y, vertices_b.z);

                            let vertices_c = object.vertices[w.0];
                            let c = Vector3::new(vertices_c.x, vertices_c.y, vertices_c.z);

                            self.elements.push(Box::new(Triangle {
                                a,
                                b,
                                c,
                                shader: get_phong(Vector3::new(0.0, 1.0, 0.0)
                            )}));
                        }
                        _ => (),
                    };
                }
            }
        }

        Ok(())
    }

    /// Parse a wavefront mnt file
    pub fn parse_mtl(&mut self, contents: String) -> Result<(), Error> {
        let mut material_set = mtl_parse(contents)?;
        self.materials.append(&mut material_set.materials);
        Ok(())
    }
}
