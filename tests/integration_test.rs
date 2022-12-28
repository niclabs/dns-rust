use std::{fs, thread, collections::HashMap};

use dns_rust::{client, resolver, config::RESOLVER_IP_PORT, config::{SBELT_ROOT_IPS, MASTER_FILES}, name_server::{zone_node::NSZone, master_file::MasterFile}};


/// Read a zone file and extract the host name to a vector.
fn get_host_names_from_zone_file(path: &str) -> Vec<String> {
        
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
fn validate_master_files() {
    for master_file in MASTER_FILES {
        let _validated_mf = MasterFile::from_file(master_file.to_string(), true);
    }
}

/// Test run_client() from client
#[test]
fn test_500000_cl_domains() {

    // Run resolver.
    thread::spawn(move || {
        resolver::run_resolver(RESOLVER_IP_PORT, SBELT_ROOT_IPS, HashMap::<u16, HashMap<String, NSZone>>::new());
    });

    // Get all host names from a zone file
    let host_names_vec: Vec<String> =  get_host_names_from_zone_file("CL-test.zone");

    for host_name in host_names_vec {
        client::run_client(host_name.as_str(), "TCP");
    }
}

#[test]
fn rfc1034_standar_queries_test_6_2_1() {
    client::run_client("test.uchile.cl", "TCP");

    // Run resolver.
    thread::spawn(move || {
        resolver::run_resolver(RESOLVER_IP_PORT, SBELT_ROOT_IPS, HashMap::<u16, HashMap<String, NSZone>>::new());
    });

}