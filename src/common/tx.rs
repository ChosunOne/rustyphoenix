use common::address::Address;
use serialization::tx;
use protobuf::Message;

pub struct Tx {
    from: Address,
    to: Address,
    amount: u64,
    fee: u64,
    nonce: u32
}

impl Tx {
    fn new(from: Address, to: Address, amount: u64, fee: u64, nonce: u32) -> Tx {
        Tx {
            from,
            to,
            amount,
            fee, 
            nonce
        }
    }

    fn from(itx: tx::Tx) -> Tx {
        let mut from = [0; 20];
        from.clone_from_slice(&itx.from[..]);
        let mut to = [0; 20];
        to.clone_from_slice(&itx.to[..]);
        Tx {
            from,
            to,
            amount: itx.amount,
            fee: itx.fee,
            nonce: itx.nonce
        }
    }

    fn equals(&self, other_tx: Tx) -> bool {
        if self.from != other_tx.from {
            false
        } else if self.to != other_tx.to {
            false
        } else if self.amount != other_tx.amount {
            false
        } else if self.fee != other_tx.fee {
            false
        } else if self.nonce != other_tx.nonce {
            false
        } else {
            true
        }
    }

    fn encode(&self) -> Vec<u8> {
        let mut itx = tx::Tx::new();
        itx.set_from(self.from.to_vec());
        itx.set_to(self.to.to_vec());
        itx.set_amount(self.amount);
        itx.set_fee(self.fee);
        itx.set_nonce(self.nonce);
        let encoding = itx.write_to_bytes();
        match encoding {
            Ok(bytes) => bytes,
            Err(e) => {
                println!("{}", e);
                vec![]
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_makes_a_transaction() {
        let from = [230, 104, 95, 253, 219, 
                    134, 92, 215, 230, 126,
                    105, 213, 18, 95, 30, 166, 
                    128, 229, 233, 114];
        let to = [87, 217, 90, 40, 10, 
                  141, 125, 74, 177, 128,
                  155, 18, 148, 149, 135,
                  84, 9, 224, 232, 102];
        let amount = 123456789;
        let fee = 1;
        let nonce = 3;
        let tx = Tx::new(from, to, amount, fee, nonce);
        assert_eq!(tx.from, from);
        assert_eq!(tx.to, to);
        assert_eq!(tx.amount, amount);
        assert_eq!(tx.fee, fee);
        assert_eq!(tx.nonce, nonce);
    }

    #[test]
    fn it_makes_a_transaction_from_itx() {
        let from = [230, 104, 95, 253, 219, 
                    134, 92, 215, 230, 126,
                    105, 213, 18, 95, 30, 166, 
                    128, 229, 233, 114];
        let to = [87, 217, 90, 40, 10, 
                  141, 125, 74, 177, 128,
                  155, 18, 148, 149, 135,
                  84, 9, 224, 232, 102];
        let amount = 123456789;
        let fee = 1;
        let nonce = 3;
        let mut itx = tx::Tx::new();
        itx.set_from(from.to_vec());
        itx.set_to(to.to_vec());
        itx.set_amount(amount);
        itx.set_fee(fee);
        itx.set_nonce(nonce);

        let tx = Tx::from(itx);
        assert_eq!(tx.from, from);
        assert_eq!(tx.to, to);
        assert_eq!(tx.amount, amount);
        assert_eq!(tx.fee, fee);
        assert_eq!(tx.nonce, nonce);
    }

    #[test]
    fn it_encodes_like_javascript() {
        let from = [230, 104, 95, 253, 219, 
                    134, 92, 215, 230, 126,
                    105, 213, 18, 95, 30, 166, 
                    128, 229, 233, 114];
        let to = [87, 217, 90, 40, 10, 
                  141, 125, 74, 177, 128,
                  155, 18, 148, 149, 135,
                  84, 9, 224, 232, 102];
        let amount = 123456789;
        let fee = 1;
        let nonce = 3;
        let tx = Tx::new(from, to, amount, fee, nonce);
        let encoding = tx.encode();
        let expected_encoding = vec![10,20,230,104,95,
                                     253,219,134,92,215,
                                     230,126,105,213,18,
                                     95,30,166,128,229,
                                     233,114,18,20,87,
                                     217,90,40,10,141,
                                     125,74,177,128,155,
                                     18,148,149,135,84,
                                     9,224,232,102,24,
                                     149,154,239,58,32,
                                     1,40,3];
        assert_eq!(encoding, expected_encoding);
    }

}