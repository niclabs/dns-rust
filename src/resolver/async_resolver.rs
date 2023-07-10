use crate::dns_cache::DnsCache;
use crate::domain_name::DomainName;
use crate::message::type_rtype::Rtype;

pub struct AsyncResolver{
    // config: ResolverConfig,  FIXME: ver si conviene para configurara tiposd e consultas que aceptara resolver
    cache: DnsCache,
    host: DomainName,
    use_cache: bool,
    recursive_available: bool,
}

impl AsyncResolver{

    pub fn new()-> Self{
        unimplemented!();
    } 

    //esta es la que los usuarios llamaran/ocuparan entonces tiene q ser simple , por eso llama internamente otra
    pub async fn inner_lookup(&self, name: DomainName,rtype: Rtype) -> Result<&'static str, &'static str>{
        //FIXME: deberia retornar algo Result<Lookup, &'static str>
        
        //obtiene todos los nombre a los que va a ir a consultar, Se construye a partir del nombre de domino completo a consultar
        let _list_names = self.build_names(name);
        
        //TODO: hara un look up para la lista entera con un map
        // LookupFuture::lookup().map();
        //logica del resolver

        unimplemented!();
    }

    ///Crea una lista con los nombres a consultar, se crea a partir del nombre de domiinio
    pub fn build_names(&self,_full_name: DomainName){
        unimplemented!();
    }

}