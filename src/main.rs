use std::env;
use std::io::Result;

mod route_utils; // Import the route_utils module
mod interface_utils;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize Daemon
    println!("Daemon Initialized");

    // Parse command-line arguments for 'tun_device_name' and 'tun_device_ip'
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Usage: {} <tun_device_name> <tun_device_ip>", args[0]);
        return Ok(());
    }
    let tun_device_name = &args[1];
    let tun_device_ip = &args[2];

    // Check Tun Device Existence
    if route_utils::check_tun_device_exists(tun_device_name).await? {
        // Store Old Default Route
        let old_route = route_utils::store_old_default_route().await?;

        // Get TUN Device's Old Gateway IP
        let tun_old_gateway_ip = route_utils::get_tun_gateway(tun_device_name).await?;

        // Set Specific Route
        route_utils::set_specific_route(tun_device_ip, &old_route).await?;

        // Set Default Route
        route_utils::set_default_route(&tun_old_gateway_ip).await?;

        // Monitor Tun Device
        let should_revert = route_utils::monitor_tun_device(tun_device_name).await?;

        if should_revert {
            // Revert to Old Default Route
            route_utils::revert_to_old_default_route(&old_route).await?;

            // Remove Specific Route
            route_utils::remove_specific_route(tun_device_ip).await?;
        }
    } else {
        println!("Tun device does not exist. Exiting...");
    }

    // Stop
    println!("Daemon Stopped");

    Ok(())
}
