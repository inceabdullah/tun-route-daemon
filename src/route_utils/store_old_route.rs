use net_route::Handle;
use std::{net::IpAddr, str::FromStr};

pub async fn get() -> std::io::Result<IpAddr> {
    let handle = Handle::new()?;
    let routes = handle.list().await?;

    let default_route = routes.iter().find(|route| {
        route.destination.is_unspecified() && route.prefix == 0
    });

    match default_route {
        Some(route) => {
            if let Some(gateway) = route.gateway {
                println!("Current default route is via gateway: {}", gateway);
                Ok(gateway)
            } else {
                println!("No gateway found for the default route.");
                Ok(IpAddr::from_str("").unwrap())
            }
        },
        None => {
            println!("No default route found.");
            Ok(IpAddr::from_str("").unwrap())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::test;

    #[test]
    async fn test_get_default_route() -> std::io::Result<()> {
        let result: IpAddr = get().await?;
        
        // Assert that the function returns a valid IP address
        assert!(result.is_ipv4(), "The returned value is not a valid IP address");

        Ok(())
    }
}

