# Architecture Overview

This page provides a comprehensive overview of the RTS Monitor architecture, including system design, component interactions, and key architectural decisions.

## Table of Contents

- [System Architecture](#system-architecture)
- [Architectural Principles](#architectural-principles)
- [Component Architecture](#component-architecture)
- [Data Flow](#data-flow)
- [State Management](#state-management)
- [File Structure](#file-structure)

## System Architecture

RTS Monitor follows a **Rust-first WebAssembly architecture**, where all application logic resides in Rust and compiles to WebAssembly for browser execution.

### High-Level Architecture

```mermaid
graph TB
    subgraph "Browser Environment"
        subgraph "Presentation Layer"
            HTML[HTML Structure<br/>index.html]
            CSS[Inline CSS Styling]
            SVG[SVG Graphics<br/>Map & Minimap]
        end

        subgraph "Minimal JavaScript Layer"
            INIT[WASM Initialization<br/>50 lines]
            BIND[Event Binding]
        end

        subgraph "Rust/WASM Core"
            LOGIC[Application Logic<br/>543 lines]
            STATE[State Management]
            EVENTS[Event Handlers]
            RENDER[DOM Manipulation]
        end

        subgraph "Browser APIs"
            WEBSYS[web-sys<br/>Browser Bindings]
            DOM_API[DOM APIs]
            EVENT_API[Event APIs]
        end
    end

    HTML --> INIT
    CSS --> HTML
    SVG --> HTML
    INIT --> LOGIC
    BIND --> EVENTS
    LOGIC --> STATE
    LOGIC --> EVENTS
    LOGIC --> RENDER
    RENDER --> WEBSYS
    EVENTS --> WEBSYS
    WEBSYS --> DOM_API
    WEBSYS --> EVENT_API
    DOM_API --> HTML
    EVENT_API --> HTML

    style LOGIC fill:#4a9eff,stroke:#333,stroke-width:3px
    style STATE fill:#2ecc71
    style EVENTS fill:#2ecc71
    style RENDER fill:#2ecc71
```

### System Layers

```mermaid
graph LR
    subgraph "Layer 1: Presentation"
        L1[HTML/CSS/SVG<br/>Static UI Structure]
    end

    subgraph "Layer 2: Initialization"
        L2[JavaScript Bootstrap<br/>WASM Loading]
    end

    subgraph "Layer 3: Application Logic"
        L3[Rust/WASM<br/>All Business Logic]
    end

    subgraph "Layer 4: Browser Interface"
        L4[web-sys<br/>Browser API Bindings]
    end

    L1 --> L2
    L2 --> L3
    L3 --> L4
    L4 -.->|Updates| L1

    style L3 fill:#4a9eff,stroke:#333,stroke-width:2px
```

## Architectural Principles

### 1. Rust-First Philosophy

**80% reduction in JavaScript** - All application logic implemented in Rust.

```mermaid
pie title Code Distribution
    "Rust Logic" : 543
    "JavaScript Init" : 50
    "HTML/CSS" : 200
```

**Benefits:**
- Type safety and memory safety
- High performance through WASM
- Better testing and maintainability
- Compile-time error checking

### 2. Single Source of Truth

All state managed in Rust via thread-local storage:
- `MAP_STATE`: Viewport position and dimensions
- `DRAG_STATE`: Mouse drag tracking

### 3. Minimal JavaScript Surface

JavaScript responsibilities limited to:
- WASM module loading and initialization
- Event listener binding to exported Rust functions
- No business logic

### 4. Direct Browser API Access

Using `web-sys` for:
- DOM manipulation
- Event handling
- SVG updates
- Console logging

## Component Architecture

### Core Components

```mermaid
graph TB
    subgraph "Rust Core Components"
        MS[MapState<br/>Viewport Management]
        DS[DragState<br/>Interaction Tracking]
        EH[Event Handlers<br/>Mouse & Keyboard]
        DM[DOM Manipulator<br/>UI Updates]
        CT[Coordinate Transform<br/>Screen ↔ World]
    end

    subgraph "UI Components"
        RP[Resource Panel<br/>System Metrics]
        MAP[Main Map<br/>SVG Topology]
        MM[Minimap<br/>Overview]
        SM[Server Menu<br/>Process Management]
        MC[MCP Menu<br/>Configuration]
        AC[Alert Console<br/>Notifications]
    end

    EH --> MS
    EH --> DS
    MS --> CT
    CT --> DM
    DM --> MAP
    DM --> MM
    MS --> DM

    MAP --> RP
    MAP --> SM
    MAP --> MC
    MAP --> AC

    style MS fill:#e74c3c,color:#fff
    style DS fill:#e74c3c,color:#fff
    style EH fill:#3498db,color:#fff
    style DM fill:#3498db,color:#fff
    style CT fill:#3498db,color:#fff
```

### MapState Structure

```rust
struct MapState {
    x: f64,        // Current viewport X position
    y: f64,        // Current viewport Y position
    width: f64,    // Viewport width
    height: f64,   // Viewport height
}
```

**Responsibilities:**
- Track current viewport position
- Manage viewport dimensions
- Provide bounds for pan limits
- Calculate visible area

### DragState Structure

```rust
struct DragState {
    is_dragging: bool,     // Mouse drag active
    start_x: f64,          // Drag start X
    start_y: f64,          // Drag start Y
    map_start_x: f64,      // Map position at drag start
    map_start_y: f64,      // Map position at drag start
}
```

**Responsibilities:**
- Track mouse drag operations
- Store drag start coordinates
- Enable smooth pan calculations

## Data Flow

### Event Flow Architecture

```mermaid
sequenceDiagram
    participant User
    participant Browser
    participant JS
    participant WASM
    participant MapState
    participant DOM

    User->>Browser: Mouse/Keyboard Event
    Browser->>JS: Event Trigger
    JS->>WASM: Call exported function
    WASM->>MapState: Update state
    MapState->>WASM: Return new state
    WASM->>DOM: Update UI via web-sys
    DOM->>Browser: Render changes
    Browser->>User: Visual feedback
```

### State Update Flow

```mermaid
graph LR
    A[User Input] --> B{Event Type}
    B -->|Mouse Down| C[Start Drag]
    B -->|Mouse Move| D[Update Pan]
    B -->|Mouse Up| E[End Drag]
    B -->|Keyboard| F[Navigate]
    B -->|Minimap Click| G[Center View]

    C --> H[Update DragState]
    D --> H
    E --> H
    F --> I[Update MapState]
    G --> I

    H --> J[Render Update]
    I --> J
    J --> K[DOM Update]
    K --> L[Visual Feedback]

    style B fill:#f39c12
    style H fill:#e74c3c,color:#fff
    style I fill:#e74c3c,color:#fff
    style J fill:#3498db,color:#fff
```

## State Management

### Thread-Local Storage Pattern

```mermaid
graph TB
    subgraph "Thread-Local State"
        TLS1[MAP_STATE<br/>RefCell&lt;MapState&gt;]
        TLS2[DRAG_STATE<br/>RefCell&lt;DragState&gt;]
    end

    subgraph "Event Handlers"
        H1[on_mouse_down]
        H2[on_mouse_move]
        H3[on_mouse_up]
        H4[on_keydown]
        H5[on_minimap_click]
    end

    subgraph "State Operations"
        R[Read State]
        W[Write State]
        U[Update UI]
    end

    H1 --> R
    H2 --> R
    H3 --> R
    H4 --> R
    H5 --> R

    R --> TLS1
    R --> TLS2

    H1 --> W
    H2 --> W
    H3 --> W
    H4 --> W
    H5 --> W

    W --> TLS1
    W --> TLS2

    W --> U

    style TLS1 fill:#e74c3c,color:#fff
    style TLS2 fill:#e74c3c,color:#fff
```

**Advantages:**
- Simple, predictable state access
- No complex state management library needed
- Fast access via RefCell borrowing
- Thread-safe for single-threaded WASM

## File Structure

### Project Layout

```
rts_monitor/
├── src/
│   └── lib.rs              # Main Rust/WASM module (543 lines)
│                           # - MapState & DragState structures
│                           # - All event handlers
│                           # - Coordinate transformations
│                           # - DOM manipulation
│                           # - Exported WASM functions
│
├── index.html              # Complete UI structure
│                           # - Inline CSS styling
│                           # - SVG map and minimap
│                           # - UI component layout
│                           # - JavaScript WASM initialization
│
├── pkg/                    # Generated by wasm-pack (auto-generated)
│   ├── rts_monitor.js      # JavaScript bindings
│   ├── rts_monitor_bg.wasm # Compiled WebAssembly
│   └── ...                 # Supporting files
│
├── scripts/
│   ├── build.sh            # Build automation
│   └── run.sh              # Development server
│
├── Cargo.toml              # Rust dependencies
└── CLAUDE.md               # Development guidance
```

### Component Responsibility Matrix

| Component | File | Responsibility | Lines |
|-----------|------|----------------|-------|
| Application Logic | `src/lib.rs` | All business logic, state, events | 543 |
| UI Structure | `index.html` | HTML layout, CSS styling | ~200 |
| WASM Init | `index.html` (script) | Load and initialize WASM | 50 |
| Build Artifacts | `pkg/*` | Compiled WASM + JS bindings | Auto-gen |

## Design Patterns

### 1. **WASM Bindgen Pattern**

Exported functions use `#[wasm_bindgen]` attribute:

```rust
#[wasm_bindgen]
pub fn on_mouse_down(event: web_sys::MouseEvent) {
    // Event handling logic
}
```

### 2. **Interior Mutability Pattern**

Thread-local state with RefCell for safe mutation:

```rust
thread_local! {
    static MAP_STATE: RefCell<MapState> = RefCell::new(MapState::new());
}
```

### 3. **Direct DOM Manipulation**

Using web-sys for browser API access:

```rust
let document = window().unwrap().document().unwrap();
let element = document.get_element_by_id("map").unwrap();
element.set_attribute("transform", &format!(...));
```

## Performance Considerations

### WebAssembly Benefits

```mermaid
graph LR
    A[Rust Source] -->|Compile| B[WASM Binary]
    B -->|Load| C[Browser]
    C -->|Execute| D[Near-Native Speed]

    E[JavaScript] -->|Interpret/JIT| C

    D -.->|Faster than| E

    style B fill:#4a9eff
    style D fill:#2ecc71
```

**Performance Advantages:**
- Compiled code vs. interpreted JavaScript
- Small binary size (~100KB)
- Fast startup time
- Efficient memory usage
- Predictable performance

### Optimization Strategies

1. **Minimal DOM Updates**: Only update changed elements
2. **Event Debouncing**: Efficient mouse move handling
3. **Thread-Local State**: Fast state access without overhead
4. **SVG Transformations**: Hardware-accelerated graphics
5. **WASM Linear Memory**: Efficient memory management

## Related Pages

- [Technology Stack](Technology-Stack.md) - Detailed technology information
- [UI Components](UI-Components.md) - UI component details
- [Interaction Model](Interaction-Model.md) - Event handling and user interactions
- [Development Guide](Development-Guide.md) - Build and development workflow
