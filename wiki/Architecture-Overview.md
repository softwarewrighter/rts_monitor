# Architecture Overview

This page provides a comprehensive overview of the RTS Monitor architecture, including high-level design, component relationships, and data flow.

## High-Level Architecture

The RTS Monitor follows a **Rust-First Architecture** where all application logic resides in Rust (compiled to WebAssembly), with minimal JavaScript for initialization and event binding.

```mermaid
graph TB
    subgraph Browser
        HTML[HTML/CSS UI]
        JS[JavaScript Init]
        WASM[Rust WASM Module]
        DOM[Browser DOM]
        WebAPIs[Web APIs]
    end

    User[User Input] --> HTML
    HTML --> JS
    JS -->|Initialize| WASM
    HTML -->|Events| WASM
    WASM -->|Manipulate| DOM
    WASM -->|Access| WebAPIs
    DOM --> HTML

    style WASM fill:#4a9eff
    style HTML fill:#66cc66
    style User fill:#ffaa44
```

### Design Principles

1. **Rust-First**: All application logic in Rust, compiled to WASM
2. **Minimal JavaScript**: Only ~50 lines for WASM init and event binding
3. **Inline Styling**: All CSS in `index.html` for self-contained UI
4. **SVG Graphics**: Scalable vector graphics for map and minimap
5. **Thread-Local State**: Global state management via Rust thread-locals

## System Architecture

```mermaid
graph LR
    subgraph Frontend
        UI[HTML/CSS UI]
        Init[JS Initialization]
    end

    subgraph WASM Module
        State[State Management]
        Events[Event Handlers]
        DOM_Man[DOM Manipulation]
        Coord[Coordinate Math]
        Tests[Test Suite]
    end

    subgraph Browser APIs
        web_sys[web-sys Bindings]
        Console[Console API]
        SVG_DOM[SVG DOM]
    end

    UI --> Init
    Init --> Events
    Events --> State
    State --> DOM_Man
    DOM_Man --> web_sys
    web_sys --> SVG_DOM
    Events --> Coord
    Events --> Console

    style State fill:#ff9999
    style Events fill:#9999ff
    style DOM_Man fill:#99ff99
```

## Component Architecture

```mermaid
graph TD
    subgraph Rust WASM src/lib.rs
        MapState[MapState Struct]
        DragState[DragState Struct]

        subgraph Event Handlers
            Mouse[Mouse Events]
            Keyboard[Keyboard Events]
            Minimap[Minimap Events]
        end

        subgraph Core Functions
            Transform[Coordinate Transform]
            Clamp[Viewport Clamping]
            Update[DOM Updates]
        end

        subgraph Utilities
            Message[Message Formatting]
            Distance[Distance Calculation]
        end
    end

    Mouse --> DragState
    DragState --> MapState
    MapState --> Transform
    Transform --> Clamp
    Clamp --> Update

    Keyboard --> MapState
    Minimap --> MapState

    Mouse --> Message
    Keyboard --> Message

    style MapState fill:#ffcccc
    style DragState fill:#ccccff
```

## Layer Architecture

The system is organized into three primary layers:

### 1. Presentation Layer (HTML/CSS)

**File**: `index.html` (complete UI definition)

**Components**:
- Resource Panel (system metrics)
- Main Map (SVG topology view)
- Minimap (overview + viewport indicator)
- Server Management Menu
- MCP Configuration Menu
- Alert Console

**Responsibilities**:
- Visual layout and styling
- SVG element definitions
- UI component structure

### 2. Application Layer (Rust/WASM)

**File**: `src/lib.rs` (543 lines of Rust)

**Components**:
- State management (MapState, DragState)
- Event handling (mouse, keyboard, minimap)
- Business logic (coordinate transforms, viewport control)
- DOM manipulation (via web-sys)

**Responsibilities**:
- All application logic
- State transitions
- User interaction processing
- DOM updates

### 3. Browser API Layer (web-sys)

**Bindings**: Rust → Browser APIs

**Components**:
- DOM manipulation
- Console logging
- Event system
- SVG rendering

**Responsibilities**:
- Browser API access from Rust
- Type-safe JavaScript interop
- Platform integration

## Data Flow

```mermaid
flowchart TD
    UserAction[User Action] --> EventCapture[Event Capture - JS]
    EventCapture --> WASMHandler[WASM Event Handler]

    WASMHandler --> StateRead[Read Thread-Local State]
    StateRead --> StateLogic[Process State Logic]
    StateLogic --> StateUpdate[Update Thread-Local State]

    StateUpdate --> DOMUpdate[Update DOM via web-sys]
    DOMUpdate --> Render[Browser Renders]

    StateLogic --> Console[Log to Console]
    Console --> UIFooter[Display in UI Footer]

    style UserAction fill:#ffaa44
    style WASMHandler fill:#4a9eff
    style StateUpdate fill:#ff9999
    style Render fill:#66cc66
```

### User Interaction Flow

1. **User Action**: Mouse click, drag, keyboard press
2. **Event Capture**: JavaScript event listener triggers
3. **WASM Handler**: Rust function called via wasm-bindgen
4. **State Processing**: Thread-local state read and modified
5. **DOM Update**: web-sys updates SVG transforms and attributes
6. **Render**: Browser re-renders updated elements
7. **Feedback**: Console messages displayed in UI footer

## State Management Architecture

```mermaid
stateDiagram-v2
    [*] --> Initialized
    Initialized --> Idle: WASM Module Loaded

    Idle --> Dragging: Mouse Down on Map
    Idle --> Panning: Keyboard Arrow/WASD
    Idle --> MinimapNav: Click on Minimap

    Dragging --> Dragging: Mouse Move
    Dragging --> Idle: Mouse Up

    Panning --> Idle: Viewport Updated
    MinimapNav --> Idle: Viewport Centered

    Dragging --> ViewportUpdate: Calculate Delta
    Panning --> ViewportUpdate: Apply Pan Offset
    MinimapNav --> ViewportUpdate: Calculate Center

    ViewportUpdate --> ClampViewport: Enforce Bounds
    ClampViewport --> UpdateDOM: Apply Transform
    UpdateDOM --> Idle: Render Complete
```

### State Storage

**Thread-Local Variables** (Rust):
```rust
thread_local! {
    static MAP_STATE: RefCell<Option<MapState>> = RefCell::new(None);
    static DRAG_STATE: RefCell<Option<DragState>> = RefCell::new(None);
}
```

**MapState Fields**:
- `offset_x`, `offset_y`: Current viewport position
- `viewport_width`, `viewport_height`: Visible area dimensions
- `world_width`, `world_height`: Total world dimensions

**DragState Fields**:
- `start_x`, `start_y`: Initial mouse position
- `initial_offset_x`, `initial_offset_y`: Viewport position at drag start

## Build and Deployment Architecture

```mermaid
flowchart LR
    Source[Rust Source] --> Cargo[Cargo Build]
    Cargo --> WASMPack[wasm-pack]
    WASMPack --> PKG[pkg/ Directory]

    PKG --> WASM_Binary[.wasm Binary]
    PKG --> JS_Glue[JS Bindings]
    PKG --> TS_Defs[TypeScript Defs]

    HTML[index.html] --> Server[HTTP Server]
    WASM_Binary --> Server
    JS_Glue --> Server

    Server --> Browser[Web Browser]

    style Source fill:#ff9966
    style WASMPack fill:#4a9eff
    style Browser fill:#66cc66
```

### Build Process

1. **Source**: Rust code in `src/lib.rs`
2. **Compilation**: `wasm-pack build --target web`
3. **Output**: WASM binary + JS bindings in `pkg/`
4. **Serving**: `basic-http-server` serves static files
5. **Loading**: Browser loads HTML, JS initializes WASM

## Technology Stack Details

For detailed information about the technology stack, see [[Technology Stack]].

## Component Details

For detailed information about specific components:

- [[UI Components]] - Presentation layer details
- [[State Management]] - State management patterns
- [[Event Handling]] - Event processing architecture
- [[Component Details]] - Component implementation details

## Related Pages

- [[Interaction Flows]] - Sequence diagrams for user interactions
- [[Build and Deploy]] - Detailed build and deployment process
- [[Development Guide]] - Development workflow and best practices

---

*Last Updated: 2025-11-18*
