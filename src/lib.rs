use wasm_bindgen::prelude::*;
use web_sys::{console, window, Document, Element, HtmlElement, MouseEvent};
use std::cell::RefCell;
use std::rc::Rc;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

// Map state management
#[derive(Clone)]
pub struct MapState {
    pub map_width: f64,
    pub map_height: f64,
    pub viewport_width: f64,
    pub viewport_height: f64,
    pub viewport_x: f64,
    pub viewport_y: f64,
}

impl MapState {
    pub fn new() -> Self {
        MapState {
            map_width: 2400.0,
            map_height: 1800.0,
            viewport_width: 800.0,
            viewport_height: 600.0,
            viewport_x: 800.0,  // Start centered
            viewport_y: 600.0,
        }
    }

    pub fn clamp_viewport(&mut self) {
        self.viewport_x = self.viewport_x.max(0.0).min(self.map_width - self.viewport_width);
        self.viewport_y = self.viewport_y.max(0.0).min(self.map_height - self.viewport_height);
    }

    pub fn move_viewport(&mut self, dx: f64, dy: f64) {
        self.viewport_x += dx;
        self.viewport_y += dy;
        self.clamp_viewport();
    }

    pub fn set_viewport_center(&mut self, world_x: f64, world_y: f64) {
        self.viewport_x = world_x - (self.viewport_width / 2.0);
        self.viewport_y = world_y - (self.viewport_height / 2.0);
        self.clamp_viewport();
    }
}

impl Default for MapState {
    fn default() -> Self {
        Self::new()
    }
}

// Global map state
thread_local! {
    static MAP_STATE: Rc<RefCell<MapState>> = Rc::new(RefCell::new(MapState::new()));
}

// Coordinate transformation functions
pub fn screen_to_world(screen_x: f64, screen_y: f64, svg_width: f64, svg_height: f64) -> (f64, f64) {
    MAP_STATE.with(|state| {
        let state = state.borrow();

        // Convert screen coordinates to SVG coordinates
        let svg_x = (screen_x / svg_width) * 800.0;
        let svg_y = (screen_y / svg_height) * 600.0;

        // Reverse the isometric transformation
        let adjusted_x = svg_x - 400.0;
        let adjusted_y = svg_y - 100.0;

        // Apply inverse matrix transformation
        let det = 0.866 * 0.5 - 0.5 * (-0.866);
        let world_x = ((0.5 * adjusted_x) - (-0.866 * adjusted_y)) / det + state.viewport_x;
        let world_y = ((-0.5 * adjusted_x) + (0.866 * adjusted_y)) / det + state.viewport_y;

        (world_x, world_y)
    })
}

pub fn minimap_to_world(minimap_x: f64, minimap_y: f64) -> (f64, f64) {
    MAP_STATE.with(|state| {
        let state = state.borrow();
        let scale_x = state.map_width / 200.0;
        let scale_y = state.map_height / 150.0;
        (minimap_x * scale_x, minimap_y * scale_y)
    })
}

// DOM manipulation functions
fn get_document() -> Result<Document, JsValue> {
    window()
        .ok_or_else(|| JsValue::from_str("No window found"))?
        .document()
        .ok_or_else(|| JsValue::from_str("No document found"))
}

fn get_element_by_id(id: &str) -> Result<Element, JsValue> {
    get_document()?
        .get_element_by_id(id)
        .ok_or_else(|| JsValue::from_str(&format!("Element '{}' not found", id)))
}

fn update_map_transform() -> Result<(), JsValue> {
    MAP_STATE.with(|state| {
        let state = state.borrow();
        let map_content = get_element_by_id("map-content")?;

        let viewport_transform = format!(
            "translate({}, {})",
            -state.viewport_x,
            -state.viewport_y
        );
        let iso_transform = "matrix(1, 0.15, -0.3, 0.9, 200, 50)";
        let transform = format!("{} {}", viewport_transform, iso_transform);

        map_content.set_attribute("transform", &transform)?;
        update_minimap_viewport()?;
        Ok(())
    })
}

fn update_minimap_viewport() -> Result<(), JsValue> {
    MAP_STATE.with(|state| {
        let state = state.borrow();
        let viewport_rect = get_element_by_id("viewport-rect")?;

        let scale_x = 200.0 / state.map_width;
        let scale_y = 150.0 / state.map_height;

        viewport_rect.set_attribute("x", &(state.viewport_x * scale_x).to_string())?;
        viewport_rect.set_attribute("y", &(state.viewport_y * scale_y).to_string())?;
        viewport_rect.set_attribute("width", &(state.viewport_width * scale_x).to_string())?;
        viewport_rect.set_attribute("height", &(state.viewport_height * scale_y).to_string())?;

        Ok(())
    })
}

// Mouse drag state
thread_local! {
    static DRAG_STATE: RefCell<Option<DragState>> = const { RefCell::new(None) };
}

struct DragState {
    start_x: f64,
    start_y: f64,
    viewport_start_x: f64,
    viewport_start_y: f64,
}

// Mouse event handlers
#[wasm_bindgen]
pub fn handle_map_mousedown(event: MouseEvent) -> Result<(), JsValue> {
    if event.button() == 0 {
        let (viewport_x, viewport_y) = MAP_STATE.with(|state| {
            let state = state.borrow();
            (state.viewport_x, state.viewport_y)
        });

        DRAG_STATE.with(|drag| {
            *drag.borrow_mut() = Some(DragState {
                start_x: event.client_x() as f64,
                start_y: event.client_y() as f64,
                viewport_start_x: viewport_x,
                viewport_start_y: viewport_y,
            });
        });

        event.prevent_default();

        if let Ok(map_div) = get_element_by_id("main-map") {
            if let Some(html_element) = map_div.dyn_ref::<HtmlElement>() {
                let _ = html_element.class_list().add_1("dragging");
            }
        }
    }
    Ok(())
}

#[wasm_bindgen]
pub fn handle_map_mousemove(event: MouseEvent) -> Result<(), JsValue> {
    DRAG_STATE.with(|drag| {
        if let Some(ref drag_state) = *drag.borrow() {
            if event.buttons() == 1 {
                let dx = drag_state.start_x - event.client_x() as f64;
                let dy = drag_state.start_y - event.client_y() as f64;

                MAP_STATE.with(|state| {
                    let mut state = state.borrow_mut();
                    state.viewport_x = drag_state.viewport_start_x + dx;
                    state.viewport_y = drag_state.viewport_start_y + dy;
                    state.clamp_viewport();
                });

                let _ = update_map_transform();
            }
        }
    });
    Ok(())
}

#[wasm_bindgen]
pub fn handle_map_mouseup(event: MouseEvent) -> Result<(), JsValue> {
    if event.button() == 0 {
        DRAG_STATE.with(|drag| {
            *drag.borrow_mut() = None;
        });

        if let Ok(map_div) = get_element_by_id("main-map") {
            if let Some(html_element) = map_div.dyn_ref::<HtmlElement>() {
                let _ = html_element.class_list().remove_1("dragging");
            }
        }
    }
    Ok(())
}

#[wasm_bindgen]
pub fn handle_minimap_click_new(event: MouseEvent) -> Result<(), JsValue> {
    let target = event
        .target()
        .ok_or_else(|| JsValue::from_str("No event target"))?;
    let element = target
        .dyn_ref::<Element>()
        .ok_or_else(|| JsValue::from_str("Target is not an Element"))?;

    let rect = element.get_bounding_client_rect();
    let x = event.client_x() as f64 - rect.left();
    let y = event.client_y() as f64 - rect.top();

    // Convert to minimap coordinates (viewBox is 0 0 200 150)
    let minimap_x = (x / rect.width()) * 200.0;
    let minimap_y = (y / rect.height()) * 150.0;

    let (world_x, world_y) = minimap_to_world(minimap_x, minimap_y);

    MAP_STATE.with(|state| {
        let mut state = state.borrow_mut();
        state.set_viewport_center(world_x, world_y);
    });

    update_map_transform()?;
    handle_minimap_click(minimap_x, minimap_y);

    Ok(())
}

// Initialize map view
#[wasm_bindgen]
pub fn init_map_view() -> Result<(), JsValue> {
    update_map_transform()?;
    log_status("Distributed System Monitor initialized - Network topology loaded");
    Ok(())
}

// Helper function to format status messages
fn format_status_message(prefix: &str, details: &str) -> String {
    format!("{}: {}", prefix, details)
}

// Helper function to format coordinates
fn format_coordinates(x: f64, y: f64) -> String {
    format!("({:.0}, {:.0})", x, y)
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

#[wasm_bindgen]
pub fn log_status(message: &str) {
    console::log_1(&format!("System Monitor: {}", message).into());
}

#[wasm_bindgen]
pub fn handle_map_click(x: f64, y: f64) {
    // Determine which server/process was clicked based on coordinates
    let clicked_element = determine_clicked_element(x, y);
    let message = match clicked_element {
        Some(element) => format_status_message("Selected", &element),
        None => format_status_message("Network space clicked at", &format_coordinates(x, y)),
    };
    log_status(&message);
}

// Helper function to determine which element was clicked
fn determine_clicked_element(x: f64, y: f64) -> Option<String> {
    // Server locations (approximate)
    if distance(x, y, 400.0, 300.0) < 150.0 {
        Some("SERVER-ALPHA: 3 processes running (2 LLM, 1 MCP)".to_string())
    } else if distance(x, y, 2000.0, 300.0) < 150.0 {
        Some("SERVER-BETA: 4 processes running (2 LLM, 2 MCP)".to_string())
    } else if distance(x, y, 400.0, 1500.0) < 150.0 {
        Some("SERVER-GAMMA: 2 processes running (1 LLM, 1 MCP)".to_string())
    } else if distance(x, y, 2000.0, 1500.0) < 150.0 {
        Some("SERVER-DELTA: 3 processes running (2 LLM, 1 MCP)".to_string())
    } else if distance(x, y, 200.0, 900.0) < 120.0 {
        Some("WORKSTATION-1: 2 processes running (1 LLM, 1 MCP)".to_string())
    } else if distance(x, y, 2200.0, 900.0) < 120.0 {
        Some("WORKSTATION-2: 1 process running (1 MCP)".to_string())
    } else if distance(x, y, 1200.0, 900.0) < 50.0 {
        Some("ROUTER-1: Central routing node - 6 active connections".to_string())
    } else {
        None
    }
}

// Simple distance calculation
fn distance(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
    ((x1 - x2).powi(2) + (y1 - y2).powi(2)).sqrt()
}

#[wasm_bindgen]
pub fn handle_minimap_click(x: f64, y: f64) {
    let message = format_status_message("Minimap clicked at", &format_coordinates(x, y));
    log_status(&message);
}

#[wasm_bindgen]
pub fn handle_build_button(building_type: &str) {
    let message = match building_type {
        "deploy_llm" => "Deploying new LLM instance... Allocating GPU resources",
        "deploy_mcp" => "Deploying MCP server... Configuring network endpoints",
        "start_service" => "Starting selected service... Initializing connections",
        "stop_service" => "Stopping selected service... Gracefully shutting down",
        "restart_service" => "Restarting service... Preserving active connections",
        "scale_up" => "Scaling up resources... Adding compute nodes",
        "scale_down" => "Scaling down... Migrating workloads",
        "health_check" => "Running health checks... All systems operational",
        _ => "Unknown command",
    };
    log_status(message);
}

#[wasm_bindgen]
pub fn handle_research_button(tech: &str) {
    let message = match tech {
        "discover" => "Discovering MCP servers... Found 7 active endpoints",
        "configure" => "Opening protocol configuration... Current version: MCP v2.1",
        "security" => "Security settings: TLS 1.3 enabled, mTLS for critical services",
        "routing" => "Routing rules: Load-balanced, latency-optimized",
        "loadbalance" => "Load balancer: Round-robin with health checks enabled",
        "failover" => "Failover configured: 30s timeout, automatic recovery",
        "monitor" => "Monitoring traffic: 1.2 Gbps throughput, 15ms avg latency",
        _ => "Unknown configuration",
    };
    log_status(message);
}

#[wasm_bindgen]
pub fn handle_unit_command(command: &str) {
    let message = match command {
        "view_alerts" => "Active alerts: 2 warnings (high memory on BETA, MCP-3 slow response)",
        "clear_alerts" => "Clearing resolved alerts... 5 alerts cleared",
        "set_threshold" => "Threshold settings: CPU > 80%, Memory > 90%, Latency > 100ms",
        "export_logs" => "Exporting system logs... Last 24 hours exported to logs_2025_01_28.json",
        _ => "Unknown alert command",
    };
    log_status(message);
}

#[wasm_bindgen]
pub fn handle_resource_click(resource: &str) {
    let message = match resource {
        "cpu" => "CPU Usage: 47% average across all nodes (Peak: SERVER-BETA at 72%)",
        "memory" => "Memory: 12.3GB / 32GB total (38% utilization)",
        "network" => "Network: 1.2 Gbps throughput, 6 active connections",
        "gpu" => "GPU: 4 of 8 GPUs active (3 for LLMs, 1 for inference)",
        "servers" => "Servers: 12 online, 3 in maintenance mode",
        _ => "Unknown resource metric",
    };
    log_status(message);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_status_message() {
        let result = format_status_message("Test", "message");
        assert_eq!(result, "Test: message");
    }

    #[test]
    fn test_format_coordinates() {
        let result = format_coordinates(123.45, 678.90);
        assert_eq!(result, "(123, 679)");
    }

    #[test]
    fn test_format_coordinates_zero() {
        let result = format_coordinates(0.0, 0.0);
        assert_eq!(result, "(0, 0)");
    }

    #[test]
    fn test_format_coordinates_negative() {
        let result = format_coordinates(-10.5, -20.8);
        assert_eq!(result, "(-10, -21)");
    }

    #[test]
    fn test_map_state_new() {
        let state = MapState::new();
        assert_eq!(state.map_width, 2400.0);
        assert_eq!(state.map_height, 1800.0);
        assert_eq!(state.viewport_width, 800.0);
        assert_eq!(state.viewport_height, 600.0);
        assert_eq!(state.viewport_x, 800.0);
        assert_eq!(state.viewport_y, 600.0);
    }

    #[test]
    fn test_map_state_clamp_viewport() {
        let mut state = MapState::new();

        // Test clamping when viewport goes negative
        state.viewport_x = -100.0;
        state.viewport_y = -50.0;
        state.clamp_viewport();
        assert_eq!(state.viewport_x, 0.0);
        assert_eq!(state.viewport_y, 0.0);

        // Test clamping when viewport exceeds bounds
        state.viewport_x = 3000.0;
        state.viewport_y = 2000.0;
        state.clamp_viewport();
        assert_eq!(state.viewport_x, 2400.0 - 800.0); // map_width - viewport_width
        assert_eq!(state.viewport_y, 1800.0 - 600.0); // map_height - viewport_height
    }

    #[test]
    fn test_map_state_move_viewport() {
        let mut state = MapState::new();
        let initial_x = state.viewport_x;
        let initial_y = state.viewport_y;

        state.move_viewport(50.0, 100.0);
        assert_eq!(state.viewport_x, initial_x + 50.0);
        assert_eq!(state.viewport_y, initial_y + 100.0);

        // Test that move_viewport clamps
        state.move_viewport(-10000.0, -10000.0);
        assert_eq!(state.viewport_x, 0.0);
        assert_eq!(state.viewport_y, 0.0);
    }

    #[test]
    fn test_map_state_set_viewport_center() {
        let mut state = MapState::new();

        state.set_viewport_center(1200.0, 900.0);
        // viewport_x should be 1200 - 400 (half of viewport_width)
        // viewport_y should be 900 - 300 (half of viewport_height)
        assert_eq!(state.viewport_x, 800.0);
        assert_eq!(state.viewport_y, 600.0);

        // Test clamping when centering near edge
        state.set_viewport_center(100.0, 100.0);
        assert_eq!(state.viewport_x, 0.0); // Clamped to 0
        assert_eq!(state.viewport_y, 0.0); // Clamped to 0
    }

    #[test]
    fn test_distance() {
        assert_eq!(distance(0.0, 0.0, 3.0, 4.0), 5.0);
        assert_eq!(distance(1.0, 1.0, 1.0, 1.0), 0.0);
        assert_eq!(distance(-1.0, -1.0, 2.0, 3.0), 5.0);
    }
}

#[cfg(test)]
mod wasm_tests {
    use super::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn test_greet_wasm() {
        greet("WASM");
        // Note: This test will trigger an alert in the browser
        // In a real app, you'd implement the actual alert function
    }

    #[wasm_bindgen_test]
    fn test_handle_map_click_wasm() {
        handle_map_click(100.0, 200.0);
        // This test verifies the function doesn't panic
        // In a real app, you'd capture and verify the console output
    }

    #[wasm_bindgen_test]
    fn test_handle_build_button_wasm() {
        handle_build_button("barracks");
        // This test verifies the function doesn't panic with valid input
    }

    #[wasm_bindgen_test]
    fn test_handle_empty_string_wasm() {
        handle_build_button("");
        handle_research_button("");
        handle_unit_command("");
        handle_resource_click("");
        // Test edge case with empty strings
    }

    #[wasm_bindgen_test]
    fn test_coordinate_edge_cases_wasm() {
        handle_map_click(0.0, 0.0);
        handle_map_click(-1.0, -1.0);
        handle_map_click(f64::MAX, f64::MIN);
        // Test edge cases for coordinate handling
    }
}
