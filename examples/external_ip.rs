extern crate ig;

fn main() {
    match ig::search_gateway() {
        Err(ref err) => println!("Error: {}", err),
        Ok(gateway) => {
            match gateway.get_external_ip() {
                Err(ref err) => {
                    println!("There was an error! {}", err);
                },
                Ok(ext_addr) => {
                    println!("Local gateway: {}, External ip address: {}", gateway, ext_addr);
                },
            }
        },
    }
}
