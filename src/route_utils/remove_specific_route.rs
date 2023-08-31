use net_route::{Route, Handle};
use std::net::IpAddr;


pub async fn remove(tun_device_ip: IpAddr) -> std::io::Result<()> {
    let handle = Handle::new()?;
    let route = Route::new(tun_device_ip, 32);

    println!("Removing specific route: {:?}", route);
    handle.delete(&route).await
}
