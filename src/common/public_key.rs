use secp256k1;

struct PublicKey {
    pub_key: Vec<u8>
}

impl PublicKey {
    fn from_buffer(buffer: Vec<u8>) -> PublicKey {
        PublicKey {
            pub_key: buffer
        }
    }    
}