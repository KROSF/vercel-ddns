use eyre::{eyre, Result};
use futures::executor;
use log::{error, info};
use public_ip::{dns, BoxToResolver, ToResolver};
use structopt::clap::{arg_enum, AppSettings};
use structopt::StructOpt;

use crate::vercel::{add_dns_record, Record};

fn get_public_ip(ip_type: &IpType) -> Result<String> {
    let resolver = match *ip_type {
        IpType::IPV4 => vec![
            BoxToResolver::new(dns::OPENDNS_RESOLVER_V4),
            BoxToResolver::new(dns::GOOGLE_DNS_TXT_RESOLVER_V4),
        ],
        IpType::IPV6 => vec![
            BoxToResolver::new(dns::OPENDNS_RESOLVER_V6),
            BoxToResolver::new(dns::GOOGLE_DNS_TXT_RESOLVER_V6),
        ],
    };

    match executor::block_on(public_ip::resolve_address(resolver.to_resolver())) {
        Some(ip) => Ok(ip.to_string()),
        None => Err(eyre!("Unable to get public IP.")),
    }
}

arg_enum! {
    #[derive(Debug,PartialEq)]
    pub enum IpType {
        IPV4,
        IPV6,
    }
}

#[derive(Debug, StructOpt)]
#[structopt(
    about,
    setting(AppSettings::ColoredHelp),
    setting(AppSettings::ColorAuto)
)]
pub struct Args {
    #[structopt(short, long, env = "VDDNS_DOMAIN")]
    pub domain: String,

    #[structopt(short, long, env = "VDDNS_SUBDOMAIN")]
    pub subdomain: String,

    #[structopt(short, long, possible_values = &IpType::variants(), case_insensitive = true, default_value = "ipv4", env = "VDDNS_IP_TYPE")]
    pub ip_type: IpType,

    #[structopt(long, default_value = "3600", env = "VDDNS_TTL")]
    pub ttl: i64,

    #[structopt(short, long, env = "VERCEL_TOKEN")]
    pub token: String,
}

pub fn run(args: Args) -> Result<()> {
    let ip = match get_public_ip(&args.ip_type) {
        Ok(ip) => ip,
        Err(e) => {
            error!("Unable to get public ip. {}", e.to_string());
            return Ok(());
        }
    };

    let rec = Record::new(
        args.subdomain,
        ip,
        match args.ip_type {
            IpType::IPV4 => String::from("A"),
            IpType::IPV6 => String::from("AAAA"),
        },
        args.ttl,
    );

    match add_dns_record(&args.domain, &args.token, rec) {
        Ok(_) => {
            info!("Record added / updated sucessfully");
            return Ok(());
        }
        Err(e) => {
            error!("Unable to add / update the record. {}", e.to_string());
            return Ok(());
        }
    }
}
