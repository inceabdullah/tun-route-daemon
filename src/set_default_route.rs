use net_route::{Route, Handle};

pub async fn set(old_gateway_ip: &str) -> std::io::Result<()> {
    let handle = Handle::new()?;
    let route = Route::new("0.0.0.0".parse().unwrap(), 0)
        .with_gateway(old_gateway_ip.parse().unwrap());

    println!("Setting default route via old gateway IP: {:?}", route);
    handle.add(&route).await
}
