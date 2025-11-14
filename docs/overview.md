# Distributed System Monitor - Project Overview

**Last Updated:** 2025-11-14 (timestamp: 2025-11-14T22:33:21Z)

## Project Purpose

This project is a web-based Distributed System Monitor designed to provide visual management and monitoring of distributed Large Language Models (LLMs) and Model Context Protocol (MCP) servers across a network of servers and workstations. The system presents the network topology as an interactive map featuring "server islands" connected by network bridges in a star configuration, with a retro terminal aesthetic (green-on-black theme).

### Key Features

- **Visual Network Topology**: Real-time visualization of distributed systems as interconnected server islands
- **Process Management**: Deploy, start, stop, and restart LLM and MCP server processes
- **Resource Monitoring**: Track CPU, memory, GPU, and network usage across all nodes
- **MCP Server Management**: Discover and configure Model Context Protocol servers
- **Alert System**: Monitor system health with configurable thresholds and notifications
- **Interactive UI**: Click-based interaction with servers, processes, and network elements

### Target Use Case

Managing distributed AI/ML infrastructure where multiple LLM instances and MCP servers need to be coordinated across various physical or virtual machines, with emphasis on:
- Resource allocation and optimization
- Process lifecycle management
- Network topology visualization
- Real-time performance monitoring

## Technology Stack

- **Backend/Logic**: Rust compiled to WebAssembly (WASM)
- **WASM Bindings**: `wasm-bindgen` for JavaScript interop
- **Browser APIs**: `web-sys` for DOM and console access
- **Frontend**: HTML5/CSS3 with inline styling, minimal JavaScript
- **Graphics**: SVG for map and minimap rendering
- **Build Tool**: `wasm-pack` for WASM compilation
- **Dev Server**: `basic-http-server` for local development

## Current Status

### Completed Features

- ✅ Core WASM module with interaction handlers (`src/lib.rs`)
- ✅ Complete UI layout with inline CSS in `index.html`
- ✅ Interactive map with click detection for 6 server nodes and central router
- ✅ Resource panel showing system-wide metrics
- ✅ Server management menu for process deployment and control
- ✅ MCP configuration menu for server discovery and protocol settings
- ✅ Alert console for notifications
- ✅ Minimap with viewport indicator
- ✅ Build automation scripts (`scripts/build.sh`, `scripts/run.sh`)
- ✅ Clean code with all clippy warnings resolved
- ✅ Proper code formatting with `cargo fmt`
- ✅ Comprehensive testing (10 tests passing)
- ✅ **Rust-focused refactoring**: 80% JavaScript reduction
- ✅ MapState management in Rust with viewport controls
- ✅ Mouse drag-to-pan functionality in Rust
- ✅ Keyboard navigation (Arrow keys + WASD) in Rust
- ✅ Minimap click-to-center in Rust
- ✅ DOM manipulation via web-sys
- ✅ MIT License with copyright attribution
- ✅ Footer with copyright and links to LICENSE/GitHub

### Known Limitations

- **Mock Data**: All displayed metrics and statuses are hardcoded demonstration data
- **No Real Backend**: No actual connection to real servers or processes
- **Static Topology**: Network layout is fixed; cannot add/remove servers dynamically
- **No Authentication**: No security layer for production deployment
- **No Data Persistence**: No database or storage mechanism

### Current Version

- **Version**: 0.1.0 (initial development)
- **Build Status**: Compiles successfully with `wasm-pack build --target web`
- **Test Status**: 10 tests passing (all unit tests in Rust)
- **Code Quality**: All clippy lints resolved, properly formatted
- **JavaScript**: Minimal (50 lines) - only WASM init and event binding
- **Rust Code**: 543 lines - all application logic

## Architecture

### Component Structure

1. **Rust WASM Module** (`src/lib.rs` - 543 lines)
   - **MapState struct**: Manages viewport position, dimensions, and clamping
   - **Event handlers**: Mouse (drag/pan), keyboard (navigation), minimap clicks
   - **DOM manipulation**: Via web-sys for transform updates and CSS class changes
   - **Coordinate transformations**: Screen-to-world, minimap-to-world conversions
   - **Thread-local state**: MAP_STATE and DRAG_STATE for global access
   - **Helper functions**: Message formatting, distance calculations
   - **Comprehensive test suite**: 10 unit tests covering all core functionality

2. **HTML Interface** (`index.html`)
   - Resource panel (system metrics)
   - Main map (SVG network topology)
   - Minimap (overview with viewport indicator)
   - Server management menu (process controls)
   - MCP configuration menu (protocol settings)
   - Alert console (notifications)

3. **Minimal JavaScript** (`index.html` - 50 lines)
   - WASM module initialization with `init()`
   - Console.log override for status display in UI
   - Event listener attachment (mousedown, mousemove, mouseup, keydown, click)
   - Export Rust functions to window for onclick handlers

4. **Build System**
   - `scripts/build.sh`: Automates `wasm-pack` compilation
   - `scripts/run.sh`: Starts development server and opens browser
   - Generated `pkg/` directory contains WASM binary and JS bindings

### Server Topology (Hardcoded Demo Data)

- **SERVER-ALPHA**: 3 processes (2 LLM, 1 MCP) at coordinates (400, 300)
- **SERVER-BETA**: 4 processes (2 LLM, 2 MCP) at coordinates (2000, 300)
- **SERVER-GAMMA**: 2 processes (1 LLM, 1 MCP) at coordinates (400, 1500)
- **SERVER-DELTA**: 3 processes (2 LLM, 1 MCP) at coordinates (2000, 1500)
- **WORKSTATION-1**: 2 processes (1 LLM, 1 MCP) at coordinates (200, 900)
- **WORKSTATION-2**: 1 process (1 MCP) at coordinates (2200, 900)
- **ROUTER-1**: Central routing node at coordinates (1200, 900)

## Next Steps

### Immediate Priorities (Phase 1)

1. **Backend Integration**
   - Implement actual server communication (WebSocket or REST API)
   - Replace mock data with real metrics from monitored systems
   - Add server agent/daemon for collecting real-time metrics

2. **State Management**
   - Implement proper state tracking in Rust
   - Persist selected servers and configuration
   - Add local storage for UI preferences

3. **Enhanced Testing**
   - Expand unit test coverage beyond helper functions
   - Add integration tests for interaction workflows
   - Implement visual regression testing for UI

### Medium-term Goals (Phase 2)

4. **Dynamic Topology**
   - Allow adding/removing servers at runtime
   - Auto-discovery of new nodes on the network
   - Configurable network layouts (not just star topology)

5. **Real Process Management**
   - Actual deployment of LLM/MCP processes via SSH or agents
   - Real-time process status monitoring
   - Resource allocation controls

6. **Enhanced Visualization**
   - Animated network traffic flow
   - Historical metrics graphing
   - Performance trend analysis

### Long-term Vision (Phase 3)

7. **Production Readiness**
   - Authentication and authorization
   - Multi-user support with role-based access
   - Encrypted communications (TLS/mTLS)
   - Audit logging

8. **Advanced Features**
   - Automated failover and recovery
   - Predictive resource scaling
   - Cost optimization recommendations
   - Integration with cloud providers (AWS, GCP, Azure)

9. **Platform Expansion**
   - Desktop application (Tauri or similar)
   - Mobile monitoring app
   - CLI interface for automation

## Development Workflow

### Quick Start

```bash
# Build the project
./scripts/build.sh

# Run development server (opens browser at http://localhost:4000/)
./scripts/run.sh

# Code quality checks
cargo clippy
cargo fmt
cargo test
```

### Project Structure

```
rts_monitor/
├── src/
│   └── lib.rs          # Main WASM module (only Rust source file)
├── docs/
│   └── overview.md     # This file
├── scripts/
│   ├── build.sh        # Build automation
│   └── run.sh          # Dev server startup
├── pkg/                # Generated WASM output (not version controlled)
├── index.html          # Complete UI with inline CSS/JS
├── Cargo.toml          # Rust dependencies
├── CLAUDE.md           # AI assistant guidance
└── README.md           # Project documentation
```

### Key Design Principles

1. **Clean Separation**: Rust for logic, HTML/CSS for presentation, minimal JavaScript
2. **Single Source File**: All Rust code in one file (`src/lib.rs`) for simplicity
3. **Inline Styling**: All CSS in `index.html` for self-contained UI
4. **No External Dependencies**: Minimal JS libraries, vanilla web technologies
5. **Retro Aesthetic**: Green-on-black terminal theme throughout

## Contributing

When working on this project:
- Keep all Rust code in `src/lib.rs`
- Maintain inline CSS in `index.html`
- Use `#[wasm_bindgen]` for all functions exposed to JavaScript
- Follow the existing retro terminal aesthetic
- Run `cargo clippy` and `cargo fmt` before committing
- Update this overview when adding major features

## References

- **WASM Bindgen**: https://rustwasm.github.io/wasm-bindgen/
- **web-sys**: https://rustwasm.github.io/wasm-bindgen/api/web_sys/
- **wasm-pack**: https://rustwasm.github.io/wasm-pack/

---

*This overview is intended for project management purposes to track progress across multiple projects. For detailed development instructions, see CLAUDE.md.*
