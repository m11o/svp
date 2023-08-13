pub struct SledClient {
    client: sled::Db,
}

use sled::IVec;

const DATABASE_PATH: &str = "tmp/word.db";

impl SledClient {
    pub fn new() -> Self {
        let client = sled::open(DATABASE_PATH).unwrap();
        Self {
            client,
        }
    }

    pub fn create(&self, word: String, item_id: String) {
        let item_id_value = IVec::from(item_id.as_bytes());
        self.client.insert(word, item_id_value).unwrap();
    }

    pub fn is_saved_key(&self, word: String) -> bool {
        match self.client.get(word) {
            Ok(Some(_)) => true,
            _ => false,
        }
    }
}