use std::env;
use std::io::Result;

mod route_utils;
mod interface_utils;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize Daemon
    println!("Daemon Initialized");

    // Call an asynchronous function from your route_utils::def module
    let ifindex = route_utils::def::get_iface().await.unwrap();

    let ifname = route_utils::def::get_iface_name().await;
    println!("ifname: {:?}", ifname);
    return Ok(());

    // Parse command-line arguments for 'tun_device_name' and 'tun_device_ip'
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Usage: {} <tun_device_name> <tun_device_ip>", args[0]);
        return Ok(());
    }
    let tun_device_name = &args[1];
    let tun_device_ip = &args[2];

    // Check Tun Device Existence
    if route_utils::check_tun_device::exists(tun_device_name).await? {
        // Store Old Default Route
        let old_route = route_utils::store_old_route::get().await?;

    // Get TUN Device's Old Gateway IP
    let tun_old_gateway_ip = route_utils::get_tun_gateway::get(tun_device_name).await?;

    // Set Specific Route
    route_utils::set_specific_route::set(tun_device_ip, &old_route).await?;

    // Set Default Route
    if let Some(tun_old_gateway_ip_str) = &tun_old_gateway_ip {
        route_utils::set_default_route::set(tun_old_gateway_ip_str).await?;
    } else {
        println!("No old gateway IP found for TUN device. Skipping setting default route.");
    }

        // Monitor Tun Device
        let should_revert = route_utils::monitor_tun_device::monitor(tun_device_name).await?;

        if should_revert {
            // Revert to Old Default Route
            route_utils::revert_old_route::revert(&old_route).await?;

            // Remove Specific Route
            route_utils::remove_specific_route::remove(tun_device_ip).await?;
        }
    } else {
        println!("Tun device does not exist. Exiting...");
    }

    // Stop
    println!("Daemon Stopped");

    Ok(())
}
