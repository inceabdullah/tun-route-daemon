use net_route::Handle;
use crate::interface_utils::get_interface_name_from_index;  // Import the helper function

pub async fn get(tun_device_name: &str) -> std::io::Result<Option<String>> {
    let handle = Handle::new()?;
    let routes = handle.list().await?;
    println!("routes: {:?}", routes);
    for route in routes {
        if let Some(ifindex) = route.ifindex {
            let interface_name = get_interface_name_from_index(ifindex).unwrap_or_default();  // Use the helper function

            if interface_name == tun_device_name {
                if let Some(gateway) = route.gateway {
                    return Ok(Some(gateway.to_string()));
                }
            }
        }
    }

    Ok(None)
}
