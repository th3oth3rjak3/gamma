# Gamma Engine: MVP Execution Plan

## Guiding Principles

This plan is designed to build a minimal viable engine (MVP) centered around the proposed API. Our sole focus is to enable a simple, fast, 2D game like "Timber" that uses PNGs, fonts, and WAV sounds.

- **API First:** The entire plan is structured to realize this target API:
  ```rust
  gamma::run(initial_state)
      .update(update_function)
      .draw(draw_function);
  ```
- **MVP Focus:** We will implement the absolute minimum needed for a playable game. Optimizations and advanced features will wait.
- **Immediate Feedback:** Each milestone will result in a visible or interactive change, tested via example files.

---

### **Milestone 1: The Game Loop Runner**

- **Goal:** Implement the core `run(state).update(update).draw(draw)` builder and prove it can execute a user's functions in a loop.
- **API to Build:**
  - `gamma::run<S>(state: S) -> GameLoopBuilder<S>`
  - `GameLoopBuilder::update(self, func: fn(&mut GammaContext, &mut S))`
  - `GameLoopBuilder::draw(self, func: fn(&mut GammaContext, &mut S))`
- **Tasks:**
  1.  Define a placeholder `GammaContext` struct (it can be empty for now).
  2.  Create the `GameLoopBuilder` struct and its methods.
  3.  Implement the `run` function to open a `winit` window and start the event loop.
  4.  Inside the event loop, call the user-provided `update` and `draw` functions on every frame.
- **Verification:** The example code runs, opens a blank window, and `println!` statements inside the `update` and `draw` functions print to the console continuously. This proves the core loop architecture works before any graphics are involved.

---

### **Milestone 2: The Graphics Context (Colored Background)**

- **Goal:** Initialize `wgpu` and empower the `draw` function to clear the screen to a color.
- **API to Build:**
  - Flesh out `GammaContext` to hold the `wgpu` rendering state (`Device`, `Queue`, `Surface`, etc.).
- **Tasks:**
  1.  Integrate `wgpu` initialization into the `run` function.
  2.  Populate `GammaContext` with the necessary `wgpu` components.
  3.  Implement the render logic inside the main loop that clears the screen using `wgpu` before calling the user's `draw` function.
- **Verification:** The window from Milestone 1 now has a solid, non-white background color.

---

### **Milestone 3: Drawing Sprites**

- **Goal:** Enable the `draw` function to render PNG images.
- **API to Build:**
  - `GammaContext::load_texture(&mut self, path: &str) -> TextureId`
  - `GammaContext::draw_sprite(&mut self, id: TextureId, x: f32, y: f32)`
- **Tasks:**
  1.  Create a simple `TextureId` type (e.g., a `usize` or a struct).
  2.  Add a texture manager inside `GammaContext` (e.g., a `Vec<wgpu::Texture>`).
  3.  Implement `load_texture` to load a PNG with the `image` crate and create a `wgpu::Texture`.
  4.  Implement `draw_sprite` to issue a `wgpu` draw call for a textured quad at a given position. This will hide the complexity of pipelines and shaders from the user.
- **Verification:** The user's `draw` function can now load a texture and call `ctx.draw_sprite(...)`, making an image appear on screen.

---

### **Milestone 4: Input Handling**

- **Goal:** Enable the `update` function to react to keyboard and mouse input.
- **API to Build:**
  - `GammaContext` will contain a public `input` field: `ctx.input`.
  - `InputContext::is_key_down(&self, key: KeyCode) -> bool`
- **Tasks:**
  1.  Create an `InputContext` struct to store the current state of all keys.
  2.  Update the `InputContext` from within the `winit` event loop based on keyboard events.
  3.  Pass an immutable reference of the `InputContext` to the user's `update` function via the `GammaContext`.
- **Verification:** The `update` function can check `ctx.input.is_key_down(...)` to move the sprite from Milestone 3.

---

### **Milestone 5: Audio and Text**

- **Goal:** Add the final MVP features: rendering text and playing sounds.
- **API to Build:**
  - `GammaContext::draw_text(&mut self, text: &str, x: f32, y: f32)`
  - `GammaContext::load_sound(&mut self, path: &str) -> SoundId`
  - `GammaContext::play_sound(&mut self, id: SoundId)`
- **Tasks:**
  1.  **Text:** Integrate `wgpu_glyph` into the `GammaContext` and expose a simple `draw_text` method.
  2.  **Audio:** Integrate `kira` into the `GammaContext`. Add methods to load `.wav` files and play them on demand.
- **Verification:** The `draw` function can display a score. The `update` function can call `ctx.play_sound(...)` when a key is pressed.

---

### **Milestone 6: Build the Game & Deploy to WASM**

- **Goal:** Prove the engine is complete by building a "Timber-like" game and deploying it to the web.
- **Tasks:**
  1.  **Game Logic:** In a single example file (`examples/timber_game.rs`), use all the engine features built so far to create a complete game loop (player movement, spawning targets, collision checks, score, sounds).
  2.  **WASM Setup:** Install `trunk` and create a basic `index.html` with a `<canvas>`.
  3.  **WASM Entry Point:** Add a small `#[wasm_bindgen(start)]` function in `lib.rs` that calls your main `run` function, configured to attach to the HTML canvas.
- **Verification:**
  - `cargo run --example timber_game` runs a complete, playable game on the desktop.
  - `trunk serve` runs the exact same game in a web browser.

---

## Post-MVP Enhancements

Once the MVP is complete and a playable game exists, we can consider these enhancements:

- **Performance:** Implement sprite batching to draw many sprites in a single draw call.
- **API Polish:** Add methods for sprite rotation, scaling, and color tinting.
- **Features:** Add a simple camera system for scrolling, or helpers for drawing basic shapes (rectangles, circles).
- **DX:** Implement asset hot-reloading for faster iteration.
