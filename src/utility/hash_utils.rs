use sha2::{Digest, Sha256};

pub fn find_merkle_root(transactions: &mut Vec<String>) -> String {
    println!("find merkle tree for transactions={:?}", transactions);
    let list: Vec<String> = create_merkle_root(transactions);
    list[0].clone()
}

fn create_merkle_root(transactions: &mut Vec<String>) -> Vec<String> {
    if transactions.len() == 1 {
        return vec![transactions[0].clone()];
    }

    if transactions.len() % 2 == 1 {
        transactions.push(transactions.last().unwrap().to_owned());
    }
    let mut tx1;
    let mut tx2;
    let mut new_list: Vec<String> = Vec::new();
    for i in (0..transactions.len()).step_by(2) {
        tx1 = hex::decode(transactions.get(i).unwrap()).unwrap();
        tx2 = hex::decode(transactions.get(i + 1).unwrap()).unwrap();

        let a = sha256d(&mut tx1, &mut tx2);
        new_list.push(a);
    }
    create_merkle_root(&mut new_list)
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
    let mut hash = hasher.finalize();
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

#[cfg(test)]
mod test {
    use crate::utility::hash_utils::find_merkle_root;

    #[test]
    fn test_find_merkle_root_bitcoin_block170_transactions() {
        let mut bitcoin_block170_transactions = vec![
            "b1fea52486ce0c62bb442b530a3f0132b826c74e473d1f2c220bfa78111c5082".to_string(),
            "f4184fc596403b9d638783cf57adfe4c75c605f6356fbc91338530e9831e9e16".to_string(),
        ];

        let bitcoin_block170_merkle_root = "7dac2c5666815c17a3b36427de37bb9d2e2c5ccec3f8633eb91a4205cb4c10ff";
        let root = find_merkle_root(&mut bitcoin_block170_transactions);
        assert_eq!(bitcoin_block170_merkle_root.to_string(), root);
    }
}
