use futures::StreamExt;
use net_route::{Handle, RouteChange};
use std::collections::HashMap;
use crate::interface_utils::get_interface_name_from_index;

pub async fn monitor(tun_device_name: &str) -> std::io::Result<bool> {
    let handle = Handle::new()?;
    let stream = handle.route_listen_stream();

    futures::pin_mut!(stream);

    println!("Monitoring TUN device {}, press Ctrl+C to cancel...", tun_device_name);

    let mut ifindex_to_interface: HashMap<u32, String> = HashMap::new(); // Store ifindex to interface_name mapping here

    // Populate the ifindex_to_interface mapping before entering the loop
    let routes = handle.list().await?;
    for route in routes {
        if let Some(ifindex) = route.ifindex {
            let interface_name = get_interface_name_from_index(ifindex).unwrap_or_default();  // Use the helper function
            ifindex_to_interface.insert(ifindex, interface_name);
        }
    }

    while let Some(route_change) = stream.next().await {
        println!("Route changed. RouteChange: {:?}", route_change);
        match route_change {
            RouteChange::Add(route) | RouteChange::Delete(route) | RouteChange::Change(route) => {
                println!("Route change detected: {:?}", route);
                if let Some(ifindex) = route.ifindex {
                    if let Some(interface_name) = ifindex_to_interface.get(&ifindex) {
                        println!("tun_device_name, interface_name and ifindex: {}, {}, {}", tun_device_name, interface_name, ifindex);
                        if interface_name == tun_device_name {
                            println!("Route change detected for TUN device: {:?}", route);
                            return Ok(true);
                        }
                    }
                }
            }
        }
    }

    Ok(false)
}
