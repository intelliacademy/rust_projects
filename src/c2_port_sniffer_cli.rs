use std::env::args;
use std::net::IpAddr;
use std::str::FromStr;

pub fn proceed() {
    println!("Port sniffer CLI");

    let args:Vec<String> = args().collect();

    for arg in args.iter() {
        println!("Argument {:?}", arg);
    }
    let mut arguments = Arguments::new(args.clone()).unwrap_or_else(
        |err| {
            println!("Problem parsing arguments: {}", err);
            std::process::exit(1);
        }
    );

}

struct Arguments {
    flag: String,
    ip: IpAddr,
    threads: u16,
}

impl Arguments {
    pub fn new( args:Vec<String>) -> Result<Arguments,&'static str> {
        if args.len() == 1 {
            return Err("No arguments found");
        }
        if args.len() > 4 {
            return Err("Too many arguments");
        }

        let ip = args[1].clone();
        return if let Ok(ipx) = IpAddr::from_str(&ip) {
            Err("Invalid IP address")
        } else {
            let flag = args[1].clone();
            if flag != "-h" && flag != "-help" && flag != "-t" {
                Err("Invalid flag")
            } else {
                if flag == "-h" || flag == "-help" {
                    Ok(Arguments { flag: String::from("help"), ip: IpAddr::from_str(&ip).unwrap(), threads: 0 })
                } else if flag.contains("-h") || flag.contains("-help") {
                    Err("Invalid flag")
                } else if flag.contains("-j") {
                    let ipaddr = match IpAddr::from_str(&args[3]) {
                        Ok(ip) => ip,
                        Err(_) => return Err("Invalid IP address"),
                    };
                    let threads = match args[2].parse::<u16>() {
                        Ok(t) => t,
                        Err(_) => return Err("Invalid thread number"),
                    };
                    Ok(Arguments { flag: String::from("scan"), ip: ipaddr, threads: threads })
                } else {
                    Err("Invalid flag")
                }
            }
        }
    }
}