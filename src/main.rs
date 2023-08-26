use std::env;
use std::io::Result;

mod check_tun_device;
mod store_old_route;
mod set_specific_route;
mod set_default_route;
mod monitor_tun_device;
mod revert_old_route;
mod remove_specific_route;
mod get_tun_gateway;
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
    if check_tun_device::exists(tun_device_name).await? {
        // Store Old Default Route
        let old_route = store_old_route::get().await?;

        // Get TUN Device's Old Gateway IP
        let tun_old_gateway_ip = get_tun_gateway::get(tun_device_name).await?;

        // Set Specific Route
        set_specific_route::set(tun_device_ip, &old_route).await?;

        // Set Default Route
        if let Some(gateway_ip) = tun_old_gateway_ip {
            set_default_route::set(&gateway_ip.to_string()).await?;
        } else {
            println!("No old gateway IP found for TUN device. Skipping setting default route.");
        }

        // Monitor Tun Device
        let should_revert = monitor_tun_device::monitor(tun_device_name).await?;

        if should_revert {
            // Revert to Old Default Route
            revert_old_route::revert(&old_route).await?;

            // Remove Specific Route
            remove_specific_route::remove(tun_device_ip).await?;
        }
    } else {
        println!("Tun device does not exist. Exiting...");
    }

    // Stop
    println!("Daemon Stopped");

    Ok(())
}
