extern crate gl;
use gl::types::*;

use std::ptr;

#[derive(Clone)]
pub struct Mesh {
    vao: GLuint,
    vertex_count: i32,
}

impl Mesh {
    pub fn new(vertices: &[f32]) -> Self {
        let mut vao: GLuint = 0;
        let mut vbo: GLuint = 0;
        let vertex_count = (vertices.len() / 3) as i32;

        unsafe {
            // Create Vertex Array Object
            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);

            // Create a Vertex Buffer Object and copy the vertex data to it
            gl::GenBuffers(1, &mut vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * size_of::<GLfloat>()) as GLsizeiptr,
                vertices.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );

            // Specify the layout of the vertex data
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE as GLboolean,
                (3 * size_of::<GLfloat>()) as GLint,
                ptr::null(),
            );

            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        }

        Mesh { vao, vertex_count }
    }

    pub fn draw(&self) {
        unsafe {
            gl::BindVertexArray(self.vao);
            gl::DrawArrays(gl::TRIANGLES, 0, self.vertex_count);
            gl::BindVertexArray(0);
        }
    }
}
