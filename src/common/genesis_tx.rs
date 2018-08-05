use common::address::Address;
use serialization::tx;
use protobuf::Message;

pub struct GenesisTx {
    to: Address,
    amount: u64
}

impl GenesisTx {
    fn new(to: Address, amount: u64) -> GenesisTx {
        GenesisTx {
            to,
            amount
        }
    }

    fn from(itx: tx::Tx) -> GenesisTx {
        let mut to = [0; 20];
        to.clone_from_slice(&itx.to[..]);
        GenesisTx {
            to,
            amount: itx.amount
        }
    }

    fn equals(&self, other_genesis_tx: GenesisTx) -> bool {
        if self.to != other_genesis_tx.to {
            false
        } else if self.amount != other_genesis_tx.amount {
            false
        } else {
            true
        }
    }

    fn encode(&self) -> Vec<u8> {
        let mut itx = tx::Tx::new();
        itx.set_to(self.to.to_vec());
        itx.set_amount(self.amount);
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
    fn it_makes_a_genesis_transaction() {
        let to = [87, 217, 90, 40, 10, 
                  141, 125, 74, 177, 128,
                  155, 18, 148, 149, 135,
                  84, 9, 224, 232, 102];
        let amount = 123456789;
        let genesis_tx = GenesisTx::new(to, amount);
        assert_eq!(genesis_tx.to, to);
        assert_eq!(genesis_tx.amount, amount);
    }

    #[test]
    fn it_makes_a_genesis_transaction_from_itx() {
        let to = [87, 217, 90, 40, 10, 
                  141, 125, 74, 177, 128,
                  155, 18, 148, 149, 135,
                  84, 9, 224, 232, 102];
        let amount = 123456789;
        let mut itx = tx::Tx::new();
        itx.set_to(to.to_vec());
        itx.set_amount(amount);

        let genesis_tx = GenesisTx::from(itx);
        assert_eq!(genesis_tx.to, to);
        assert_eq!(genesis_tx.amount, amount);
    }

    #[test]
    fn it_encodes_like_javascript() {
        let to = [87, 217, 90, 40, 10, 
                  141, 125, 74, 177, 128,
                  155, 18, 148, 149, 135,
                  84, 9, 224, 232, 102];
        let amount = 123456789;
        let genesis_tx = GenesisTx::new(to, amount);
        let encoding = genesis_tx.encode();
        let expected_encoding = vec![18,20,87,217,90,
                                     40,10,141,125,74,
                                     177,128,155,18,148,
                                     149,135,84,9,224,
                                     232,102,24,149,154,
                                     239,58];
        assert_eq!(encoding, expected_encoding);
    }

}