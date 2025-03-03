extern crate gl;
extern crate glfw;

use cgmath::{Deg, Matrix4, Point3, Vector3, perspective};
use glfw::{Context, Glfw, GlfwReceiver, PWindow, WindowEvent};

use crate::rendering::mesh::Mesh;
use crate::rendering::shader::Shader;
use crate::world::{block::Block, world::World};

pub struct App {
    pub glfw: Glfw,
    pub window: PWindow,
    pub events: GlfwReceiver<(f64, WindowEvent)>,
    pub shader: Shader,
    pub world: World,
    pub projection: Matrix4<f32>,
    pub view: Matrix4<f32>,
}

impl App {
    pub fn new(width: u32, height: u32, title: &str) -> Self {
        let mut glfw = glfw::init(glfw::fail_on_errors).expect("Failed to initialize GLFW");

        glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(
            glfw::OpenGlProfileHint::Core,
        ));

        let (mut window, events) = glfw
            .create_window(width, height, title, glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window.");

        window.make_current();
        window.set_key_polling(true);
        window.set_framebuffer_size_polling(true);

        gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

        let shader = Shader::new(
            "src/shaders/vertex_shader.glsl",
            "src/shaders/fragment_shader.glsl",
        );
        let cube_vertices: [f32; 108] = Block::get_cube_vertices();
        let mesh = Mesh::new(&cube_vertices);
        let world = World::new(mesh);

        let projection: Matrix4<f32> =
            perspective(Deg(60.0), width as f32 / height as f32, 0.1, 100.0);
        let view: Matrix4<f32> = Matrix4::look_at_rh(
            Point3::new(25.0, 25.0, 25.0), // Position camera further back diagonally
            Point3::new(8.0, 8.0, 8.0),    // Look at center of the chunk
            Vector3::new(0.0, 1.0, 0.0),   // Keep same up vector
        );

        App {
            glfw,
            window,
            events,
            shader,
            world,
            projection,
            view,
        }
    }
}
