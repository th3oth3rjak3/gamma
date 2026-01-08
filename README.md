# Gamma - Minimal Rust 2D Game Engine

This project is an effort to build a minimal, performant 2D game engine from first principles using Rust. The primary goal is to create a lightweight, modular foundation suitable for small games, with WebAssembly (WASM) as a first-class deployment target.

This engine is intentionally built from a curated set of low-level libraries, avoiding the complexity and opinions of larger, "batteries-included" frameworks. The focus is on control, performance, and a deep understanding of the rendering pipeline.

## Core Philosophy

- **First Principles:** Understand and implement the core components of a game engine directly, rather than relying on a monolithic framework.
- **Performance & Size:** Strive for low memory usage, fast execution, and small binary sizes, which are critical for web-based games.
- **WASM First:** Architect the engine from the ground up to compile and run smoothly in a web browser via WebAssembly.
- **Control:** Retain full control over the render loop, state management, and asset pipeline.
- **Modular Stack:** Assemble the engine from a set of focused, high-quality crates, each responsible for one specific task.

## Target Features

The initial goal for the engine is to support the essential features required for simple 2D games:

- [ ] **Window & Input Management:** Create a window, handle user input (keyboard and mouse), and manage the main event loop.
- [ ] **2D Sprite Rendering:** Load PNG images and render them as sprites on the screen.
- [ ] **Text Rendering:** Load TrueType (`.ttf`) or OpenType (`.otf`) fonts and render text.
- [ ] **Audio Playback:** Load and play `.wav` sound files for sound effects and music.
- [ ] **Cross-Platform Compilation:** Support for major desktop platforms (Windows, macOS, Linux) and WebAssembly.

## Technology Stack

The engine is built on a carefully selected stack of modern, well-maintained Rust libraries that are ideal for this from-scratch approach.

| Role                  | Crate         | Description                                                                                                                                                                           |
| :-------------------- | :------------ | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| **Windowing & Input** | `winit`       | The de-facto standard for creating windows and handling the event loop in a platform-agnostic way. It has excellent support for both desktop and web targets.                         |
| **Graphics API**      | `wgpu`        | A modern, safe graphics API based on the WebGPU standard. It provides a portable interface over Vulkan, Metal, DirectX 12, and WebGL, making our rendering code truly cross-platform. |
| **Image Loading**     | `image`       | A robust and feature-rich library for decoding various image formats, including the PNG files we need for sprites.                                                                    |
| **Font Rendering**    | `glyph_brush` | A powerful library for handling the complexities of font parsing, glyph rasterization, and generating vertex data for efficient text rendering with `wgpu`.                           |
| **Audio**             | `kira`        | A modern game audio library designed for performance and flexibility. It handles the audio thread, sound loading, and playback, with first-class support for WASM.                    |

## Getting Started

_(This section is a placeholder for future build and usage instructions.)_

1.  **Prerequisites:** Install the Rust toolchain and platform-specific dependencies for `wgpu`.
2.  **Building for Native:**
    ```bash
    cargo run --release
    ```
3.  **Building for WebAssembly:**
    ```bash
    wasm-pack build --target web
    # Serve the generated files from a local web server
    ```
