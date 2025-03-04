# Cubix

Cubix is a Minecraft clone game written in Rust. It features a 3D world with player movement, camera control, and basic rendering.

## Features

- 3D rendering using OpenGL
- Player movement with walking and flying modes
- Camera control with mouse input
- Configurable settings via `config.toml`

## Getting Started

### Prerequisites

- Rust (latest stable version)
- Cargo (Rust package manager)
- OpenGL
- GLFW

### Installation

1. Clone the repository:

    ```sh
    git clone https://github.com/yourusername/cubix.git
    cd cubix
    ```

2. Build the project:

    ```sh
    cargo build --release
    ```

3. Run the project:

    ```sh
    cargo run --release
    ```

### Configuration

The game configuration is stored in `config.toml` and `src/config.rs`. You can modify this file to change various settings such as window size, camera sensitivity, and physics parameters. If you modify `src/config.rs` ensure to remove the `config.toml` file for changes to update.

### Controls

- `W` - Move forward
- `S` - Move backward
- `A` - Strafe left
- `D` - Strafe right
- `Space` - Jump (or move up in fly mode)
- `Left Shift` - Move down in fly mode
- `F` - Toggle fly mode
- `Left Control` - Sprint
- `Escape` - Exit the game
- `F11` - Toggle fullscreen

### Code Structure

- `src/main.rs`: The main entry point of the application.
- `src/config.rs`: Configuration management.
- `src/events.rs`: Event handling.
- `src/player/`: Player-related functionality (camera and input).
- `src/rendering/`: Rendering-related functionality (mesh and shader).
- `src/world/`: World-related functionality.

### Contributing

Contributions are welcome! Please open an issue or submit a pull request.

### License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
