# vercel-ddns

netlify-ddns is a simple command line tool for creating a DNS record for Vercel's Managed DNS service.

## Installation

Install using [cargo][cargo]:

```
cargo install netlify-ddns
```

## Usage

```sh
vercel-ddns 0.1.0


USAGE:
    vercel-ddns [OPTIONS] --domain <domain> --subdomain <subdomain> --token <token>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -d, --domain <domain>           [env: VDDNS_DOMAIN=]
    -i, --ip-type <ip-type>         [env: VDDNS_IP_TYPE=]  [default: ipv4]  [possible values: IPV4,
                                   IPV6]
    -s, --subdomain <subdomain>     [env: VDDNS_SUBDOMAIN=]
    -t, --token <token>             [env: VERCEL_TOKEN=]
        --ttl <ttl>                 [env: VDDNS_TTL=]  [default: 3600]
```

## Related

Check out [lukehsiao/netlify-ddns-rs](https://github.com/lukehsiao/netlify-ddns-rs) for a similar
client for Netlify.
