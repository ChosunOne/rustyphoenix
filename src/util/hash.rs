use blake2_rfc::blake2b::blake2b;

pub fn hash(data: &[u8]) -> Vec<u8> {
        let hash_data = blake2b(32, &[], data);
        let mut hash_vec = vec![0; 32];
        hash_vec.clone_from_slice(&hash_data.as_bytes()[..]);
        hash_vec
}