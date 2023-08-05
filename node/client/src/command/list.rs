use sp_keyring::RedjubjubKeyring as AccountKeyring;

pub(crate) fn list_command() {
    println!("Alice: {:?}", AccountKeyring::Alice.to_account_id());
    println!("Bob: {:?}", AccountKeyring::Bob.to_account_id());
    println!("Charlie: {:?}", AccountKeyring::Charlie.to_account_id());
    println!("Dave: {:?}", AccountKeyring::Dave.to_account_id());
    println!("Eve: {:?}", AccountKeyring::Eve.to_account_id());
    println!("Ferdie: {:?}", AccountKeyring::Ferdie.to_account_id());
    println!("One: {:?}", AccountKeyring::One.to_account_id());
    println!("Two: {:?}", AccountKeyring::Two.to_account_id());
}
