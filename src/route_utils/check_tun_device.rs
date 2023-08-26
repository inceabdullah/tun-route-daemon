use crate::interface_utils::get_interface_name_from_index;

use net_route::Handle;

pub async fn exists(tun_device_name: &str) -> std::io::Result<bool> {
    // Create a new handle for interacting with the routing table
    let handle = Handle::new()?;
    
    // Fetch all routes
    let routes = handle.list().await?;

    // Check if the TUN device exists among them
    let tun_exists = routes.iter().any(|route| {
        if let Some(ifindex) = route.ifindex {
            let interface_name = get_interface_name_from_index(ifindex).unwrap_or_default();
            interface_name == tun_device_name
        } else {
            false
        }
    });

    // Log the result
    if tun_exists {
        println!("TUN device {} exists.", tun_device_name);
    } else {
        println!("TUN device {} does not exist.", tun_device_name);
    }

    Ok(tun_exists)
}


// #[cfg(test)]
// mod tests {
//     use super::*;
//     use tokio::test;

//     #[test]
//     async fn test_exists() -> std::io::Result<()> {
//         // Use a common device name that should exist on most systems
//         let device_name = "lo";

//         let result = exists(device_name).await?;

//         // Check if the function correctly identifies that the device exists
//         assert!(result, "The device should exist");

//         Ok(())
//     }
// }
