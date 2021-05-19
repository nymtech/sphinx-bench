extern crate sphinx;

use sphinx::{crypto, SphinxPacket};
use sphinx::route::{Node, NodeAddressBytes, Destination, DestinationAddressBytes};
use sphinx::constants::{NODE_ADDRESS_LENGTH, DESTINATION_ADDRESS_LENGTH, IDENTIFIER_LENGTH};
use std::time::Duration;
use sphinx::header::{delays, keys};
use sphinx::test_utils::fixtures::hkdf_salt_fixture;

const node1_sk: [u8; 32] = [152, 16, 3, 198, 237, 81, 164, 139, 120, 223, 232, 1, 139, 75, 143, 146, 40, 188, 90, 144, 77, 189, 196, 140, 94, 110, 253, 254, 24, 38, 45, 77];
const node2_sk: [u8; 32] = [160, 100, 57, 26, 222, 114, 219, 2, 16, 134, 59, 254, 63, 198, 158, 232, 71, 206, 250, 204, 20, 243, 242, 73, 125, 189, 158, 212, 37, 161, 50, 101];
const node3_sk: [u8;32] = [248, 185, 223, 21, 39, 10, 86, 226, 105, 179, 216, 43, 140, 188, 0, 0, 241, 29, 59, 194, 9, 80, 36, 176, 180, 40, 219, 200, 140, 145, 252, 86];

const node1_pk_bytes: [u8; 32] = [96, 85, 39, 207, 33, 61, 106, 35, 99, 98, 193, 184, 10, 103, 161, 180, 199, 3, 114, 92, 90, 245, 91, 135, 7, 195, 109, 48, 156, 59, 141, 83];
const node2_pk_bytes: [u8; 32] = [34, 109, 116, 102, 45, 145, 189, 34, 236, 138, 142, 57, 141, 230, 94, 233, 0, 230, 230, 121, 13, 195, 66, 209, 227, 217, 244, 170, 15, 15, 166, 38];
const node3_pk_bytes: [u8; 32] = [164, 103, 20, 95, 51, 139, 88, 47, 250, 8, 226, 247, 244, 31, 146, 209, 146, 110, 78, 87, 209, 104, 80, 245, 19, 63, 185, 198, 28, 175, 198, 87];


const salt1: [u8; 32] =[157, 119, 175, 80, 29, 2, 228, 213, 134, 226, 222, 108, 204, 40, 53, 44, 83, 145, 117, 45, 139, 234, 30, 39, 224, 196, 145, 165, 82, 183, 131, 238];
const salt2: [u8; 32] = [130, 43, 117, 106, 227, 230, 203, 89, 191, 62, 96, 181, 228, 181, 51, 173, 91, 181, 155, 72, 82, 17, 206, 223, 169, 68, 250, 110, 240, 43, 162, 61];
const salt3: [u8; 32] =  [191, 246, 173, 250, 231, 232, 191, 76, 77, 15, 5, 203, 13, 115, 136, 182, 18, 31, 34, 232, 29, 109, 77, 50, 214, 168, 61, 44, 74, 251, 127, 144];

const initital_secret_bytes: [u8; 32] =  [208, 246, 193, 236, 95, 209, 243, 65, 40, 138, 57, 103, 135, 200, 177, 15, 182, 131, 110, 131, 93, 39, 109, 125, 115, 121, 135, 39, 238, 135, 134, 120];
const initital_shared_secret_bytes: [u8; 32] = [37, 196, 197, 122, 29, 47, 44, 45, 216, 119, 133, 224, 42, 14, 175, 211, 109, 141, 172, 123, 182, 0, 252, 29, 136, 120, 140, 232, 87, 201, 230, 70];

const shared_key1_bytes: [u8; 32] =  [208, 232, 201, 166, 191, 135, 41, 153, 107, 45, 179, 119, 5, 219, 55, 72, 149, 2, 206, 140, 29, 89, 177, 4, 159, 234, 171, 99, 34, 229, 70, 105];
const shared_key2_bytes: [u8; 32] =  [110, 116, 113, 237, 90, 11, 235, 200, 32, 16, 138, 245, 11, 151, 17, 126, 79, 167, 55, 63, 85, 171, 131, 45, 252, 255, 25, 7, 135, 153, 96, 113];
const shared_key3_bytes: [u8; 32] = [37, 250, 14, 151, 66, 29, 169, 137, 81, 14, 46, 115, 73, 176, 21, 251, 116, 59, 225, 39, 3, 22, 217, 127, 45, 104, 53, 135, 212, 189, 10, 96];


fn main() {
    println!("Doing a Sphinx packet creation with shared key reuse");

    for _ in 0..1000 {
        let node1_pk: crypto::PublicKey = crypto::PublicKey::from(node1_pk_bytes);
        let node2_pk: crypto::PublicKey = crypto::PublicKey::from(node2_pk_bytes);
        let node3_pk: crypto::PublicKey = crypto::PublicKey::from(node3_pk_bytes);

        let node1 = Node::new(
            NodeAddressBytes::from_bytes([5u8; NODE_ADDRESS_LENGTH]),
            node1_pk,
        );
        let node2 = Node::new(
            NodeAddressBytes::from_bytes([4u8; NODE_ADDRESS_LENGTH]),
            node2_pk,
        );
        let node3 = Node::new(
            NodeAddressBytes::from_bytes([2u8; NODE_ADDRESS_LENGTH]),
            node3_pk,
        );

        let route = [node1, node2, node3];
        let average_delay = Duration::from_secs_f64(1.0);
        let delays = delays::generate_from_average_duration(route.len(), average_delay);
        let hkdf_salt = [salt1, salt2, salt3];
        let destination = Destination::new(
            DestinationAddressBytes::from_bytes([3u8; DESTINATION_ADDRESS_LENGTH]),
            [4u8; IDENTIFIER_LENGTH],
        );

        let initial_shared_secret: crypto::SharedKey = crypto::PublicKey::from(initital_shared_secret_bytes);
        let shared_key1: crypto::SharedKey = crypto::PublicKey::from(shared_key1_bytes);
        let shared_key2: crypto::SharedKey = crypto::PublicKey::from(shared_key2_bytes);
        let shared_key3: crypto::SharedKey = crypto::PublicKey::from(shared_key3_bytes);
        let key_material = keys::KeyMaterial{initial_shared_group_element: initial_shared_secret, shared_keys: vec![shared_key1, shared_key2, shared_key3]};


        let message = vec![13u8, 16];
        let sphinx_packet = match SphinxPacket::new_with_precomputed_keys(
            message.clone(),
                    &route,
                    &destination,
                    &delays,
                    &hkdf_salt,
                    &key_material.shared_keys,
                    &initial_shared_secret,
        )
            .unwrap()
        {
            SphinxPacket { header, payload } => SphinxPacket { header, payload },
        };
    }
}