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

    println!("Address: {:>13}", addr.addr());
    println!("NetMask: {:>15}", addr.netmask());
    println!("Hosts: {:>7}", addr.hosts().size_hint().0);
    println!("Network: {:>13}", addr.network());
    println!("Broadcast: {:>5}", addr.broadcast());

    ExitCode::SUCCESS
}
