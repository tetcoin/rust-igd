extern crate ig;

fn main() {
    match ig::search_gateway() {
        Err(ref err) => println!("Error: {}", err),
        Ok(gateway) => {
            match gateway.remove_port(ig::PortMappingProtocol::TCP, 80) {
                Err(ref err) => {
                    println!("There was an error! {}", err);
                },
                Ok(()) => {
                    println!("It worked");
                },
            }
        },
    }
}
