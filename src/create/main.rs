extern crate sphinx;

use sphinx::{crypto, SphinxPacket};
use sphinx::route::{Node, NodeAddressBytes, Destination, DestinationAddressBytes};
use sphinx::constants::{NODE_ADDRESS_LENGTH, DESTINATION_ADDRESS_LENGTH, IDENTIFIER_LENGTH};
use std::time::Duration;
use sphinx::header::delays;
use sphinx::test_utils::fixtures::hkdf_salt_fixture;

fn main() {
    println!("Doing a Sphinx packet creation");

    for _ in 0..1000 {
        let (node1_sk, node1_pk) = crypto::keygen();
        let node1 = Node::new(
            NodeAddressBytes::from_bytes([5u8; NODE_ADDRESS_LENGTH]),
            node1_pk,
        );
        let (node2_sk, node2_pk) = crypto::keygen();
        let node2 = Node::new(
            NodeAddressBytes::from_bytes([4u8; NODE_ADDRESS_LENGTH]),
            node2_pk,
        );
        let (node3_sk, node3_pk) = crypto::keygen();
        let node3 = Node::new(
            NodeAddressBytes::from_bytes([2u8; NODE_ADDRESS_LENGTH]),
            node3_pk,
        );

        let route = [node1, node2, node3];
        let average_delay = Duration::from_secs_f64(1.0);
        let delays = delays::generate_from_average_duration(route.len(), average_delay);
        let hkdf_salt = [hkdf_salt_fixture(), hkdf_salt_fixture(), hkdf_salt_fixture()];
        let destination = Destination::new(
            DestinationAddressBytes::from_bytes([3u8; DESTINATION_ADDRESS_LENGTH]),
            [4u8; IDENTIFIER_LENGTH],
        );

        let message = vec![13u8, 16];
        let sphinx_packet = match SphinxPacket::new(
            message.clone(),
            &route,
            &destination,
            &delays,
            &hkdf_salt,
        )
            .unwrap()
        {
            SphinxPacket { header, payload } => SphinxPacket { header, payload },
        };
    }
}