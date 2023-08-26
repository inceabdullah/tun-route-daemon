use net_route::{Handle, RouteChange};
use crate::interface_utils::get_interface_name_from_index;

mod check_tun_device;
mod store_old_route;
mod set_specific_route;
mod set_default_route;
mod monitor_tun_device;
mod revert_old_route;
mod remove_specific_route;
mod get_tun_gateway;
mod interface_utils;

pub async fn check_tun_device_exists(tun_device_name: &str) -> std::io::Result<bool> {
    check_tun_device::exists(tun_device_name).await
}

pub async fn store_old_default_route() -> std::io::Result<String> {
    store_old_route::get().await
}

pub async fn get_tun_gateway(tun_device_name: &str) -> std::io::Result<Option<String>> {
    get_tun_gateway::get(tun_device_name).await
}

pub async fn set_specific_route(tun_device_ip: &str, old_route: &str) -> std::io::Result<()> {
    set_specific_route::set(tun_device_ip, old_route).await
}

pub async fn set_default_route(old_gateway_ip: &Option<String>) -> std::io::Result<()> {
    set_default_route::set(old_gateway_ip).await
}

pub async fn monitor_tun_device(tun_device_name: &str) -> std::io::Result<bool> {
    monitor_tun_device::monitor(tun_device_name).await
}

pub async fn revert_to_old_default_route(old_route: &str) -> std::io::Result<()> {
    revert_old_route::revert(old_route).await
}

pub async fn remove_specific_route(tun_device_ip: &str) -> std::io::Result<()> {
    remove_specific_route::remove(tun_device_ip).await
}
