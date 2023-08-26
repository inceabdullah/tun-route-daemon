use net_route::{Route, Handle};

pub async fn remove(tun_device_ip: &str) -> std::io::Result<()> {
    let handle = Handle::new()?;
    let route = Route::new(tun_device_ip.parse().unwrap(), 32);

    println!("Removing specific route: {:?}", route);
    handle.delete(&route).await
}
