use bitcoin_hashes::siphash24;
use ckb_hash::blake2b_256;
use rand::prelude::*;

pub const M: u64 = 784_931;

// fast reduction of hash to [0, nm) range
fn map_to_range(hash: u64, nm: u64) -> u64 {
    ((hash as u128 * nm as u128) >> 64) as u64
}

fn main() {
    let mut rng = rand::thread_rng();
    let mut data = [0u8; 512];
    let mut u64_bytes = [0u8; 8];
    let k0: u64 = 0x_07_06_05_04_03_02_01_00;
    let k1: u64 = 0x_0f_0e_0d_0c_0b_0a_09_08;
    for round in 0..6 {
        rng.fill(&mut data);
        for n in [1u64, 11, 111, 1111, 11111, 111111, 1111111, 11111111] {
            let nm = n * M;
            let sip_u64 = siphash24::Hash::hash_to_u64_with_keys(k0, k1, &data);
            let sip_range = map_to_range(sip_u64, nm);
            u64_bytes.copy_from_slice(&blake2b_256(&data)[0..8]);
            let blake2b_u64 = u64::from_le_bytes(u64_bytes);
            let blake2b_range = map_to_range(blake2b_u64, nm);
            println!("round={}, n={:>8}, sip=({:>20} => {:<15})", round, n, sip_u64, sip_range);
            println!("                 blake2b=({:>20} => {:<15})\n", blake2b_u64, blake2b_range);
        }
    }
}
