pub mod zones;



#[derive (PartialEq, Debug)]
pub struct NameServer {
    zones: HashMap<String, Zone>, // Cada zona está asociada a un dominio.
    forwarders: Vec<String>, // Lista de servidores a los que se pueden delegar consultas.
}


pub fn new(forwarders: Vec<String>) -> Self {
    NameServer {
        zones: HashMap::new(),
        forwarders,
    }
}


pub fn add_zone(&mut self, zone: Zone) {
    self.zones.insert(zone.domain.clone(), zone);
}


pub fn resolve(&self, query: &Question) -> Option<Vec<ResourceRecord>> {
    // Buscar en las zonas locales
    if let Some(zone) = self.zones.get(&query.qname.get_name()) {
        return Some(zone.get_records(query.rrtype));
    }

    // Delegar a forwarders si no se encuentra
    self.forwarders.iter().find_map(|forwarder| {
        // Implementar consulta a forwarders
        None // Placeholder para forwarders
    })
}
