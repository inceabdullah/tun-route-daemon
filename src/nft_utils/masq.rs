use std::process::Command;
use regex::Regex;

static CHAIN_NAME: &str = "tun_route_daemon";


pub fn rm_masq_for_ifname(ifname: &str) {

    let chain_name = "postrouting_tun_route_daemon";
    
    // Run the command and capture its output
    let output = Command::new("nft")
        .args(&["-a", "list", "chain", "ip", "nat", &chain_name])
        .output()
        .expect("Failed to execute command");

    // Convert output bytes to string
    let output_str = String::from_utf8_lossy(&output.stdout);

    println!("output_str: {:#?}", output_str);
    
    // Define a regular expression to match handle IDs
    let re = Regex::new(&format!(r#"oifname "{}" masquerade"#, ifname)).unwrap();
    
    // Collect handle IDs in a vector
    let mut handle_ids = Vec::new();
    for line in output_str.lines() {
        if re.is_match(line) {
            if let Some(capture) = line.rsplitn(2, " # handle ").next() {
                if let Ok(handle_id) = capture.parse::<usize>() {
                    handle_ids.push(handle_id);
                }
            }
        }
    }
    println!("handle_ids: {:#?}", handle_ids);

    
    // Iterate through the handle IDs and run nft delete commands
    for handle_id in handle_ids {
        let cmd_result = Command::new("nft")
            .args(&["delete", "rule", "nat", &chain_name, &format!("handle {}", handle_id)])
            .output();
        
        match cmd_result {
            Ok(_) => println!("Deleted rule with handle {}", handle_id),
            Err(e) => eprintln!("Error deleting rule with handle {}: {:?}", handle_id, e),
        }
    }
}

pub fn add_masq_for_ifname(ifname: &str) {

    let chain_name_prerouting = format!("prerouting_{}", CHAIN_NAME);
    let chain_name_postrouting = format!("postrouting_{}", CHAIN_NAME);
    
    // nft add table ip nat
    Command::new("nft")
        .args(&["add", "table", "ip", "nat"])
        .status()
        .expect("Failed to execute 'nft add table' command");

    // nft add chain ip nat $chain_name_prerouting { type nat hook prerouting priority 0; policy accept; }
    Command::new("nft")
        .args(&["add", "chain", "ip", "nat", &chain_name_prerouting, "{type nat hook prerouting priority 0; policy accept;}"])
        .status()
        .expect("Failed to execute 'nft add chain' command");

    // nft add chain ip nat $chain_name_postrouting { type nat hook postrouting priority 100; policy accept; }
    Command::new("nft")
        .args(&["add", "chain", "ip", "nat", &chain_name_postrouting, "{ type nat hook postrouting priority 100; policy accept;}"])
        .status()
        .expect("Failed to execute 'nft add chain' command");

    // nft add rule ip nat $chain_name_postrouting oifname "${ifname}" masquerade
    Command::new("nft")
        .args(&["add", "rule", "ip", "nat", &chain_name_postrouting, &format!("oifname \"{}\" masquerade", ifname)])
        .status()
        .expect("Failed to execute 'nft add rule' command");
}
