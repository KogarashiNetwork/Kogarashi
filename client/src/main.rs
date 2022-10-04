use codec::Compact;
use sp_keyring::{sr25519::sr25519::Pair, AccountKeyring};
use sp_runtime::{AccountId32, MultiAddress};
use substrate_api_client::{compose_extrinsic, Api, XtStatus};

fn main() {
    let url = "ws://127.0.0.1:9944";
    let from = AccountKeyring::Alice.pair();

    let api = Api::<Pair>::new(url.to_string())
        .map(|api| api.set_signer(from.clone()).unwrap())
        .unwrap();

    let to = AccountKeyring::Bob.to_account_id();
    match api.get_account_data(&to).unwrap() {
        Some(bob) => println!("[+] Bob's Free Balance is is {}\n", bob.free),
        None => println!("[+] Bob's Free Balance is is 0\n"),
    }

    let xt = compose_extrinsic(
        &api,
        "Balances",
        "transfer",
        (
            MultiAddress::<AccountId32, ()>::Id(to.clone()),
            Compact(1000),
        ),
    );

    println!(
        "Sending an extrinsic from Alice (Key = {:?}),\n\nto Bob (Key = {})\n",
        from.as_ref().public,
        to
    );

    println!("[+] Composed extrinsic: {:?}\n", xt);

    // send and watch extrinsic until finalized
    let tx_hash = api
        .send_extrinsic(xt.hex_encode(), XtStatus::InBlock)
        .unwrap();
    println!("[+] Transaction got included. Hash: {:?}\n", tx_hash);

    // verify that Bob's free Balance increased
    let bob = api.get_account_data(&to).unwrap().unwrap();
    println!("[+] Bob's Free Balance is now {}\n", bob.free);
}
