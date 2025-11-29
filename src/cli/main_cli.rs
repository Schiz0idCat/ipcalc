use crate::error::ParseError;

use clap::Parser;
use std::net::IpAddr;
use std::str::FromStr;
use ipnet::IpNet;

#[derive(Parser)]
#[command(arg_required_else_help = true)]
#[command(about = "Simple ip calculator")]
pub struct Cli {
    #[arg(long, short, help = "ip address (may be without CIDR)")]
    ip: String,

    #[arg(long, short, help = "netmask (optional)")]
    mask: Option<String>,
}

impl Cli {
    pub fn to_ipnet(&self) -> Result<IpNet, ParseError> {
        match &self.mask {
            None => Ok(IpNet::from_str(&self.ip)?),
            Some(_) => self.parse_ip_with_mask(),
        }
    }

    fn parse_ip_with_mask(&self) -> Result<IpNet, ParseError> {
        let mask_str = match &self.mask {
            Some(m) => m.as_str(),
            None => return Err(ParseError::MissingMask),
        };

        let ip_addr: IpAddr = self.ip.parse()?;

        match mask_str.parse::<IpAddr>() {
            // long mask format (e.g. 255.255.255.0)
            Ok(mask_ip) => Ok(IpNet::with_netmask(ip_addr, mask_ip)?),

            // CIDR mask format (/24 or 24)
            Err(_) => {
                let prefix = mask_str.trim_start_matches('/').parse()?;

                Ok(IpNet::new(ip_addr, prefix)?)
            }
        }
    }
}
