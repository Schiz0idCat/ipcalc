use ipcalc::Cli;

use clap::Parser;
use ipnet::IpNet;

use std::process::ExitCode;

fn main() -> ExitCode {
    let cli = Cli::parse();

    let addr: IpNet = match cli.to_ipnet() {
        Ok(net) => net,
        Err(e) => {
            eprintln!("{}", e);

            return ExitCode::FAILURE;
        }
    };

    println!("Address: {:>10}", addr.addr());
    println!("NetMask: {:>10}", addr.netmask());
    println!("Hosts: {:>5}", addr.hosts().size_hint().0);

    ExitCode::SUCCESS
}
