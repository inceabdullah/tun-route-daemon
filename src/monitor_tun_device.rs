use futures::StreamExt;
use net_route::{Handle, RouteChange};
use crate::interface_utils::get_interface_name_from_index;  // Import the helper function

pub async fn monitor(tun_device_name: &str) -> std::io::Result<bool> {
    let handle = Handle::new()?;
    let stream = handle.route_listen_stream();

    futures::pin_mut!(stream);

    println!("Monitoring TUN device {}, press Ctrl+C to cancel...", tun_device_name);

    while let Some(route_change) = stream.next().await {
        match route_change {
            RouteChange::Add(route) | RouteChange::Delete(route) | RouteChange::Change(route) => {
                if let Some(ifindex) = route.ifindex {
                    let interface_name = get_interface_name_from_index(ifindex).unwrap_or_default();  // Use the helper function

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
