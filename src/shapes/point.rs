use std::f32;

use glium;

use {Screen, ScreenType};
use errors::ProcessingErr;

use shapes::{Shape, ShapeVertex, IndexType, load_colors};

/// For the purposes of `processing-rs`, a point is basically just a pixel, specifed
/// by a single position.
pub struct Point {
    fill_buffer: glium::vertex::VertexBuffer<ShapeVertex>,
    stroke_buffer: glium::vertex::VertexBuffer<ShapeVertex>,
    fill_index_buffer: IndexType,
    stroke_index_buffer: IndexType,
}

impl Shape for Point {
    fn fill_buffer(&self) -> Box<&glium::vertex::VertexBuffer<ShapeVertex>> {
        Box::new(&self.fill_buffer)
    }

    fn stroke_buffer(&self) -> Box<&glium::vertex::VertexBuffer<ShapeVertex>> {
        Box::new(&self.stroke_buffer)
    }

    fn fill_indices(&self) -> Box<&IndexType> {
        Box::new(&self.fill_index_buffer)
    }

    fn stroke_indices(&self) -> Box<&IndexType> {
        Box::new(&self.stroke_index_buffer)
    }

    fn get_texture(&self) -> Option<Box<&glium::texture::Texture2d>> {
        None
    }
}

impl Point {
	/// Create a new Point to be drawn later. It is specified by its position
	/// (xi, yi, zi).
    #[inline]
    pub fn new(screen: &mut Screen, xi: &[f64], yi: &[f64], zi: &[f64]) -> Result<Self, ProcessingErr> {
        let mut x: Vec<f64> = xi.iter().map(|&v| v).collect::<Vec<f64>>();
        let mut y: Vec<f64> = yi.iter().map(|&v| v).collect::<Vec<f64>>();
        let z: Vec<f64> = zi.iter().map(|&v| v).collect::<Vec<f64>>();
        if screen.preserve_aspect_ratio {
            if screen.aspect_ratio > 1f32 {
                for i in 0..x.len() {
                    x[i] = x[i] / screen.aspect_ratio as f64;
                }
            } else {
                for i in 0..x.len() {
                    y[i] = y[i] * screen.aspect_ratio as f64;
                }
            }
        }

        if screen.stroke_stuff {
            let eps = f32::EPSILON;
            let mut shape = vec![];
            for (i, _) in x.iter().enumerate() {
                let vertex = ShapeVertex {
                    position: [
                        x[i] as f32,
                        y[i] as f32,
                        if z[i] == 0.0 {
                            eps * i as f32
                        } else {
                            z[i] as f32
                        },
                    ],
                    color: [0.0, 0.0, 0.0, 0.0],
                    texcoord: [0f32, 0.],
                };
                shape.push(vertex);
            }

            load_colors(&mut shape, &screen.stroke_col);
            let fill_shape_buffer = match screen.display {
                ScreenType::Window(ref d) => glium::VertexBuffer::new(d, &shape)
                	.map_err(|e| ProcessingErr::VBNoCreate(e))?,
                ScreenType::Headless(ref d) => glium::VertexBuffer::new(d, &shape)
                	.map_err(|e| ProcessingErr::VBNoCreate(e))?,
            };
            let stroke_shape_buffer = match screen.display {
                ScreenType::Window(ref d) => glium::VertexBuffer::new(d, &shape)
                	.map_err(|e| ProcessingErr::VBNoCreate(e))?,
                ScreenType::Headless(ref d) => glium::VertexBuffer::new(d, &shape)
                	.map_err(|e| ProcessingErr::VBNoCreate(e))?,
            };

            // screen.draw_params = glium::draw_parameters::DrawParameters {
            // smooth: None,
            // ..screen.draw_params.clone()
            // };

            // screen.drew_points = true;

            return Ok(Point {
                fill_buffer: fill_shape_buffer,
                stroke_buffer: stroke_shape_buffer,
                fill_index_buffer: IndexType::NoBuffer {
                    ind: glium::index::NoIndices(glium::index::PrimitiveType::Points),
                },
                stroke_index_buffer: IndexType::NoBuffer {
                    ind: glium::index::NoIndices(glium::index::PrimitiveType::Points),
                },
            });
        }

        return Ok(Point {
            fill_buffer: match screen.display {
                ScreenType::Window(ref d) => glium::VertexBuffer::new(d, &vec![])
                	.map_err(|e| ProcessingErr::VBNoCreate(e))?,
                ScreenType::Headless(ref d) => glium::VertexBuffer::new(d, &vec![])
                	.map_err(|e| ProcessingErr::VBNoCreate(e))?,
            },
            stroke_buffer: match screen.display {
                ScreenType::Window(ref d) => glium::VertexBuffer::new(d, &vec![])
                	.map_err(|e| ProcessingErr::VBNoCreate(e))?,
                ScreenType::Headless(ref d) => glium::VertexBuffer::new(d, &vec![])
                	.map_err(|e| ProcessingErr::VBNoCreate(e))?,
            },
            fill_index_buffer: IndexType::NoBuffer {
                ind: glium::index::NoIndices(glium::index::PrimitiveType::Points),
            },
            stroke_index_buffer: IndexType::NoBuffer {
                ind: glium::index::NoIndices(glium::index::PrimitiveType::Points),
            },
        });
    }
}
