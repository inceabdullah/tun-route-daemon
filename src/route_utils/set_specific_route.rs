use net_route::{Route, Handle};
use std::net::IpAddr;

pub async fn set(tun_device_ip: IpAddr, old_route: IpAddr) -> std::io::Result<()> {
    let handle = Handle::new()?;
    let route = Route::new(tun_device_ip, 32)
        .with_gateway(old_route);

    println!("Setting specific route: {:?}", route);
    handle.add(&route).await
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;
    use tokio::test;

    #[test]
    async fn test_set_specific_route() -> std::io::Result<()> {
        // Dummy data for testing
        let tun_device_ip = IpAddr::from_str("192.168.2.2").unwrap();
        let old_route =  IpAddr::from_str("192.168.2.1").unwrap();



        // Call the set function
        let result = set(tun_device_ip, old_route).await;

        // Check if the function executes without errors
        assert!(result.is_ok(), "Setting specific route failed");

        Ok(())
    }
}
