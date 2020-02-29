use std::str::FromStr;
use structopt::StructOpt;
use tokio::net::UdpSocket;
use tokio::runtime::Runtime;
use trust_dns_client::client::{AsyncClient, ClientHandle};
use trust_dns_client::rr::{DNSClass, Name, Record, RecordType};
use trust_dns_client::udp::UdpClientStream;
use trust_dns_resolver::config::{
    LookupIpStrategy, NameServerConfig, Protocol, ResolverConfig, ResolverOpts,
};
use trust_dns_resolver::TokioAsyncResolver;

macro_rules! printb {
    ($($ex:expr),+) => {
        println!("{}", ansi_term::Style::new().bold().paint(format!($($ex),+)));
    };
}

#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
struct Opt {
    /// Domain for resolver
    #[structopt(long)]
    domain: Option<String>,

    /// Search domains for resolver
    #[structopt(long)]
    search: Vec<String>,

    /// Resolve IPv6 as well as IPv4
    #[structopt(long)]
    ipv6: bool,

    /// DNS Server to query
    #[structopt(name = "SERVER")]
    server: String,

    /// DNS Name to query
    #[structopt(name = "NAME")]
    name: String,

    /// DNS Record to query
    #[structopt(default_value = "A")]
    record: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    fern::Dispatch::new()
        // Perform allocation-free log formatting
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}\t[{}] {}",
                record.level(),
                record.target(),
                message
            ))
        })
        .level(log::LevelFilter::Info)
        .level_for("trust_dns_client", log::LevelFilter::Debug)
        .level_for("trust_dns_proto", log::LevelFilter::Debug)
        .level_for("trust_dns_resolver", log::LevelFilter::Debug)
        .chain(std::io::stdout())
        .apply()?;

    let mut opt = Opt::from_args();

    printb!("<> Starting tokio runtime");
    let mut runtime = Runtime::new()?;

    printb!("<> Parsing server address");
    let address = opt.server.parse()?;

    printb!("<> Connecting to server");
    let stream = UdpClientStream::<UdpSocket>::new(address);
    let client = AsyncClient::connect(stream);
    let (mut client, bg) = runtime.block_on(client)?;

    // Spawn background worker
    runtime.spawn(bg);

    printb!("<> Parsing name to query");
    let name = Name::from_str(&opt.name)?;
    let rtype = RecordType::from_str(&opt.record)?;

    printb!("<> Querying with raw client");
    let query = client.query(name.clone(), DNSClass::IN, rtype);
    let response = runtime.block_on(query)?;

    let answers: &[Record] = response.answers();

    printb!("<> Got {} records", answers.len());
    for answer in answers {
        println!("{:?}", answer.rdata());
    }

    match rtype {
        RecordType::A => {}
        RecordType::AAAA => {
            if !opt.ipv6 {
                printb!("<> Force-enabling IPv6 because AAAA requested");
                opt.ipv6 = true;
            }
        }
        _ => return Ok(()),
    };

    printb!("<> Configuring resolver");
    let mut config = ResolverConfig::new();

    if let Some(dom) = opt.domain {
        printb!(" > Domain: {}", dom);
        let dname = Name::from_str(&dom)?;
        config.set_domain(dname);
    }

    for search in opt.search {
        printb!(" > Search: {}", search);
        let sname = Name::from_str(&search)?;
        config.add_search(sname);
    }

    printb!(" > Nameserver: {}", address);
    config.add_name_server(NameServerConfig {
        socket_addr: address,
        protocol: Protocol::Udp,
        tls_dns_name: None,
    });

    let mut opts = ResolverOpts::default();

    printb!(" > Cache size: {}", opts.cache_size);
    printb!(" > EDNS (larger records): {}", opts.edns0);

    printb!(" > DNSSEC Validation: true");
    opts.validate = true;

    printb!(" > Use hosts file: false");
    opts.use_hosts_file = false;

    printb!(" > Concurrent requests: false");
    opts.num_concurrent_reqs = 1;

    if opt.ipv6 {
        printb!(" > IPv6: true");
        opts.ip_strategy = LookupIpStrategy::Ipv4AndIpv6;
    } else {
        printb!(" > IPv6: false");
        opts.ip_strategy = LookupIpStrategy::Ipv4Only;
    }

    printb!("<> Starting resolver");
    let resolver = TokioAsyncResolver::tokio(config, opts);
    let resolver = runtime.block_on(resolver)?;

    printb!("<> Resolving IP record");
    let response = runtime.block_on(resolver.lookup_ip(name))?;

    let mut length = 0;
    let mut responses = Vec::new();
    for address in response {
        length += 1;
        responses.push(format!("{:?}", address));
    }

    printb!("<> Got {} addresses", length);
    printb!("{}", responses.join("\n"));

    Ok(())
}
