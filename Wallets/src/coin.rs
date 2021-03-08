use neo_core::fixed8::fixed8;

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash, Serialize, Deserialize)]
pub struct Coin<'a> {
    index: u32,
    txid: &'a str,
    value: fixed8,
}


impl<'a> Coin<'a> {

    pub fn index(&self) -> u32 {
        self.index
    }
    pub fn txid(&self) -> &'a str {
        self.txid
    }
    pub fn value(&self) -> &fixed8 {
        &self.value
    }

    pub fn export(&self) -> Coin {
        Coin {
            index: self.index,
            txid: self.txid.clone(),
            value: self.value.clone(),
        }
    }

    pub fn equals(&self, other: &Coin) -> bool {
        self.index == &other.index &&
            self.txid == &other.txid &&
            self.value.equals(&other.value)
    }
}