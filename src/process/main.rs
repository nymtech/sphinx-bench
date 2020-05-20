extern crate sphinx;

use sphinx::crypto;
use sphinx::header::delays;
use sphinx::route::NodeAddressBytes;
use sphinx::route::{Destination, DestinationAddressBytes, Node};
use sphinx::SphinxPacket;
use std::time::Duration;

const NODE_ADDRESS_LENGTH: usize = 32;
const DESTINATION_ADDRESS_LENGTH: usize = 32;
const IDENTIFIER_LENGTH: usize = 16;

fn main() {
    println!("Creating 1 Sphinx packet and unwrapping it a lot of times");

    let (node1_sk, node1_pk) = crypto::keygen();
    let node1 = Node::new(
        NodeAddressBytes::from_bytes([5u8; NODE_ADDRESS_LENGTH]),
        node1_pk,
    );
    let (_, node2_pk) = crypto::keygen();
    let node2 = Node::new(
        NodeAddressBytes::from_bytes([4u8; NODE_ADDRESS_LENGTH]),
        node2_pk,
    );
    let (_, node3_pk) = crypto::keygen();
    let node3 = Node::new(
        NodeAddressBytes::from_bytes([2u8; NODE_ADDRESS_LENGTH]),
        node3_pk,
    );

    let route = [node1, node2, node3];
    let delays = delays::generate_from_average_duration(route.len(), Duration::from_millis(10));
    let destination = Destination::new(
        DestinationAddressBytes::from_bytes([3u8; DESTINATION_ADDRESS_LENGTH]),
        [4u8; IDENTIFIER_LENGTH],
    );
    let message = vec![13u8, 16];
    let packet = SphinxPacket::new(message.clone(), &route, &destination, &delays, None).unwrap();

    for _ in 0..1000 {
        packet.clone().process(node1_sk).unwrap();
    }
}
