use std::env;
use clap::{Parser, Command, Arg, ValueEnum};
use anyhow::Result;
use std::net::IpAddr;
use std::convert::From;


mod route_utils;
mod interface_utils;
mod nft_utils;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(value_enum)]
    mode: Mode,

    #[clap(long)]
    tun_device_name: Option<String>,

    #[clap(long)]
    tun_device_ip: Option<IpAddr>,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Mode {
    Tun,
    Masq,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize Daemon
    println!("Daemon Initialized");

    // Parse command-line arguments
    let args = Args::parse(); 

    // Check the mode
    let mode = args.mode;

    match mode {
        Mode::Tun => {
    // Check Tun Device Existence
    let tun_device_name = &args.tun_device_name.unwrap();
    let tun_device_ip = args.tun_device_ip.unwrap();
    if route_utils::check_tun_device::exists(tun_device_name).await? {
        // Store Old Default Route
        let old_route = route_utils::store_old_route::get().await?;

    // Get TUN Device's Old Gateway IP
    let tun_old_gateway_ip = route_utils::get_tun_gateway::get(tun_device_name).await?;

    // Set Specific Route
    route_utils::set_specific_route::set(tun_device_ip, old_route).await?;

    // Set Default Route
    if let Some(tun_old_gateway_ip_str) = &tun_old_gateway_ip {
        route_utils::set_default_route::set(tun_old_gateway_ip_str).await?;
    } else {
        println!("No old gateway IP found for TUN device. Skipping setting default route.");
    }

        // Monitor Tun Device
        let should_revert = route_utils::monitor_tun_device::monitor(tun_device_name).await?;

        if should_revert {
            // Revert to Old Default Route
            route_utils::revert_old_route::revert(old_route).await?;

            // Remove Specific Route
            route_utils::remove_specific_route::remove(tun_device_ip).await?;
        }
    } else {
        println!("Tun device does not exist. Exiting...");
    }

    // Stop
    println!("Daemon Stopped");

    Ok(())
        }
        Mode::Masq => {
            nft_utils::add_clean_masq_for_def().await;
            return Ok(());
        }
        _ => unreachable!(),
    }



}
