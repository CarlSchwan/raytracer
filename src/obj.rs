use crate::error::Error;
use crate::shader::{
    ambient_shader::AmbientShader, diffuse_shader::DiffuseShader, get_phong,
    mirror_shader::MirrorShader, specular_shader::SpecularShader, Shader,
};
use crate::storage::collector::Collector;
use crate::world::triangle::Triangle;
use na::Vector3;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use wavefront_obj::mtl::Material;
use wavefront_obj::mtl::{parse as mtl_parse, Color, Illumination};
use wavefront_obj::obj::parse as obj_parse;
use wavefront_obj::obj::Primitive;

/// FileParser struct
/// Can parse obj and mtl wavefront files
/// After parsing multiple files with this struct, get the elements field
pub struct FileParser {
    pub elements: Collector,
    materials: HashMap<String, Material>,
}

impl FileParser {
    /// Create a FileParser to parse wavefront files (obj and mtl)
    pub fn new() -> Self {
        FileParser {
            elements: Collector::new(),
            materials: HashMap::new(),
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
            Err(Error::from(
                "File is not a wavefront obj file or a mnt file",
            ))
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

                            let shader = if let Some(name) = &geometry.material_name {
                                let mat = self.materials.get(name).expect("Material don't exist");
                                material_to_shader(mat)
                            } else {
                                Ok(get_phong(Vector3::new(0.0, 1.0, 0.0)))
                            }?;

                            self.elements
                                .add_bounded(Box::new(Triangle { a, b, c, shader }));
                        }
                        _ => (),
                    };
                }
            }
        }

        Ok(())
    }

    /// Parse a wavefront mnt file
    fn parse_mtl(&mut self, contents: String) -> Result<(), Error> {
        let material_set = mtl_parse(contents)?;
        for material in material_set.materials {
            let name = material.name.clone();
            self.materials.insert(name, material);
        }
        Ok(())
    }
}

fn color_to_vec(color: Color) -> Vector3<f64> {
    Vector3::new(color.r, color.g, color.b)
}

fn material_to_shader(material: &Material) -> Result<Box<Shader>, Error> {
    let diffuse_shader: Box<Shader> = Box::new(DiffuseShader {
        color: color_to_vec(material.color_diffuse),
    });
    let specular_shader = SpecularShader { alpha: 10.0 }; // TODO FIXME use material.color_diffuse
    let ambient_shader: Box<Shader> = Box::new(color_to_vec(material.color_ambient));
    match material.illumination {
        Illumination::Ambient => Ok(ambient_shader),
        Illumination::AmbientDiffuse => Ok(0.5 * diffuse_shader + 0.5 * ambient_shader),
        Illumination::AmbientDiffuseSpecular => {
            Ok(0.5 * diffuse_shader + specular_shader + 0.5 * ambient_shader)
        }
        Illumination::Reflection => Ok(Box::new(MirrorShader { initial_step: 1.0 })),
        _ => Err(Error::from("Illumination not yet supported")),
    }
}
