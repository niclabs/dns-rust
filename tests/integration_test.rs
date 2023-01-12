use std::{fs, thread, collections::HashMap, time};

use dns_rust::{client, resolver, config::RESOLVER_IP_PORT, config::{SBELT_ROOT_IPS, MASTER_FILES}, name_server::{zone::NSZone, master_file::MasterFile}};


/// Gets a Vec of host names from a external file
fn get_host_names_from_zone_file(path: &str) -> Vec<String> {
        
    // Read file content
    let contents = fs::read_to_string(path)
    .expect("Should have been able to read the file"); 
    
    // Split file content
    let splitted_content: Vec<&str> = contents.split("\n").collect();

    // Create a vec of host names
    let mut host_names_vec: Vec<String> = Vec::new();
    // Extract host names from file
    for host_name in splitted_content {
        host_names_vec.push(host_name.to_string())
    } 

    // Return all host names from file
    return host_names_vec
}


#[test]
fn validate_rfc_master_files() {
    for (master_file, master_file_origin) in MASTER_FILES {
        let _validated_mf = MasterFile::from_file(master_file.to_string(),master_file_origin.to_string(), true);
    }
}

/// Robustness test
#[test]
fn test_500000_cl_domains() {

    // Run resolver.
    let resolver = thread::spawn(move || {
        resolver::run_resolver(RESOLVER_IP_PORT, SBELT_ROOT_IPS, HashMap::<u16, HashMap<String, NSZone>>::new());
    });

    // Get all host names from a file
    let host_names_vec: Vec<String> =  get_host_names_from_zone_file("tests/test_files/test_domains_names.txt");    
    for host_name in host_names_vec{
        println!("Domain name: {}", host_name);
        let mut dnsmessage = client::create_client_query(host_name.as_str(), "TCP" , 1 , 1);
        dnsmessage.print_dns_message()
    }


}

#[test]
fn rfc1034_standard_queries_test_6_2_1() {
    
    // Run resolver.
    thread::spawn(move || {
        resolver::run_resolver(RESOLVER_IP_PORT, SBELT_ROOT_IPS, HashMap::<u16, HashMap<String, NSZone>>::new());
    });

    thread::sleep(time::Duration::from_millis(40));
    client::create_client_query("dcc.uchile.cl", "TCP",1,1);

}