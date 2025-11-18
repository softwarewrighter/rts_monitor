# Interaction Flows

This page documents common user interaction workflows in RTS Monitor using sequence diagrams and flow charts.

## Overview

This page illustrates the complete flow of user interactions from input to visual feedback, showing the collaboration between JavaScript, Rust/WASM, state management, and DOM updates.

## Interaction Categories

1. **Mouse Drag to Pan** - Primary navigation method
2. **Keyboard Navigation** - Arrow keys and WASD panning
3. **Minimap Click** - Quick viewport positioning
4. **Application Initialization** - Startup sequence

---

## 1. Mouse Drag to Pan

The most common interaction: dragging the map to pan the viewport.

### Complete Sequence

```mermaid
sequenceDiagram
    actor User
    participant Browser
    participant JS as JavaScript
    participant Rust as Rust WASM
    participant MapState
    participant DragState
    participant DOM
    participant Console

    Note over User,Console: User Starts Dragging

    User->>Browser: Mouse Down on Map
    Browser->>JS: mousedown event
    JS->>JS: Get element coordinates
    JS->>Rust: on_mouse_down(x, y)

    Rust->>MapState: Read current offset
    MapState-->>Rust: offset_x, offset_y
    Rust->>DragState: Create new DragState
    Note over DragState: start_x, start_y,<br/>initial_offset_x, initial_offset_y
    Rust->>DOM: Set cursor='grabbing'
    DOM-->>User: Cursor changes

    Note over User,Console: User Drags Mouse

    User->>Browser: Mouse Move
    Browser->>JS: mousemove event (repeated)
    JS->>Rust: on_mouse_move(x, y)

    Rust->>DragState: Read drag start position
    DragState-->>Rust: start_x, start_y, initial_offset
    Rust->>Rust: Calculate delta
    Note over Rust: delta_x = x - start_x<br/>delta_y = y - start_y

    Rust->>MapState: Update offset (inverted)
    Note over MapState: offset_x = initial - delta_x<br/>offset_y = initial - delta_y
    MapState->>MapState: clamp_offset()

    Rust->>DOM: Update SVG transform
    Note over DOM: translate(-offset_x, -offset_y)
    Rust->>DOM: Update minimap viewport rect
    Rust->>Console: Log viewport position
    Console-->>DOM: Display in footer
    DOM-->>User: Map moves smoothly

    Note over User,Console: User Releases Mouse

    User->>Browser: Mouse Up
    Browser->>JS: mouseup event
    JS->>Rust: on_mouse_up()

    Rust->>DragState: Destroy DragState
    Note over DragState: Set to None
    Rust->>DOM: Set cursor='grab'
    DOM-->>User: Cursor changes back
```

### Step-by-Step Breakdown

#### Phase 1: Mouse Down

1. **User Action**: Clicks and holds on map
2. **Event Capture**: Browser fires `mousedown`
3. **Coordinate Calc**: JavaScript calculates element-relative coordinates
4. **Rust Handler**: `on_mouse_down(x, y)` called
5. **State Creation**: `DragState` created with:
   - Current mouse position
   - Current viewport offset
6. **Visual Feedback**: Cursor changes to `grabbing`

#### Phase 2: Mouse Move (Repeated)

1. **User Action**: Moves mouse while holding button
2. **Event Capture**: Browser fires `mousemove` (60+ Hz)
3. **Rust Handler**: `on_mouse_move(x, y)` called
4. **Delta Calculation**:
   ```
   delta_x = current_x - start_x
   delta_y = current_y - start_y
   ```
5. **Offset Update** (inverted for natural drag):
   ```
   offset_x = initial_offset_x - delta_x
   offset_y = initial_offset_y - delta_y
   ```
6. **Clamping**: Ensure viewport stays in bounds
7. **DOM Update**: SVG `transform` attribute updated
8. **Minimap Update**: Viewport rectangle repositioned
9. **Visual Feedback**: Map appears to move with mouse

#### Phase 3: Mouse Up

1. **User Action**: Releases mouse button
2. **Event Capture**: Browser fires `mouseup`
3. **Rust Handler**: `on_mouse_up()` called
4. **State Cleanup**: `DragState` destroyed
5. **Visual Feedback**: Cursor changes back to `grab`
6. **Result**: Viewport remains at new position

---

## 2. Keyboard Navigation

Panning the viewport using arrow keys or WASD.

### Complete Sequence

```mermaid
sequenceDiagram
    actor User
    participant Browser
    participant JS as JavaScript
    participant Rust as Rust WASM
    participant MapState
    participant DOM
    participant Console

    User->>Browser: Press Arrow/WASD Key
    Browser->>JS: keydown event
    JS->>Rust: on_key_down(key)

    Rust->>Rust: Match key to direction
    Note over Rust: ArrowUp/W → (0, -50)<br/>ArrowDown/S → (0, +50)<br/>ArrowLeft/A → (-50, 0)<br/>ArrowRight/D → (+50, 0)

    Rust->>MapState: pan(delta_x, delta_y)
    MapState->>MapState: offset += delta
    MapState->>MapState: clamp_offset()

    Rust->>DOM: Update SVG transform
    Note over DOM: translate(-offset_x, -offset_y)

    Rust->>DOM: Update minimap viewport rect
    Note over DOM: Update rect position

    Rust->>Console: Log direction
    Console-->>DOM: Display "Panned [direction]"

    DOM-->>User: Map moves 50px in direction
```

### Key Handling Flow

```mermaid
flowchart TD
    Start[Key Press] --> GetKey{Which Key?}

    GetKey -->|Arrow Up / W| Up[Pan Up: delta_y = -50]
    GetKey -->|Arrow Down / S| Down[Pan Down: delta_y = +50]
    GetKey -->|Arrow Left / A| Left[Pan Left: delta_x = -50]
    GetKey -->|Arrow Right / D| Right[Pan Right: delta_x = +50]
    GetKey -->|Other| Ignore[Ignore]

    Up --> Update[Update MapState]
    Down --> Update
    Left --> Update
    Right --> Update

    Update --> Clamp[Clamp to Bounds]
    Clamp --> UpdateDOM[Update DOM]
    UpdateDOM --> Feedback[Visual Feedback]
    Feedback --> End[Done]

    Ignore --> End

    style Update fill:#ffcccc
    style UpdateDOM fill:#99ff99
```

### Step-by-Step

1. **User Action**: Presses arrow key or WASD
2. **Event Capture**: Browser fires `keydown`
3. **Key Identification**: JavaScript passes `event.key` to Rust
4. **Direction Mapping**:
   - `ArrowUp`/`W` → `pan(0, -50)`
   - `ArrowDown`/`S` → `pan(0, +50)`
   - `ArrowLeft`/`A` → `pan(-50, 0)`
   - `ArrowRight`/`D` → `pan(+50, 0)`
5. **State Update**: `MapState.offset` incremented
6. **Clamping**: Bounds enforcement
7. **DOM Update**: Transform and minimap updated
8. **Feedback**: Status message displayed

---

## 3. Minimap Click to Center

Clicking the minimap to quickly navigate to a specific area.

### Complete Sequence

```mermaid
sequenceDiagram
    actor User
    participant Browser
    participant JS as JavaScript
    participant Rust as Rust WASM
    participant MapState
    participant DOM
    participant Console

    User->>Browser: Click on Minimap
    Browser->>JS: click event
    JS->>JS: Get minimap-relative coordinates
    JS->>Rust: minimap_click(mini_x, mini_y)

    Rust->>Rust: Scale minimap to world coords
    Note over Rust: world_x = mini_x * 15.0<br/>world_y = mini_y * 15.0<br/>(3000 / 200 = 15)

    Rust->>Rust: Calculate centered viewport
    Note over Rust: offset_x = world_x - viewport_w/2<br/>offset_y = world_y - viewport_h/2

    Rust->>MapState: Update offset_x, offset_y
    MapState->>MapState: clamp_offset()

    Rust->>DOM: Update SVG transform
    Note over DOM: Viewport jumps to position

    Rust->>DOM: Update minimap viewport rect
    Note over DOM: Viewport indicator moves

    Rust->>Console: Log centered coordinates
    Console-->>DOM: Display "Centered on (x, y)"

    DOM-->>User: Map jumps to clicked area
```

### Coordinate Transformation Flow

```mermaid
flowchart TD
    Start[Minimap Click] --> GetCoords[Get Click Coords<br/>minimap_x, minimap_y]
    GetCoords --> Scale[Scale to World]
    Note1[world_x = minimap_x * 15.0<br/>world_y = minimap_y * 15.0]

    Scale --> Center[Calculate Center Offset]
    Note2[offset_x = world_x - 400<br/>offset_y = world_y - 300]

    Center --> Clamp{In Bounds?}
    Clamp -->|No| ClampFix[Clamp to [0, max]]
    Clamp -->|Yes| Update[Update DOM]
    ClampFix --> Update

    Update --> Transform[Update SVG Transform]
    Transform --> MiniRect[Update Minimap Rect]
    MiniRect --> End[Done]

    style Scale fill:#ffffcc
    style Center fill:#ffcccc
    style Update fill:#99ff99
```

### Step-by-Step

1. **User Action**: Clicks on minimap
2. **Event Capture**: Browser fires `click`
3. **Coordinate Calc**: JavaScript gets minimap-relative position
4. **Rust Handler**: `minimap_click(mini_x, mini_y)` called
5. **Coordinate Scaling**:
   ```
   scale = 3000 / 200 = 15.0
   world_x = minimap_x * 15.0
   world_y = minimap_y * 15.0
   ```
6. **Center Calculation**:
   ```
   offset_x = world_x - viewport_width / 2   (400)
   offset_y = world_y - viewport_height / 2  (300)
   ```
7. **Clamping**: Ensure valid offset
8. **DOM Update**: Map and minimap updated
9. **Feedback**: Viewport instantly centered on clicked point

---

## 4. Application Initialization

The startup sequence when the page loads.

### Complete Sequence

```mermaid
sequenceDiagram
    participant Browser
    participant HTML
    participant JS as JavaScript
    participant WASM as WASM Binary
    participant Rust as Rust Code
    participant MapState
    participant DOM
    participant Console

    Note over Browser,Console: Page Load

    Browser->>HTML: Load index.html
    HTML->>HTML: Parse HTML/CSS
    HTML->>Browser: Render initial UI

    Browser->>JS: Execute inline script
    JS->>WASM: init() - Load WASM module
    WASM->>WASM: Instantiate binary
    WASM-->>JS: Module ready

    Note over Browser,Console: WASM Initialization

    JS->>Rust: initialize_map()
    Rust->>MapState: Create new MapState
    Note over MapState: offset_x = 0<br/>offset_y = 0<br/>viewport = 800x600<br/>world = 3000x3000

    Rust->>DOM: Set initial cursor='grab'
    Rust->>Console: Log "Map initialized"
    Console-->>DOM: Display in footer

    Note over Browser,Console: Event Binding

    JS->>DOM: Add mousedown listener
    JS->>DOM: Add mousemove listener
    JS->>DOM: Add mouseup listener
    JS->>DOM: Add keydown listener
    JS->>DOM: Add minimap click listener

    DOM-->>Console: Log "Map initialized. Use mouse to drag..."
    Console-->>Browser: Ready for interaction
```

### Initialization Flow

```mermaid
flowchart TD
    Start[Page Load] --> LoadHTML[Load HTML]
    LoadHTML --> ParseCSS[Parse Inline CSS]
    ParseCSS --> RenderUI[Render Initial UI]

    RenderUI --> LoadJS[Execute JavaScript]
    LoadJS --> LoadWASM[Load WASM Binary]
    LoadWASM --> InstantiateWASM[Instantiate WASM Module]

    InstantiateWASM --> InitMap[Call initialize_map]
    InitMap --> CreateState[Create MapState]
    CreateState --> SetCursor[Set Cursor: 'grab']

    SetCursor --> BindEvents[Bind Event Listeners]
    BindEvents --> MouseEvents[Mouse: down, move, up]
    BindEvents --> KeyEvents[Keyboard: keydown]
    BindEvents --> MinimapEvents[Minimap: click]

    MouseEvents --> Ready[Application Ready]
    KeyEvents --> Ready
    MinimapEvents --> Ready

    Ready --> WaitUser[Wait for User Input]

    style CreateState fill:#ffcccc
    style BindEvents fill:#ccffcc
    style Ready fill:#99ff99
```

### Step-by-Step

1. **Browser Load**: Parse HTML and CSS
2. **Initial Render**: Display UI with no interactions
3. **JavaScript Execution**: Inline script runs
4. **WASM Loading**:
   - `init()` fetches WASM binary
   - Binary instantiated
   - Rust functions exposed to JavaScript
5. **State Initialization**:
   - `initialize_map()` called
   - `MapState` created with defaults
   - Cursor set to `grab`
6. **Event Binding**:
   - Mouse events: `mousedown`, `mousemove`, `mouseup`
   - Keyboard events: `keydown`
   - Minimap events: `click`
7. **Ready State**:
   - Console message: "Map initialized..."
   - Application ready for user interaction

---

## 5. Complete User Session Flow

A typical user session combining multiple interactions.

```mermaid
stateDiagram-v2
    [*] --> PageLoad
    PageLoad --> WASMInit: Load WASM
    WASMInit --> Idle: initialize_map()

    Idle --> Dragging: Mouse Down
    Idle --> Panning: Key Press
    Idle --> Jumping: Minimap Click

    Dragging --> Dragging: Mouse Move
    Dragging --> Idle: Mouse Up

    Panning --> Idle: Pan complete
    Jumping --> Idle: Center complete

    state Idle {
        [*] --> WaitingForInput
        WaitingForInput --> WaitingForInput
    }

    state Dragging {
        [*] --> UpdateViewport
        UpdateViewport --> UpdateViewport: Mouse move
    }

    note right of Dragging
        DRAG_STATE exists
        Cursor: grabbing
        Frequent DOM updates
    end note

    note right of Idle
        DRAG_STATE is None
        Cursor: grab
        No updates
    end note
```

### Example Session

```mermaid
sequenceDiagram
    actor User
    participant App as RTS Monitor

    Note over User,App: Session Start
    User->>App: Open page
    App-->>User: Display UI
    App-->>User: "Map initialized"

    Note over User,App: Explore via Drag
    User->>App: Drag map to SERVER-BETA
    App-->>User: Viewport pans smoothly
    App-->>User: "Viewport: (1200, 0)"

    Note over User,App: Fine-tune with Keyboard
    User->>App: Press Down arrow 3x
    App-->>User: Map pans down 150px
    App-->>User: "Panned down"

    Note over User,App: Jump to Different Area
    User->>App: Click minimap bottom-left
    App-->>User: Viewport jumps instantly
    App-->>User: "Centered on (500, 2500)"

    Note over User,App: Explore New Area
    User->>App: Drag around
    App-->>User: Viewport updates
    App-->>User: Minimap tracks position

    Note over User,App: Session End
    User->>App: Close tab
```

---

## Performance Characteristics

### Event Frequency

| Interaction | Frequency | Updates/Second |
|-------------|-----------|----------------|
| Mouse Drag | Continuous | 60+ (mousemove) |
| Keyboard Hold | Continuous | ~10 (key repeat) |
| Minimap Click | Discrete | 1 (single event) |

### Update Pipeline Performance

```
Mouse Move Event
  ↓ < 1ms - Event capture
JavaScript Dispatch
  ↓ < 1ms - wasm_bindgen call
Rust Handler
  ↓ < 1ms - State update + clamp
DOM Update
  ↓ < 16ms - Browser render (60fps)
Visual Feedback
```

**Total Latency**: < 20ms (smooth 60fps)

### Optimization Strategies

1. **Minimal State**: Only `MapState` and `DragState`
2. **Direct DOM Updates**: No virtual DOM overhead
3. **Efficient Clamping**: Single pass per update
4. **Batched Updates**: State + DOM together
5. **No Re-renders**: Only transform updates

---

## Error Handling

### Missing Elements

If DOM elements not found:
```rust
if let Some(element) = document().get_element_by_id("map-content") {
    // Update element
} else {
    console::log_1(&"Warning: map-content not found".into());
}
```

### Invalid State

If state not initialized:
```rust
MAP_STATE.with(|state| {
    if let Some(map_state) = state.borrow().as_ref() {
        // Use state
    } else {
        console::log_1(&"Error: MapState not initialized".into());
    }
});
```

### Boundary Violations

Automatically handled by `clamp_offset()`:
```rust
self.offset_x = self.offset_x.max(0.0).min(max_x);
self.offset_y = self.offset_y.max(0.0).min(max_y);
```

---

## Related Pages

- [[Event Handling]] - Detailed event handler implementation
- [[State Management]] - State structures and transitions
- [[UI Components]] - Components involved in interactions
- [[Architecture Overview]] - System-level architecture

---

*Last Updated: 2025-11-18*
