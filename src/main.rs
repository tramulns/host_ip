use clap::{value_t, App, Arg};
use dns_lookup::lookup_host;

fn main() {
    let app = App::new("host_ip").arg(
        Arg::with_name("HOST")
            .index(1)
            .multiple(false)
            .required(true),
    );

    let matches = match app.get_matches_from_safe(std::env::args_os().into_iter()) {
        Ok(m) => m,
        Err(ref e)
            if e.kind == clap::ErrorKind::HelpDisplayed
                || e.kind == clap::ErrorKind::VersionDisplayed =>
        {
            println!("{}", e);
            std::process::exit(0);
        }
        Err(f) => {
            eprintln!("{}", f);
            std::process::exit(1)
        }
    };

    let hostname = value_t!(matches, "HOST", String).unwrap_or_else(|e| e.exit());
    let ips: Vec<std::net::IpAddr> = lookup_host(&hostname).unwrap();
    for ip in ips {
        println!("{}", ip);
    }
}
