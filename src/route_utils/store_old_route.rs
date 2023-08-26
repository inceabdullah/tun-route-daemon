use net_route::Handle;

pub async fn get() -> std::io::Result<String> {
    let handle = Handle::new()?;
    let routes = handle.list().await?;

    let default_route = routes.iter().find(|route| {
        route.destination.is_unspecified() && route.prefix == 0
    });

    match default_route {
        Some(route) => {
            if let Some(gateway) = route.gateway {
                println!("Current default route is via gateway: {}", gateway);
                Ok(gateway.to_string())
            } else {
                println!("No gateway found for the default route.");
                Ok("".to_string())
            }
        },
        None => {
            println!("No default route found.");
            Ok("".to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::test;

    #[test]
    async fn test_get_default_route() -> std::io::Result<()> {
        let result = get().await?;
        
        // Check if the result is an IP address (very basic validation)
        let is_ip: bool = result.parse::<std::net::IpAddr>().is_ok();
        
        // Assert that the function returns a valid IP address
        assert!(is_ip, "The returned value is not a valid IP address");

        Ok(())
    }
}

