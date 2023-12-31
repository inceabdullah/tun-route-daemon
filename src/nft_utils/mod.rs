use crate::route_utils;

pub mod masq;

pub async fn add_clean_masq_for_def() {
    let ifname = route_utils::def::get_iface_name().await;
    println!("cleaning... {} masq", &ifname);
    masq::rm_masq_for_ifname(&ifname);
    println!("adding... {} masq", &ifname);
    masq::add_masq_for_ifname(&ifname);

    println!("Added {} masq", &ifname);
}
