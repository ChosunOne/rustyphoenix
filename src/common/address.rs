use rust_base58::{ToBase58, FromBase58};
use util::hash::hash;

use std::result::Result;

fn check_sum(arr: &[u8; 20]) -> String {
    let arr_hash = hash(arr);
    println!("{:?}", arr_hash);
    let string = arr_hash.to_base58();
    format!("{}", &string[0..4])
}

pub type Address = [u8; 20];

trait Standard {
    fn to_string(&self) -> String;
    fn from_string(string: &String) -> Result<Address, String>;
}

impl Standard for Address {
    fn to_string(&self) -> String {
        let base58_address = self.to_base58();
        "H".to_string() + &base58_address + &check_sum(&self)
    }

    fn from_string(string: &String) -> Result<Address, String> {
        let mut string_iter = string.chars();
        let first_char = string_iter.next();
        match first_char {
            Some(letter) => {
                if letter.to_string() != "H" {
                    return Err("Address must begin with an H".to_string())
                }
            },
            None => {
                return Err("No data was supplied".to_string())
            }
        }

        if string.len() != 32 {
            return Err("Address string must be 32 characters long".to_string())
        }
        let address_and_checksum = &string[1..32];
        let address = &address_and_checksum[0..27];
        let checksum = &address_and_checksum[27..31];

        let decoded = address.from_base58();
        let mut decoded_bytes: [u8; 20] = [0; 20];
        match decoded {
            Ok(s) => {
                if s.len() != 20 {
                    return Err(format!("{} is {} bytes long", address, decoded_bytes.len()))
                }
                decoded_bytes.clone_from_slice(&s[..]); 
            },
            Err(e) => {return Err(format!("{}", e))}
        }

        let checksum_bytes = check_sum(&decoded_bytes);
        if checksum_bytes.to_string() != checksum {
            return Err(format!("{} did not match {}", checksum, checksum_bytes.to_string()))
        }

        let mut address_bytes: [u8; 20] = [0; 20];
        address_bytes.clone_from_slice(&decoded_bytes[0..20]);
        Ok(address_bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_makes_an_address_string() {
        let address: Address = [  11, 216, 55, 107, 140, 
                                        247, 121, 126, 90, 115, 
                                        233, 197, 108, 128, 64, 
                                        46, 135, 184, 87, 180
                                     ];
        let expected_address_string = "HAa7S1QqVRMw13VdUnrSkn5w6oNK891W";
        assert_eq!(expected_address_string, address.to_string());
    }

    #[test]
    fn it_makes_an_address_from_string() {
        let address_string = "HAa7S1QqVRMw13VdUnrSkn5w6oNK891W".to_string();
        let address: Address = Address::from_string(&address_string).unwrap();
        assert_eq!(address.to_string(), address_string);
    }

    #[test]
    fn it_only_allows_valid_addresses() {
        let address_string = "Not a valid address string".to_string();
        let address = Address::from_string(&address_string);
        match address {
            Ok(addr) => {panic!("{} is not a valid address string!", address_string)},
            Err(e) => {}
        }
    }

    #[test]
    fn it_checks_checksum() {
        let address_string = "HAa7S1QqVRMw13VdUnrSkn5w6oNK36MM".to_string();
        let address = Address::from_string(&address_string);
        match address {
            Ok(addr) => {panic!("{} is not a valid address string!", address_string)},
            Err(e) => {}
        }
    }

    #[test]
    fn it_checks_equality() {
        let address_string = "HAa7S1QqVRMw13VdUnrSkn5w6oNK891W".to_string();
        let address: Address = Address::from_string(&address_string).unwrap();

        let other_address: [u8; 20] = [ 11, 216, 55, 107, 140, 
                              247, 121, 126, 90, 115, 
                              233, 197, 108, 128, 64, 
                              46, 135, 184, 87, 180
                            ];
        assert_eq!(address, other_address);
    }
}