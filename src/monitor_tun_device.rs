use futures::StreamExt;
use net_route::{Handle, RouteChange};
use crate::interface_utils::get_interface_name_from_index;  // Import the helper function

pub async fn monitor(tun_device_name: &str) -> std::io::Result<bool> {
    let handle = Handle::new()?;
    let stream = handle.route_listen_stream();

    futures::pin_mut!(stream);

    println!("Monitoring TUN device {}, press Ctrl+C to cancel...", tun_device_name);

    while let Some(route_change) = stream.next().await {
        println!("Route changed. RouteChange: {:?}", route_change);
        match route_change {
            RouteChange::Add(route) | RouteChange::Delete(route) | RouteChange::Change(route) => {
                println!("Route change detected: {:?}", route);
                if let Some(ifindex) = route.ifindex {
                    let interface_name = get_interface_name_from_index(ifindex).unwrap_or_default();  // Use the helper function
                    println!("interface_name and ifindex: {}, {:?}", interface_name, ifindex);

                    if interface_name == tun_device_name {
                        println!("Route change detected for TUN device: {:?}", route);
                        return Ok(true);
                    }
                }
            }
        }
    }

    Ok(false)
}
