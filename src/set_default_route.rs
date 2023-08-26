use net_route::{Route, Handle};
use std::net::IpAddr;

pub async fn set(old_gateway_ip: Option<&str>) -> std::io::Result<()> {
    if let Some(gateway_ip) = old_gateway_ip {
        // Remove Default Route
        remove_default_route().await?;
        
        // Add Default Route
        add_default_route(gateway_ip).await?;
    } else {
        println!("No old gateway IP found for TUN device. Skipping setting default route.");
    }
    
    Ok(())
}

pub async fn remove_default_route() -> std::io::Result<()> {
    let handle = Handle::new()?;
    let default_route = IpAddr::V4("0.0.0.0".parse().unwrap());
    
    println!("Removing default route: {:?}", default_route);
    handle.delete(default_route).await
}

pub async fn add_default_route(old_gateway_ip: &str) -> std::io::Result<()> {
    let handle = Handle::new()?;
    let route = Route::new("0.0.0.0".parse().unwrap(), 0)
        .with_gateway(old_gateway_ip.parse().unwrap());

    println!("Adding default route via old gateway IP: {:?}", route);
    handle.add(&route).await
}
