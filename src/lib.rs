// Re-export interface_utils module
pub mod interface_utils;

// Re-export the route_utils module
pub mod route_utils {
    pub mod check_tun_device;
    pub mod get_tun_gateway;
    pub mod monitor_tun_device;
    pub mod remove_specific_route;
    pub mod revert_old_route;
    pub mod set_default_route;
    pub mod set_specific_route;
    pub mod store_old_route;

    use std::io::Result;

    pub async fn run_daemon(tun_device_name: &str, tun_device_ip: &str) -> Result<()> {
        // Initialize Daemon
        println!("Daemon Initialized");

        // Check Tun Device Existence
        if check_tun_device::exists(tun_device_name).await? {
            // Store Old Default Route
            let old_route = store_old_route::get().await?;

            // Get TUN Device's Old Gateway IP
            let tun_old_gateway_ip = get_tun_gateway::get(tun_device_name).await?;

            // Set Specific Route
            set_specific_route::set(tun_device_ip, &old_route).await?;

            // Set Default Route
            if let Some(tun_old_gateway_ip_str) = &tun_old_gateway_ip {
                set_default_route::set(tun_old_gateway_ip_str).await?;
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
}
