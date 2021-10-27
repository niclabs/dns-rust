pub struct ResolverQuery {
    sname: String,
    stype: u16,
    sclass: u16,
    slist: Vec<ResourceRecord>,
    cache: HashMap<String, ResourceRecord>,
}
