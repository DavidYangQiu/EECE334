use serde::{Serialize,Deserialize};
use ring::signature::{self,Ed25519KeyPair, Signature, KeyPair, VerificationAlgorithm, EdDSAParameters};
use rand::Rng;
use ring::digest;
use super::hash::{Hashable, H256};
use crate::crypto::hash::tests::generate_random_hash;


//[Jan.15, 2023 for Warmup2]
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Transaction {
    pub input_data: Vec<input_mode>,
    pub output_data: Vec<output_mode>
}
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct input_mode{
    pub message: H256,
    //println!("input data success!")
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct output_mode{
    pub message: H256,
    //println!("input data su12ccess!")
}
// #[derive(Serialize,Deserialize,Debug,Default)]
// pub struct signed_Transaction{
// }
impl Hashable for Transaction {
    fn hash(&self) -> H256 {
        let m = bincode::serialize(&self).unwrap();
        digest::digest(&digest::SHA256, m.as_ref()).into()
    }
}
   
/// Create digital signature of a transaction
pub fn sign(t: &Transaction, key: &Ed25519KeyPair) -> Signature {
   // unimplemented!()
   let message: Vec<u8> = bincode::serialize(&t).unwrap();
   let hash_value: Vec<u8> = digest::digest(&digest::SHA256, &message).as_ref().to_vec();
   key.sign(&hash_value)
}

/// Verify digital signature of a transaction, using public key instead of secret key
pub fn verify(t: &Transaction, public_key: &[u8], signature: &[u8]) -> bool {
    //unimplemented!()
    //, public_key: &<Ed25519KeyPair as KeyPair>::PublicKey,
    let message: Vec<u8> = bincode::serialize(&t).unwrap();
    let hash_value: Vec<u8> = digest::digest(&digest::SHA256, &message).as_ref().to_vec();    
    let pubic_key = signature::UnparsedPublicKey::new(&signature::ED25519, public_key);
    pubic_key.verify(&hash_value, signature).is_ok()
}

#[cfg(any(test, test_utilities))]
mod tests {
    use super::*;
    use crate::crypto::key_pair;

    pub fn generate_random_transaction() -> Transaction {
       // Default::default()
        //unimplemented!()
        let test_data_1 = vec![input_mode{message: generate_random_hash()}];
        let test_data_2 = vec![output_mode{message: generate_random_hash(),}];

        Transaction{input_data:test_data_1, output_data:test_data_2,};
    }

    #[test]
    fn sign_verify() {
        let t = generate_random_transaction();
        let key = key_pair::random();
        let signature = sign(&t, &key);
        assert!(verify(&t, &(key.public_key()), &signature));
    }
}
