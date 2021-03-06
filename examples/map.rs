use pcp::{Alert, Client, InboundMap, ProtocolNumber, Request};
use std::net::Ipv4Addr;

fn main() {
    let pcp = Client::<Ipv4Addr>::start(
        [192, 168, 1, 101].into(), // My address
        [192, 168, 1, 1].into(),   // PCP server address
    )
    .unwrap();

    // Define a mapping that maps any incoming request on TCP port 6000 to my address
    let map = InboundMap::new(6000, 120).protocol(ProtocolNumber::Tcp);

    // Request the mapping
    let handle = pcp.request(map).unwrap();

	while let Ok(alert) = handle.wait() {
		match alert {
			Alert::StateChange => println!("State: {:?}", handle.state()),
			Alert::Assigned(ip, port) => println!("Assigned ip: {:?}\nAssigned port: {}", ip, port),
		}
	}
}
