use std::{io, net::SocketAddr, net::ToSocketAddrs};

use anyhow::{anyhow, Result};

pub struct StdResolver;

impl ureq::Resolver for StdResolver {
    fn resolve(&self, netloc: &str) -> io::Result<Vec<SocketAddr>> {
        return match std_resolve(netloc) {
            Ok(res) => Ok(vec![res]),
            Err(e) => Err(std::io::Error::last_os_error()),
        };
    }
}

pub fn std_resolve(domain: &str) -> Result<SocketAddr> {
    match (domain, 80_u16).to_socket_addrs() {
        Ok(mut s1) => match s1.next() {
            Some(s2) => return Ok(s2),
            None => {
                return Err(anyhow!(
                    "DNS resolution {} returned no IP addresses",
                    domain
                ))
            }
        },
        Err(e) => {
            return Err(anyhow!(
                "Unable to resolve the {} IP address with error {}",
                domain,
                e
            ))
        }
    };
}
