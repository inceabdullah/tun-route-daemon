use net_route::Handle;
use crate::route_utils;
use crate::interface_utils::get_interface_name_from_index;


pub async fn get_iface() -> std::io::Result<u32> {
    let handle = Handle::new()?;
    let routes = handle.list().await?;

    let default_route = routes.iter().find(|route| {
        route.destination.is_unspecified() && route.prefix == 0
    });

    println!("default_route: {:#?}", default_route);

    match default_route {
        Some(route) => {
            if let Some(ifindex) = route.ifindex {
                println!("Current default ifindex is {}", ifindex);
                Ok(ifindex)
            } else {
                println!("No ifindex found for the default route.");
                Ok(0 as u32)
            }
        },
        None => {
            println!("No default route found.");
            Ok(0 as u32)
        }
    }
}

pub async fn get_iface_name() -> String {
    let ifindex = route_utils::def::get_iface().await.unwrap();
    match get_interface_name_from_index(ifindex) {
        Some(ifname) => ifname,
        None => String::new(),
    }
}
