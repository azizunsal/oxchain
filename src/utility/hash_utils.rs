use crate::Hash;
use sha2::{Digest, Sha256};

pub fn find_merkle_root(transactions: Vec<String>) -> String {
    create_merkle_root(transactions)[0].clone()
}

fn create_merkle_root(transactions: Vec<String>) -> Vec<String> {
    if transactions.len() == 1 {
        return vec![transactions[0].clone()];
    }
    let mut tx1;
    let mut tx2;
    let mut new_list: Vec<String> = Vec::new();
    for i in (0..transactions.len()).step_by(2) {
        tx1 = hex::decode(transactions.get(i).unwrap()).unwrap();
        if i == transactions.len() - 1 {
            tx2 = hex::decode(transactions.get(i).unwrap()).unwrap();
        } else {
            tx2 = hex::decode(transactions.get(i + 1).unwrap()).unwrap();
        }
        let a = sha256d(&mut tx1, &mut tx2);
        new_list.push(a);
    }
    create_merkle_root(new_list)
}

fn sha256d(a: &mut [u8], b: &mut [u8]) -> String {
    reverse_byte_array(a);
    reverse_byte_array(b);

    let merged_bytes = [a, b].concat().leak();

    let mut hash = sha256(&sha256(merged_bytes));
    reverse_byte_array(&mut hash);

    hex::encode(&hash)
}

fn sha256(bytes: &[u8]) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    let hash = hasher.finalize();
    let x: [u8; 32] = hash.into();
    Vec::from(x)
}

// TODO: User std::mem::swap instead of manually swapping
fn reverse_byte_array(arr: &mut [u8]) {
    let mut s = 0;
    let mut e = arr.len() - 1;
    while s < e {
        let temp = arr[e];
        arr[e] = arr[s];
        arr[s] = temp;
        s += 1;
        e -= 1;
    }
}

pub fn hash_to_binary_representation(hash: Hash) -> String {
    let mut res: String = String::default();

    for c in hash {
        res.push_str(&format!("{:b}", c));
    }
    res
}

#[cfg(test)]
mod test {
    use crate::utility::file_utils::read_file_line_by_line;
    use crate::utility::hash_utils::find_merkle_root;

    #[test]
    fn test_find_merkle_root_bitcoin_block170_transactions() {
        let filepath = "resources/BitcoinBlock170TXs.txt";
        let data = read_file_line_by_line(filepath);

        let bitcoin_block170_merkle_root = "7dac2c5666815c17a3b36427de37bb9d2e2c5ccec3f8633eb91a4205cb4c10ff";
        let root = find_merkle_root(data.unwrap());
        assert_eq!(bitcoin_block170_merkle_root.to_string(), root);
    }

    #[test]
    fn test_find_merkle_root_bitcoin_block286819_transactions() {
        let filepath = "resources/BitcoinBlock286819TXs.txt";
        let data = read_file_line_by_line(filepath);

        let bitcoin_block286819_merkle_root = "871714dcbae6c8193a2bb9b2a69fe1c0440399f38d94b3a0f1b447275a29978a";
        let root = find_merkle_root(data.unwrap());
        assert_eq!(bitcoin_block286819_merkle_root.to_string(), root);
    }
}
