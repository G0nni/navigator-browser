# Navigator Browser - Roadmap

## 📋 Project Status

**Current Phase:** Phase 1 - Core Engine ✅ COMPLETED

The browser can currently:
- ✅ Fetch web pages over HTTPS
- ✅ Parse HTML5 documents into DOM
- ✅ Extract page metadata (titles, etc.)
- ✅ Display page structure in text mode
- ✅ Validate URLs and enforce security policies

---

## 🗺️ Development Phases

### Phase 1: Core Engine ✅ COMPLETED
**Goal:** Build the fundamental browser engine without visual rendering

- [x] Project setup with Cargo
- [x] Clean Architecture implementation (Domain/Application/Infrastructure)
- [x] HTML5 parser integration (html5ever)
- [x] HTTPS networking layer (reqwest + rustls)
- [x] DOM construction
- [x] Security layer (URL validation, HTTPS enforcement, XSS protection)
- [x] Database layer for bookmarks/history (SQLite)
- [x] Text-mode rendering for testing
- [x] Windows compilation support

**Deliverable:** A working browser that can fetch and parse real websites (Google, etc.)

---

### Phase 2: Visual Rendering 🚧 NEXT
**Goal:** Add graphical interface and basic rendering

#### 2.1 Window Management
- [ ] Integrate `winit` for cross-platform windowing
- [ ] Create main browser window
- [ ] Handle window events (resize, close, etc.)
- [ ] Basic keyboard shortcuts

#### 2.2 GPU Rendering
- [ ] Integrate `wgpu` for hardware-accelerated rendering
- [ ] Setup render pipeline
- [ ] Implement basic rectangle rendering
- [ ] Text rendering (using `wgpu-text` or `fontdue`)

#### 2.3 CSS Parser
- [ ] Integrate `cssparser` and `selectors`
- [ ] Parse CSS stylesheets
- [ ] Build CSS cascade
- [ ] Compute styles for DOM elements

#### 2.4 Layout Engine
- [ ] Implement box model
- [ ] Flow layout (block and inline)
- [ ] Positioning (static, relative, absolute)
- [ ] Flexbox layout (basic)
- [ ] Text wrapping and line breaking

#### 2.5 Painting
- [ ] Render text nodes
- [ ] Render backgrounds and borders
- [ ] Render images
- [ ] Handle colors and gradients
- [ ] Implement scrolling

**Estimated Time:** 4-6 weeks  
**Deliverable:** A browser that can display simple HTML pages visually

---

### Phase 3: UI/UX Features 📋 PLANNED
**Goal:** Make the browser usable for daily browsing

#### 3.1 Navigation UI
- [ ] URL address bar with autocomplete
- [ ] Back/Forward navigation buttons
- [ ] Reload button
- [ ] Home button
- [ ] Bookmarks bar

#### 3.2 Tab Management
- [ ] **Vertical tabs sidebar** (signature feature!)
- [ ] Tab creation/deletion
- [ ] Tab switching
- [ ] Tab dragging and reordering
- [ ] Tab groups/organization

#### 3.3 User Settings
- [ ] Settings panel
- [ ] Default search engine selection
- [ ] Privacy settings
- [ ] Theme customization (dark mode)
- [ ] Font size controls

#### 3.4 Bookmarks & History
- [ ] Bookmark management UI
- [ ] Bookmark folders
- [ ] History viewer
- [ ] Search history
- [ ] Clear browsing data

**Estimated Time:** 3-4 weeks  
**Deliverable:** A browser with complete UI for everyday use

---

### Phase 4: JavaScript Engine 📋 PLANNED
**Goal:** Add JavaScript execution capability

#### 4.1 JS Engine Integration
- [ ] Integrate `boa_engine` (Rust JS engine)
- [ ] Execute inline `<script>` tags
- [ ] Execute external scripts
- [ ] Console API implementation
- [ ] Error handling and reporting

#### 4.2 Web APIs
- [ ] DOM API (document, getElementById, etc.)
- [ ] Event handling (addEventListener, etc.)
- [ ] Timers (setTimeout, setInterval)
- [ ] Fetch API
- [ ] LocalStorage/SessionStorage
- [ ] Basic Web APIs subset

#### 4.3 Performance
- [ ] JS execution sandboxing
- [ ] Memory management
- [ ] Garbage collection tuning

**Estimated Time:** 6-8 weeks  
**Deliverable:** A browser that can run interactive websites

---

### Phase 5: Advanced Features 📋 FUTURE
**Goal:** Add advanced browsing capabilities

#### 5.1 Developer Tools
- [ ] Inspector panel
- [ ] Console
- [ ] Network monitor
- [ ] Performance profiler
- [ ] DOM tree viewer

#### 5.2 Extensions System
- [ ] Extension manifest format
- [ ] Extension API
- [ ] Extension marketplace
- [ ] Ad blocker extension

#### 5.3 Performance Optimization
- [ ] Multi-process architecture
- [ ] GPU acceleration improvements
- [ ] Memory optimization
- [ ] Network caching
- [ ] Lazy loading

#### 5.4 Standards Compliance
- [ ] HTML5 compliance tests
- [ ] CSS3 support expansion
- [ ] ES2015+ JavaScript features
- [ ] WebAssembly support

**Estimated Time:** 12+ weeks  
**Deliverable:** A feature-complete modern browser

---

## 🎯 Immediate Next Steps

**To start Phase 2, you need to:**

1. **Install dependencies:**
   ```bash
   # Already in Cargo.toml, just need to add:
   winit = "0.30"
   wgpu = "22.0"
   ```

2. **Create window module:**
   ```bash
   cargo run --example simple_window
   ```

3. **Test basic rendering:**
   - Open a window
   - Clear screen with a color
   - Render "Hello World" text

4. **Integrate with existing engine:**
   - Load HTML in background
   - Display in window
   - Handle user input

**First milestone:** Display Google.com homepage visually (even if ugly!)

---

## 🤔 Technical Decisions

### Why not use existing engines?

| Engine | Why NOT used |
|--------|--------------|
| **Chromium (Blink)** | Too heavy, not a learning experience |
| **Gecko (Firefox)** | C++ complexity, huge codebase |
| **WebKit** | Not available on Windows easily |
| **Servo** | Takes 30-60min to compile, still experimental |

**Our approach:** Build component by component using Rust libraries.

### Technology Stack

| Component | Library | Why? |
|-----------|---------|------|
| HTML Parser | `html5ever` | Mozilla's parser, standards-compliant |
| CSS Parser | `cssparser` | Used by Servo, battle-tested |
| Networking | `reqwest` + `rustls` | Async, secure, pure Rust |
| Rendering | `wgpu` | Cross-platform GPU access |
| Windowing | `winit` | Cross-platform, Rust-native |
| JS Engine | `boa_engine` | Pure Rust, modern JS |
| Database | `sqlx` + SQLite | Type-safe, async |

---

## 📚 Learning Resources

### Browser Engine Development
- [Let's build a browser engine!](https://limpet.net/mbrubeck/2014/08/08/toy-layout-engine-1.html) - Matt Brubeck
- [Browser from Scratch](https://viethung.space/blog/2020/05/29/Browser-from-Scratch-Introduction/) - Viet Hung
- [Servo Architecture](https://github.com/servo/servo/wiki/Design) - Mozilla

### Rust Graphics
- [Learn Wgpu](https://sotrh.github.io/learn-wgpu/) - wgpu tutorial
- [winit examples](https://github.com/rust-windowing/winit/tree/master/examples)

### Web Standards
- [HTML5 Spec](https://html.spec.whatwg.org/)
- [CSS Spec](https://www.w3.org/Style/CSS/)
- [MDN Web Docs](https://developer.mozilla.org/)

---

## 🚀 Getting Started with Phase 2

Want to start adding visual rendering? Here's what to do:

```bash
# 1. Add winit and wgpu to Cargo.toml (already done)
# 2. Create a simple window example
cargo run --example simple_window

# 3. Test it works
# You should see a blank window appear
```

Then we'll integrate it with the HTML parser to display real pages!

---

## 💡 Ideas for Unique Features

Things that could make Navigator special:

- 🎨 **Vertical tabs** (main UX differentiator)
- 🔒 **Privacy-first** by default (no telemetry)
- ⚡ **Blazing fast** Rust performance
- 🛡️ **Security hardened** from the ground up
- 🎯 **Minimalist UI** focused on content
- 🔧 **Developer-friendly** built-in tools
- 📦 **Lightweight** compared to Chrome/Firefox

---

**Last Updated:** 2025-10-07  
**Current Version:** 0.1.0 (Core Engine)  
**Next Version:** 0.2.0 (Visual Rendering)
