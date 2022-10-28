use core::time;
use std::{fs, thread};

use dns_rust::client;

fn get_host_names_from_zone_file(path: &str) -> Vec<String> {
    // Read a zone file and extract the host name to a vector.
        
    // Read zone file content
    let contents = fs::read_to_string(path)
    .expect("Should have been able to read the zone file"); 
    
    // Split file content
    let splitted_content: Vec<&str> = contents.split("\n").collect();


    // Create a vec of host names
    let mut host_names_vec: Vec<String> = Vec::new();
    // Extract host names from zone file
    for host_info in splitted_content {
        let host_info_splitted: Vec<&str> = host_info.split("\t").collect();
        let mut host_name = host_info_splitted[0].to_string();
        // Remove last dot
        host_name.pop();
        host_names_vec.push(host_name)
    } 
    host_names_vec.dedup();

    // Return all host names from zone file
    return host_names_vec
}

#[test]
fn run_client_test() {
    // Test run_client() from client
    let host_names_vec: Vec<String> =  get_host_names_from_zone_file("tests/zonesTest.zone");

    for host_name in host_names_vec {
        client::run_client(host_name.as_str(), "TCP");
    }
}