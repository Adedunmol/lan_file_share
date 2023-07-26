use std::{ error::Error, process, net::{IpAddr, TcpListener, TcpStream }, io::{ BufReader, prelude::* }};
use local_ip_address::local_ip;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // run the logic of the program here

    if config.sub_command == "-receive" {
        // Setup the server here
        let _ = setup_server(); // handle error here
    } else if config.sub_command == "-connect" {
        // Setup the client to connect to the server here
        let address = if let Some(arg) = config.address_or_file_path {
            arg
        } else {
            eprintln!("Did not get an address");
            process::exit(1);
        };

        let _ = setup_connection(&address); //handle error here
    } else {
        eprintln!("Subcommand not recognized");
        process::exit(1);
    }
    
    Ok(())
}

pub struct Config {
    sub_command: String,
    address_or_file_path: Option<String>, // we do not get a file_path when establishing a server
}

impl Config {

    pub fn build(
        mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let sub_command = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a subcommand"),
        };
        let address_or_file_path = args.next();
    
        Ok(Config { sub_command, address_or_file_path })
    }

}

fn get_local_ip() -> IpAddr {
    let ip_address = local_ip().unwrap_or_else(|err| {
        eprintln!("An error occurred (Probably you are not connected to a network): {err}");
        process::exit(1);
    });

    ip_address
}

fn setup_server() {
    let address = get_local_ip();

    // This returns an address without an assigned port to signal to the OS to assign one for us
    let address_without_port = format!("{}:0", address);

    let listener = TcpListener::bind(address_without_port).unwrap_or_else(|err| {
        eprintln!("Error occurred while trying to setup server: {err}");
        process::exit(1);
    });

    println!("Server has been set up");

    let port = listener.local_addr().unwrap().port();

    println!("Server is available on {address}:{port}");

    for stream in listener.incoming() {
        let stream = stream.unwrap_or_else(|err| {
            eprintln!("An error occurred: {err}");
            process::exit(1);
        });

        handle_connection(stream);
    }


}

fn setup_connection(address: &str) -> std::io::Result<()> {
    let mut stream = TcpStream::connect(address).unwrap_or_else(|err| {
        eprintln!("An error occurred while trying to connect to the server: {err}");
        process::exit(1);
    });

    stream.write("Hello".as_bytes());

    Ok(())
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);

    let data: Vec<_> = buf_reader
                            .lines()
                            .map(|result| result.unwrap())
                            .take_while(|line| !line.is_empty())
                            .collect();

    println!("{:#?}", data);
}

mod tests {

    use super::*;

    #[test]
    fn test_get_local_ip() {
        let ip_address = get_local_ip();
        let private_ipv4_addresses = vec!["10.", "172.", "192.168"];

        assert_eq!(ip_address.is_ipv4(), true);
        // Ensures the IP address received is a private IPv4 Address
        assert!(private_ipv4_addresses.iter().any(|&val| ip_address.to_string().starts_with(val)));
    }
}