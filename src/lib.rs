use wasm_bindgen::prelude::*;
use web_sys::console;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
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
