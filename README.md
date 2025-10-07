# Navigator - Custom Web Browser

A high-performance web browser built from scratch in Rust with custom HTML parsing engine (no Chromium, no Gecko).

## 🚀 Features

- ✅ **Custom HTML Parser** - Uses `html5ever` for standards-compliant HTML5 parsing
- ✅ **HTTPS Support** - Secure connections with `rustls` and certificate validation
- ✅ **DOM Construction** - Full Document Object Model building
- ✅ **Security First** - URL validation, XSS protection, HTTPS enforcement
- ✅ **Clean Architecture** - Hexagonal architecture with Domain-Driven Design
- ⚠️ **JavaScript Engine** - Coming soon (boa_engine)
- ⚠️ **CSS Rendering** - Coming soon
- ⚠️ **Visual Rendering** - Coming soon (wgpu + winit)
- ⚠️ **Vertical Tabs** - Coming soon

## 🏗️ Architecture

Navigator follows Clean Architecture principles:

- **Domain Layer** - Core business logic, entities, and interfaces
- **Application Layer** - Use cases and application state
- **Infrastructure Layer** - External dependencies (HTML parser, network, database)

## 🛠️ Technologies

- **Language**: Rust
- **HTML Parser**: html5ever
- **Networking**: reqwest + rustls
- **Database**: SQLite with sqlx
- **Async Runtime**: tokio

## 📦 Installation

### Prerequisites

- Rust (latest stable)
- Windows: Visual Studio Build Tools with C++ support

### Build

```bash
cargo build --release
```

### Run

```bash
cargo run
```

## 🎯 Current Status

**Phase 1: Core Engine (COMPLETED ✅)**
- HTML parsing and DOM construction
- Network stack with HTTPS
- Security layer

**Phase 2: Visual Rendering (In Progress 🚧)**
- Window management (winit)
- GPU rendering (wgpu)
- CSS parsing and layout engine

**Phase 3: Features (Planned 📋)**
- JavaScript engine (boa)
- Bookmarks and history
- Extensions support
- Developer tools

## 🧪 Testing

Currently the browser works in text mode. It can:

1. Fetch web pages over HTTPS
2. Parse HTML into DOM
3. Extract page titles
4. Display page structure in text format

Example output:
```
Page Title: Google
URL: https://www.google.com/
<html>
  <head>
    <title>
      Google
  <body>
    ...
```

## 🤝 Contributing

This is a personal learning project. Feel free to fork and experiment!

## 📝 License

MIT OR Apache-2.0

## 🎓 Learning Resources

Building a browser from scratch resources:
- [Let's build a browser engine!](https://limpet.net/mbrubeck/2014/08/08/toy-layout-engine-1.html)
- [html5ever documentation](https://docs.rs/html5ever/)
- [Servo browser engine](https://servo.org/)

---

**Note**: This is a custom browser engine built from scratch for educational purposes. It does NOT use Chromium, Gecko, or any existing browser engine.
