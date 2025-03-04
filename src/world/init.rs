// In src/world/init.rs
extern crate gl;
extern crate glfw;

use crate::config::Config;
use crate::player::camera::Camera;
use crate::rendering::mesh::Mesh;
use crate::rendering::shader::Shader;
use crate::world::{block::Block, world::World};
use cgmath::{Deg, Matrix4, Point3, perspective};
use glfw::{Context, CursorMode, Glfw, GlfwReceiver, PWindow, WindowEvent};

/// Represents the main application state, including window, camera, shader, and world.
pub struct App {
    pub glfw: Glfw,
    pub window: PWindow,
    pub events: GlfwReceiver<(f64, WindowEvent)>,
    pub shader: Shader,
    pub world: World,
    pub projection: Matrix4<f32>,
    pub view: Matrix4<f32>,
    pub camera: Camera,
}

impl App {
    /// Creates a new `App` instance with the given configuration.
    ///
    /// # Arguments
    ///
    /// * `config` - A reference to the game configuration.
    ///
    /// # Returns
    ///
    /// A new `App` instance.
    pub fn new(config: &Config) -> Self {
        let mut glfw = glfw::init(glfw::fail_on_errors).expect("Failed to initialize GLFW");

        glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(
            glfw::OpenGlProfileHint::Core,
        ));

        // Create initial window
        let (mut window, events) = glfw
            .create_window(
                config.window.width,
                config.window.height,
                &config.window.title,
                glfw::WindowMode::Windowed,
            )
            .expect("Failed to create GLFW window.");

        // Apply fullscreen if configured
        if config.window.fullscreen {
            glfw.with_primary_monitor(|_, m| {
                if let Some(monitor) = m {
                    if let Some(video_mode) = monitor.get_video_mode() {
                        window.set_monitor(
                            glfw::WindowMode::FullScreen(monitor),
                            0,
                            0,
                            video_mode.width,
                            video_mode.height,
                            Some(video_mode.refresh_rate),
                        );
                    }
                }
            });
        }

        window.make_current();
        window.set_key_polling(true);
        window.set_framebuffer_size_polling(true);
        window.set_cursor_pos_polling(true);

        if config.controls.cursor_locked {
            window.set_cursor_mode(CursorMode::Disabled);
        }

        gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

        let shader = Shader::new(
            "src/shaders/vertex_shader.glsl",
            "src/shaders/fragment_shader.glsl",
        );
        let cube_vertices: [f32; 108] = Block::get_cube_vertices();
        let mesh = Mesh::new(&cube_vertices);
        let world = World::new(mesh);

        // Get the current framebuffer size for projection matrix
        let (width, height) = window.get_framebuffer_size();
        let projection: Matrix4<f32> = perspective(
            Deg(config.camera.fov),
            width as f32 / height as f32,
            config.camera.near_plane,
            config.camera.far_plane,
        );

        // Initialize camera at config position
        let camera = Camera::new(Point3::new(
            config.camera.position_x,
            config.camera.position_y,
            config.camera.position_z,
        ));
        let view = camera.get_view_matrix();

        App {
            glfw,
            window,
            events,
            shader,
            world,
            projection,
            view,
            camera,
        }
    }

    /// Updates the view matrix based on the current camera orientation.
    pub fn update_view_matrix(&mut self) {
        self.view = self.camera.get_view_matrix();
    }
}
