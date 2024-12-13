use std::env::args;
use std::net::IpAddr;
use std::{io, process};
use std::io::Write;
use std::str::FromStr;
use std::sync::mpsc::{channel, Sender};

const MAX: u16 = 65535;

pub fn proceed() {
    println!("Port sniffer CLI");

    let args:Vec<String> = args().collect();

    for arg in args.iter() {
        println!("Argument {:?}", arg);
    }
    println!("---------------------------------");
    let mut arguments = Arguments::new(args.clone()).unwrap_or_else(
        |err| {
            if err.contains("help") {
                process::exit(0);
            }else {
                eprintln!("Problem parsing arguments: {}", err);
                eprintln!("Ip address format [1]: {:?} \
                \nThread number format [2]: {:?}", args[1], args[2]);
                process::exit(0);
            }
        }
    );

    let mut threads = arguments.threads;
    let (tx,rx) = channel();
    for i in 0..threads {
        let tx = tx.clone();
        std::thread::spawn(move || {
            scan(tx, i, arguments.ip, threads);
        });
    }

    let mut out = vec![];
    drop(tx);

    for p in rx {
        out.push(p);
    }

    println!("---------------------------------");

    out.sort();

    for v in out {
        println!("Port {} is open", v);
    }

}

fn scan(tx: Sender<u16>, p1: u16, ip_addr: IpAddr, p3: u16) {
    println!("Thread {} scanning", p1);
    let mut port:u16 = p1 + 1;
    loop {
        match std::net::TcpStream::connect((ip_addr, port)) {
            Ok(_) => {
                io::stdout().flush().unwrap();
                tx.send(port.clone()).expect("Could not send data");
            }
            Err(_) => {}
        }
        if (MAX - port) <= p3 {
            break;
        }
        port += p3;
    }
}

#[derive(Debug)]
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
        if let Ok(ipx) = IpAddr::from_str(&ip) {
            let threads = args[2].parse::<u16>().unwrap_or_else(|_| 16_u16);
            Ok(Arguments { flag: String::from("scan"), ip: ipx, threads })
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