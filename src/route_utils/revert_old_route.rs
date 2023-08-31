use net_route::{Route, Handle};
use std::{net::IpAddr, str::FromStr};

pub async fn revert(old_route: IpAddr) -> std::io::Result<()> {
    let handle = Handle::new()?;
    let route = Route::new(IpAddr::from_str("0.0.0.0").unwrap(), 0)
        .with_gateway(old_route);

    println!("Reverting to old default route: {:?}", route);
    handle.add(&route).await
}
