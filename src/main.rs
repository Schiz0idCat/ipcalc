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

    println!("Address:   {}", addr.addr());
    println!("NetMask:   {}", addr.netmask());
    println!("Hosts:     {}", addr.hosts().size_hint().0);
    println!("Network:   {}", addr.network());
    println!("Broadcast: {}", addr.broadcast());

    ExitCode::SUCCESS
}
