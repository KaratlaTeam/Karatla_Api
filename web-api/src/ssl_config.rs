use std::fs::File;
use std::io::BufReader;

use rustls::internal::pemfile::{certs, rsa_private_keys};
use rustls::{NoClientAuth, ServerConfig};

pub fn ssl_load() -> ServerConfig {
    
    let mut config = ServerConfig::new(NoClientAuth::new());
    
    // open file
    let cert_file = &mut BufReader::new(File::open("static/karatla_ssl_pem.pem").unwrap());
    let key_file = &mut BufReader::new(File::open("static/karatla_ssl_key.key").unwrap());
    
    // get key amd cert
    let cert_chain = certs(cert_file).unwrap();
    let mut keys = rsa_private_keys(key_file).unwrap();

    config.set_single_cert(cert_chain, keys.remove(0)).unwrap();

    config
}