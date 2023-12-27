use std::{io, net::SocketAddr};

use anyhow::{anyhow, Result};

pub struct TrustResolver;

impl ureq::Resolver for TrustResolver {
    fn resolve(&self, netloc: &str) -> io::Result<Vec<SocketAddr>> {
        return match trust_from_system_conf_resolve(netloc) {
            Ok(res) => Ok(vec![res]),
            Err(e) => Err(std::io::Error::last_os_error()),
        };
    }
}

pub fn trust_from_system_conf_resolve(domain: &str) -> Result<SocketAddr> {
    // On Unix/Posix systems, this will read the /etc/resolv.conf
    let resolver = match trust_dns_resolver::Resolver::from_system_conf() {
        Ok(res) => res,
        Err(e) => {
            return Err(anyhow!(
                "DNS resolution {} failed with from_system_conf error {:?}",
                domain,
                e
            ))
        }
    };

    // Lookup the IP addresses associated with a name.
    let response = match resolver.lookup_ip(domain) {
        Ok(res) => res,
        Err(e) => {
            return Err(anyhow!(
                "DNS resolution {} failed with lookup_ip error {:?}",
                domain,
                e
            ))
        }
    };
    match response.iter().next() {
        Some(ipinfo) => return Ok(SocketAddr::new(ipinfo, 80)),
        None => {
            return Err(anyhow!(
                "DNS resolution {} returned no IP addresses",
                domain
            ))
        }
    }
}
