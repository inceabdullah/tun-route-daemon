use net_route::{Route, Handle};

pub async fn set(tun_device_ip: &str, old_route: &str) -> std::io::Result<()> {
    let handle = Handle::new()?;
    let route = Route::new(tun_device_ip.parse().unwrap(), 32)
        .with_gateway(old_route.parse().unwrap());

    println!("Setting specific route: {:?}", route);
    handle.add(&route).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::test;

    #[test]
    async fn test_set_specific_route() -> std::io::Result<()> {
        // Dummy data for testing
        let tun_device_ip = "192.168.2.2";
        let old_route = "192.168.2.1";

        // Call the set function
        let result = set(tun_device_ip, old_route).await;

        // Check if the function executes without errors
        assert!(result.is_ok(), "Setting specific route failed");

        Ok(())
    }
}
