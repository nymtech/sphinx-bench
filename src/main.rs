use sphinx::ProcessedPacket;

extern crate sphinx;

use sphinx::crypto;
use sphinx::header::delays;
use sphinx::route::{Destination, Node};
use sphinx::SphinxPacket;

const NODE_ADDRESS_LENGTH: usize = 32;
const DESTINATION_ADDRESS_LENGTH: usize = 32;
const IDENTIFIER_LENGTH: usize = 16;
const SECURITY_PARAMETER: usize = 16;
const PAYLOAD_SIZE: usize = 1024;

fn main() {
    println!("Hello, sphinx!");

    for _ in 0..100000 {
        let (node1_sk, node1_pk) = crypto::keygen();
        let node1 = Node::new([5u8; NODE_ADDRESS_LENGTH], node1_pk);
        let (node2_sk, node2_pk) = crypto::keygen();
        let node2 = Node::new([4u8; NODE_ADDRESS_LENGTH], node2_pk);
        let (node3_sk, node3_pk) = crypto::keygen();
        let node3 = Node::new([2u8; NODE_ADDRESS_LENGTH], node3_pk);

        let route = [node1, node2, node3];
        let delays = delays::generate(route.len());
        let destination =
            Destination::new([3u8; DESTINATION_ADDRESS_LENGTH], [4u8; IDENTIFIER_LENGTH]);

        let message = vec![13u8, 16];
        let sphinx_packet =
            match SphinxPacket::new(message.clone(), &route, &destination, &delays).unwrap() {
                SphinxPacket { header, payload } => SphinxPacket { header, payload },
            };

        let next_sphinx_packet_1 = match sphinx_packet.process(node1_sk) {
            ProcessedPacket::ProcessedPacketForwardHop(next_packet, next_hop_addr1, _delay1) => {
                assert_eq!([4u8; NODE_ADDRESS_LENGTH], next_hop_addr1);
                next_packet
            }
            _ => panic!(),
        };
        let next_sphinx_packet_2 = match next_sphinx_packet_1.process(node2_sk) {
            ProcessedPacket::ProcessedPacketForwardHop(next_packet, next_hop_addr2, _delay2) => {
                assert_eq!([2u8; NODE_ADDRESS_LENGTH], next_hop_addr2);
                next_packet
            }
            _ => panic!(),
        };
        match next_sphinx_packet_2.process(node3_sk) {
            ProcessedPacket::ProcessedPacketFinalHop(_, _, payload) => {
                let zero_bytes = vec![0u8; SECURITY_PARAMETER];
                let additional_padding = vec![
                    0u8;
                    PAYLOAD_SIZE
                        - SECURITY_PARAMETER
                        - message.len()
                        - destination.address.len()
                        - 1
                ];
                let expected_payload = [
                    zero_bytes,
                    destination.address.to_vec(),
                    message,
                    vec![1],
                    additional_padding,
                ]
                .concat();
                assert_eq!(expected_payload, payload.get_content());
            }
            _ => panic!(),
        };
    }
}
