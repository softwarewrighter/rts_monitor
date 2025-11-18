# RTS Monitor Wiki

Welcome to the **Distributed System Monitor** (RTS Monitor) documentation. This project provides a web-based visual interface for managing distributed LLMs and Model Context Protocol (MCP) servers across a network.

## Project Overview

RTS Monitor is a Rust-based WebAssembly application that visualizes and manages distributed AI/ML infrastructure. The system presents network topology as interactive "server islands" connected by bridges in a star configuration, with a retro terminal aesthetic.

### Key Features

- **Visual Network Topology**: Real-time visualization of distributed systems
- **Process Management**: Deploy, start, stop, and restart LLM and MCP server processes
- **Resource Monitoring**: Track CPU, memory, GPU, and network usage
- **Interactive UI**: Mouse drag-to-pan, keyboard navigation, and click-based interaction
- **Rust-First Architecture**: All logic in Rust/WASM, minimal JavaScript

## Documentation Structure

### Architecture Documentation

- **[[Architecture Overview]]** - High-level system architecture with block diagrams
- **[[Technology Stack]]** - Detailed breakdown of technologies and tools used
- **[[Component Details]]** - Deep dive into core components

### Component Documentation

- **[[UI Components]]** - User interface elements and their responsibilities
- **[[State Management]]** - MapState, DragState, and thread-local storage patterns
- **[[Event Handling]]** - Mouse, keyboard, and UI event processing in Rust

### Workflow Documentation

- **[[Interaction Flows]]** - Sequence diagrams for common user interactions
- **[[Build and Deploy]]** - Development workflow and deployment process

### Development Documentation

- **[[Development Guide]]** - Getting started, coding standards, and best practices
- **[[Testing Guide]]** - Test strategy, running tests, and adding new tests

## Quick Links

- [Project README](../blob/main/README.md) - Main project documentation
- [CLAUDE.md](../blob/main/CLAUDE.md) - AI assistant development guidance
- [Project Overview](../blob/main/docs/overview.md) - Detailed project status
- [Source Code](../tree/main/src) - Browse the Rust source

## Project Status

**Version**: 0.1.0
**Status**: Active Development
**Last Updated**: 2025-11-18

### Current Status

- ✅ Core WASM module with interaction handlers
- ✅ Complete UI with inline CSS
- ✅ Mouse drag-to-pan and keyboard navigation
- ✅ 10 comprehensive tests passing
- ✅ 80% JavaScript reduction (Rust-first approach)
- ✅ Clean code (all clippy warnings resolved)

### Technology Highlights

- **Language**: Rust (compiled to WebAssembly)
- **Framework**: wasm-bindgen + web-sys
- **UI**: HTML5/CSS3 with inline styling, SVG graphics
- **Build**: wasm-pack
- **Testing**: Cargo test (10 passing tests)

## Contributing

When working on this project:
- Keep all Rust code in `src/lib.rs`
- Follow Rust-first philosophy (minimal JavaScript)
- Run `cargo clippy` and `cargo fmt` before committing
- Add tests for new functionality
- Update wiki documentation for significant changes

## Getting Started

```bash
# Build the project
./scripts/build.sh

# Run development server
./scripts/run.sh

# Run tests
cargo test

# Check code quality
cargo clippy
cargo fmt
```

---

*For detailed architectural information, see [[Architecture Overview]]. For development instructions, see [[Development Guide]].*
