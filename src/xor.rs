pub trait Xor {
    fn xor(&self, key: &[u8]) -> Vec<u8>;
}

impl Xor for Vec<u8> {
    fn xor(&self, key_bytes: &[u8]) -> Vec<u8> {
        xor(self, key_bytes)
    }
}

pub fn xor(input: &[u8], key: &[u8]) -> Vec<u8> {
    let mut key_idx = 0;
    let mut encoded_bytes: Vec<u8> = Vec::new();
    for b in input {
        let k = key[key_idx];
        let e = b ^ k;
        encoded_bytes.push(e);
        key_idx += 1;
        if key_idx >= key.len() {
            key_idx %= key.len();
        }
    }
    encoded_bytes
}
