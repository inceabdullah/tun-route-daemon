extern crate pnet;

use pnet::datalink;

pub fn get_interface_name_from_index(ifindex: u32) -> Option<String> {
    for iface in datalink::interfaces() {
        if iface.index == ifindex {
            return Some(iface.name);
        }
    }
    None
}
