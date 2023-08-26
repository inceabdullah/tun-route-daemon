use net_route::{Route, Handle};

pub async fn revert(old_route: &str) -> std::io::Result<()> {
    let handle = Handle::new()?;
    let route = Route::new("0.0.0.0".parse().unwrap(), 0)
        .with_gateway(old_route.parse().unwrap());

    println!("Reverting to old default route: {:?}", route);
    handle.add(&route).await
}
