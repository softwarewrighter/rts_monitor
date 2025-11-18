# Interaction Model

This page details the user interaction patterns, event handling, and control flows in RTS Monitor with comprehensive sequence diagrams.

## Table of Contents

- [Interaction Overview](#interaction-overview)
- [Mouse Interactions](#mouse-interactions)
- [Keyboard Interactions](#keyboard-interactions)
- [Minimap Navigation](#minimap-navigation)
- [Event Flow Architecture](#event-flow-architecture)
- [State Transitions](#state-transitions)

## Interaction Overview

RTS Monitor provides multiple interaction methods for navigating and controlling the distributed system visualization.

### Interaction Methods

```mermaid
graph TB
    subgraph "Input Methods"
        A[Mouse Events]
        B[Keyboard Events]
        C[Minimap Clicks]
    end

    subgraph "Actions"
        D[Pan View]
        E[Navigate Map]
        F[Jump to Location]
        G[Select Elements]
    end

    subgraph "Feedback"
        H[Visual Update]
        I[Status Message]
        J[Cursor Change]
    end

    A --> D
    A --> G
    B --> E
    C --> F

    D --> H
    D --> I
    E --> H
    E --> I
    F --> H
    F --> I
    G --> H

    D --> J
    G --> J

    style A fill:#3498db
    style B fill:#3498db
    style C fill:#3498db
    style H fill:#2ecc71
    style I fill:#2ecc71
    style J fill:#2ecc71
```

## Mouse Interactions

### Drag-to-Pan Interaction

The primary mouse interaction is **drag-to-pan**, allowing users to navigate the map by clicking and dragging.

#### Mouse Down Event

```mermaid
sequenceDiagram
    participant User
    participant Browser
    participant JavaScript
    participant WASM
    participant DragState
    participant MapState
    participant DOM

    User->>Browser: Mouse Down on Map
    Browser->>JavaScript: mousedown event
    JavaScript->>WASM: on_mouse_down(event)

    WASM->>WASM: Extract coordinates<br/>(client_x, client_y)
    WASM->>DragState: Set is_dragging = true
    WASM->>DragState: Store start_x, start_y
    WASM->>MapState: Read current position
    MapState-->>WASM: map_x, map_y
    WASM->>DragState: Store map_start_x, map_start_y

    WASM->>DOM: Update cursor style<br/>("grabbing")
    WASM->>DOM: Log message to console
    WASM->>DOM: Update status footer

    DOM-->>Browser: Render updates
    Browser-->>User: Visual feedback
```

**Code Flow (src/lib.rs):**

```rust
#[wasm_bindgen]
pub fn on_mouse_down(event: web_sys::MouseEvent) {
    let x = event.client_x() as f64;
    let y = event.client_y() as f64;

    DRAG_STATE.with(|ds| {
        let mut drag = ds.borrow_mut();
        drag.is_dragging = true;
        drag.start_x = x;
        drag.start_y = y;

        MAP_STATE.with(|ms| {
            let map = ms.borrow();
            drag.map_start_x = map.x;
            drag.map_start_y = map.y;
        });
    });

    // Update UI feedback
    update_cursor("grabbing");
    log_to_console("Drag started");
}
```

#### Mouse Move Event (Dragging)

```mermaid
sequenceDiagram
    participant User
    participant Browser
    participant JavaScript
    participant WASM
    participant DragState
    participant MapState
    participant Coordinates
    participant DOM

    User->>Browser: Drag Mouse
    Browser->>JavaScript: mousemove event (continuous)
    JavaScript->>WASM: on_mouse_move(event)

    WASM->>DragState: Check is_dragging

    alt Dragging Active
        WASM->>WASM: Get current mouse position
        WASM->>DragState: Read start_x, start_y
        WASM->>DragState: Read map_start_x, map_start_y

        WASM->>WASM: Calculate delta<br/>dx = current_x - start_x<br/>dy = current_y - start_y

        WASM->>MapState: new_x = map_start_x - dx
        WASM->>MapState: new_y = map_start_y - dy

        WASM->>Coordinates: Apply pan limits<br/>clamp to bounds
        Coordinates-->>MapState: Bounded coordinates

        WASM->>DOM: Update map transform<br/>translate(new_x, new_y)
        WASM->>DOM: Update minimap viewport
        WASM->>DOM: Update status message

        DOM-->>Browser: Render smooth pan
        Browser-->>User: Real-time visual feedback
    else Not Dragging
        WASM-->>Browser: No action
    end
```

**Coordinate Calculation:**

```rust
// Calculate delta from drag start
let dx = current_x - drag.start_x;
let dy = current_y - drag.start_y;

// Apply to map position (inverted for natural movement)
let new_x = drag.map_start_x - dx;
let new_y = drag.map_start_y - dy;

// Clamp to world bounds
let clamped_x = new_x.max(0.0).min(MAX_X);
let clamped_y = new_y.max(0.0).min(MAX_Y);
```

#### Mouse Up Event

```mermaid
sequenceDiagram
    participant User
    participant Browser
    participant JavaScript
    participant WASM
    participant DragState
    participant DOM

    User->>Browser: Release Mouse Button
    Browser->>JavaScript: mouseup event
    JavaScript->>WASM: on_mouse_up(event)

    WASM->>DragState: Set is_dragging = false
    WASM->>DragState: Clear start coordinates

    WASM->>DOM: Reset cursor style<br/>("default")
    WASM->>DOM: Log "Drag ended" to console
    WASM->>DOM: Update status footer

    DOM-->>Browser: Render updates
    Browser-->>User: Visual feedback (cursor change)
```

### Complete Drag Cycle

```mermaid
stateDiagram-v2
    [*] --> Idle

    Idle --> Dragging: Mouse Down
    Dragging --> Panning: Mouse Move
    Panning --> Panning: Continue Moving
    Panning --> Idle: Mouse Up
    Dragging --> Idle: Mouse Up (no move)

    note right of Idle
        DragState.is_dragging = false
        Cursor = "default"
    end note

    note right of Dragging
        DragState.is_dragging = true
        Store start coordinates
        Cursor = "grabbing"
    end note

    note right of Panning
        Calculate delta
        Update MapState
        Apply transform
        Real-time rendering
    end note
```

## Keyboard Interactions

### Keyboard Navigation

Users can navigate the map using **Arrow Keys** or **WASD** for continuous, smooth panning.

#### Keydown Event Sequence

```mermaid
sequenceDiagram
    participant User
    participant Browser
    participant JavaScript
    participant WASM
    participant MapState
    participant DOM

    User->>Browser: Press Arrow/WASD Key
    Browser->>JavaScript: keydown event
    JavaScript->>WASM: on_keydown(event)

    WASM->>WASM: Get key code<br/>event.key()

    alt Arrow Up / W
        WASM->>MapState: y -= PAN_STEP
    else Arrow Down / S
        WASM->>MapState: y += PAN_STEP
    else Arrow Left / A
        WASM->>MapState: x -= PAN_STEP
    else Arrow Right / D
        WASM->>MapState: x += PAN_STEP
    else Other Key
        WASM-->>Browser: No action
    end

    WASM->>MapState: Clamp to bounds
    WASM->>DOM: Update map transform
    WASM->>DOM: Update minimap viewport
    WASM->>DOM: Update status message
    WASM->>Browser: Prevent default scroll

    DOM-->>Browser: Render smooth pan
    Browser-->>User: Visual feedback
```

**Key Mapping:**

```rust
match event.key().as_str() {
    "ArrowUp" | "w" | "W" => {
        map.y -= PAN_STEP;
        event.prevent_default();
    }
    "ArrowDown" | "s" | "S" => {
        map.y += PAN_STEP;
        event.prevent_default();
    }
    "ArrowLeft" | "a" | "A" => {
        map.x -= PAN_STEP;
        event.prevent_default();
    }
    "ArrowRight" | "d" | "D" => {
        map.x += PAN_STEP;
        event.prevent_default();
    }
    _ => {} // Ignore other keys
}
```

### Keyboard Navigation Flow

```mermaid
graph LR
    A[Key Press] --> B{Which Key?}

    B -->|Up/W| C[y -= 20]
    B -->|Down/S| D[y += 20]
    B -->|Left/A| E[x -= 20]
    B -->|Right/D| F[x += 20]
    B -->|Other| G[Ignore]

    C --> H[Clamp Bounds]
    D --> H
    E --> H
    F --> H

    H --> I[Update Transform]
    I --> J[Render]

    G --> K[No Action]

    style B fill:#f39c12
    style H fill:#3498db
    style I fill:#2ecc71
```

### Continuous Key Press

```mermaid
sequenceDiagram
    participant User
    participant Browser
    participant WASM
    participant MapState

    User->>Browser: Hold Arrow Key

    loop Every keydown event (auto-repeat)
        Browser->>WASM: on_keydown(event)
        WASM->>MapState: Update position
        WASM->>Browser: Render update
        Note over User,Browser: Smooth continuous movement
    end

    User->>Browser: Release Key
    Browser-->>User: Movement stops
```

## Minimap Navigation

### Click-to-Center Interaction

The minimap provides **quick navigation** by clicking to center the main map view on any location.

#### Minimap Click Sequence

```mermaid
sequenceDiagram
    participant User
    participant Browser
    participant JavaScript
    participant WASM
    participant Minimap
    participant Coordinates
    participant MapState
    participant DOM

    User->>Browser: Click on Minimap
    Browser->>JavaScript: click event
    JavaScript->>WASM: on_minimap_click(event)

    WASM->>Minimap: Get click coordinates<br/>(relative to minimap)
    WASM->>WASM: offset_x = event.offset_x()<br/>offset_y = event.offset_y()

    WASM->>Coordinates: Convert minimap to world<br/>screen_to_world_coords()

    Note over WASM,Coordinates: world_x = (offset_x / minimap_width) * world_width<br/>world_y = (offset_y / minimap_height) * world_height

    Coordinates-->>WASM: World coordinates

    WASM->>MapState: Center viewport at world coords<br/>x = world_x - viewport_width/2<br/>y = world_y - viewport_height/2

    WASM->>MapState: Clamp to bounds

    WASM->>DOM: Update main map transform
    WASM->>DOM: Update minimap viewport rect
    WASM->>DOM: Log jump action to console
    WASM->>DOM: Update status message

    DOM-->>Browser: Render instant jump
    Browser-->>User: View centered at clicked location
```

**Coordinate Transformation:**

```rust
fn screen_to_world_coords(screen_x: f64, screen_y: f64,
                          minimap_width: f64, minimap_height: f64,
                          world_width: f64, world_height: f64)
                          -> (f64, f64) {
    let world_x = (screen_x / minimap_width) * world_width;
    let world_y = (screen_y / minimap_height) * world_height;
    (world_x, world_y)
}

fn center_viewport(world_x: f64, world_y: f64,
                   viewport_width: f64, viewport_height: f64)
                   -> (f64, f64) {
    let map_x = world_x - (viewport_width / 2.0);
    let map_y = world_y - (viewport_height / 2.0);
    (map_x, map_y)
}
```

### Minimap Interaction Flow

```mermaid
graph TB
    A[Click Minimap] --> B[Get Offset Coords]
    B --> C[Scale to World Space]
    C --> D[Center Viewport]
    D --> E[Clamp to Bounds]
    E --> F[Update Main Map]
    F --> G[Update Minimap Rect]
    G --> H[Visual Feedback]

    I[World Width: 2000]
    J[World Height: 1500]
    K[Minimap: 200x150]
    L[Viewport: 800x600]

    I -.-> C
    J -.-> C
    K -.-> C
    L -.-> D

    style A fill:#3498db
    style C fill:#e74c3c,color:#fff
    style D fill:#e74c3c,color:#fff
    style E fill:#f39c12
    style H fill:#2ecc71
```

## Event Flow Architecture

### Complete Event Processing Pipeline

```mermaid
graph TB
    subgraph "Browser Layer"
        A[User Input]
        B[Browser Event]
    end

    subgraph "JavaScript Layer"
        C[Event Listener]
        D[WASM Function Call]
    end

    subgraph "Rust/WASM Layer"
        E[Event Handler]
        F[Extract Event Data]
        G{Event Type?}
        H[Mouse Logic]
        I[Keyboard Logic]
        J[Click Logic]
    end

    subgraph "State Layer"
        K[Update DragState]
        L[Update MapState]
        M[Coordinate Transform]
        N[Bounds Checking]
    end

    subgraph "Rendering Layer"
        O[DOM Manipulation]
        P[SVG Transform]
        Q[Status Update]
        R[Console Log]
    end

    A --> B
    B --> C
    C --> D
    D --> E
    E --> F
    F --> G
    G -->|Mouse| H
    G -->|Keyboard| I
    G -->|Minimap| J

    H --> K
    H --> L
    I --> L
    J --> L

    K --> M
    L --> M
    M --> N

    N --> O
    O --> P
    O --> Q
    O --> R

    P --> S[Browser Render]
    Q --> S
    R --> S

    style E fill:#4a9eff
    style K fill:#e74c3c,color:#fff
    style L fill:#e74c3c,color:#fff
    style M fill:#f39c12
    style S fill:#2ecc71
```

### Event Handler Mapping

| Browser Event | JavaScript Function | Rust Handler | State Modified |
|---------------|-------------------|--------------|----------------|
| `mousedown` | `on_mouse_down` | `on_mouse_down()` | `DragState` |
| `mousemove` | `on_mouse_move` | `on_mouse_move()` | `MapState`, `DragState` |
| `mouseup` | `on_mouse_up` | `on_mouse_up()` | `DragState` |
| `keydown` | `on_keydown` | `on_keydown()` | `MapState` |
| `click` (minimap) | `on_minimap_click` | `on_minimap_click()` | `MapState` |

## State Transitions

### MapState Lifecycle

```mermaid
stateDiagram-v2
    [*] --> Initialized: new()

    Initialized --> Panning: Mouse Drag / Keyboard
    Panning --> Panning: Continue Input
    Panning --> ViewUpdated: Calculate New Position
    ViewUpdated --> BoundsCheck: Apply Limits
    BoundsCheck --> Rendering: Clamp Coordinates
    Rendering --> Stable: DOM Updated

    Stable --> Panning: New Input
    Stable --> Jumping: Minimap Click

    Jumping --> ViewUpdated: Center on Target

    note right of Initialized
        x = 0, y = 0
        width = 800, height = 600
    end note

    note right of BoundsCheck
        x.clamp(0, MAX_X)
        y.clamp(0, MAX_Y)
    end note

    note right of Rendering
        Update SVG transform
        Update minimap viewport
        Update status message
    end note
```

### DragState Lifecycle

```mermaid
stateDiagram-v2
    [*] --> Idle

    Idle --> DragStart: Mouse Down
    DragStart --> Active: Store Coords
    Active --> Updating: Mouse Move
    Updating --> Updating: Continue Dragging
    Updating --> DragEnd: Mouse Up
    DragEnd --> Idle: Reset State

    note right of Idle
        is_dragging = false
        All coords = 0
    end note

    note right of Active
        is_dragging = true
        start_x, start_y set
        map_start_x, map_start_y set
    end note

    note right of DragEnd
        is_dragging = false
        Clear coordinates
    end note
```

### Combined State Flow

```mermaid
sequenceDiagram
    participant Input
    participant DragState
    participant MapState
    participant UI

    rect rgb(200, 220, 255)
        Note over Input,UI: Mouse Drag Cycle
        Input->>DragState: Mouse Down
        DragState->>DragState: is_dragging = true

        loop While Dragging
            Input->>DragState: Mouse Move
            DragState->>MapState: Calculate new position
            MapState->>MapState: Apply bounds
            MapState->>UI: Update transform
        end

        Input->>DragState: Mouse Up
        DragState->>DragState: is_dragging = false
    end

    rect rgb(200, 255, 220)
        Note over Input,UI: Keyboard Navigation
        Input->>MapState: Arrow Key Press
        MapState->>MapState: Adjust x/y position
        MapState->>MapState: Apply bounds
        MapState->>UI: Update transform
    end

    rect rgb(255, 220, 200)
        Note over Input,UI: Minimap Jump
        Input->>MapState: Minimap Click
        MapState->>MapState: Convert coordinates
        MapState->>MapState: Center viewport
        MapState->>MapState: Apply bounds
        MapState->>UI: Update transform
    end
```

## Performance Optimizations

### Event Throttling Strategy

```mermaid
graph LR
    A[Mouse Move Events<br/>100+ per second] --> B{Needs Update?}
    B -->|Yes| C[Update State]
    B -->|No| D[Skip Processing]

    C --> E[DOM Update]
    E --> F[Browser Render<br/>~60 FPS]

    D --> G[Ignored]

    style A fill:#e74c3c,color:#fff
    style B fill:#f39c12
    style E fill:#3498db
    style F fill:#2ecc71
```

**Optimization Techniques:**

1. **Request Animation Frame**: Sync updates with browser rendering
2. **Conditional Updates**: Only update if position actually changed
3. **Batch DOM Operations**: Group multiple attribute changes
4. **WASM Speed**: Native-speed calculations vs JavaScript

### Update Efficiency

```rust
// Efficient state update pattern
DRAG_STATE.with(|ds| {
    let drag = ds.borrow();
    if !drag.is_dragging {
        return; // Early exit - no update needed
    }

    MAP_STATE.with(|ms| {
        let mut map = ms.borrow_mut();
        // Calculate new position
        let new_x = calculate_new_x();
        let new_y = calculate_new_y();

        // Only update if changed
        if (new_x - map.x).abs() > 0.1 || (new_y - map.y).abs() > 0.1 {
            map.x = new_x;
            map.y = new_y;
            update_dom(&map); // Single DOM update
        }
    });
});
```

## Coordinate Systems

### Coordinate Spaces

```mermaid
graph TB
    subgraph "Screen Space"
        A[Browser Window<br/>1024x768]
        B[Map Container<br/>800x600]
    end

    subgraph "World Space"
        C[Full Network<br/>2000x1500]
        D[Visible Area<br/>800x600]
    end

    subgraph "Minimap Space"
        E[Minimap<br/>200x150]
        F[Viewport Indicator<br/>Scaled]
    end

    A --> B
    B --> D
    C --> D
    C --> E
    D --> F

    style A fill:#3498db
    style C fill:#e74c3c,color:#fff
    style E fill:#2ecc71
```

### Coordinate Transformations

**Screen to World:**
```rust
world_x = (screen_x / screen_width) * world_width
world_y = (screen_y / screen_height) * world_height
```

**World to Screen:**
```rust
screen_x = (world_x / world_width) * screen_width
screen_y = (world_y / world_height) * screen_height
```

**Viewport Centering:**
```rust
map_x = target_world_x - (viewport_width / 2.0)
map_y = target_world_y - (viewport_height / 2.0)
```

## Related Pages

- [Architecture](Architecture.md) - System architecture overview
- [UI Components](UI-Components.md) - UI component details
- [Technology Stack](Technology-Stack.md) - Technologies used
- [Development Guide](Development-Guide.md) - Development workflow
- [Home](Home.md) - Return to wiki home
