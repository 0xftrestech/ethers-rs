#[tokio::test]
#[cfg(not(feature = "celo"))]
async fn nonce_manager() {
    use ethers_core::types::*;
    use ethers_middleware::{Client, NonceManager};
    use ethers_providers::{Http, Middleware, Provider};
    use ethers_signers::Wallet;
    use std::convert::TryFrom;
    use std::time::Duration;

    let provider =
        Provider::<Http>::try_from("https://rinkeby.infura.io/v3/fd8b88b56aa84f6da87b60f5441d6778")
            .unwrap()
            .interval(Duration::from_millis(2000u64));

    let wallet = "59c37cb6b16fa2de30675f034c8008f890f4b2696c729d6267946d29736d73e4"
        .parse::<Wallet>()
        .unwrap();
    let address = wallet.address();

    let provider = Client::new(provider, wallet);

    // the nonce manager must be over the Client so that it overrides the nonce
    // before the client gets it
    let provider = NonceManager::new(provider, address);

    let nonce = provider
        .get_transaction_count(address, Some(BlockNumber::Pending))
        .await
        .unwrap()
        .as_u64();

    let mut tx_hashes = Vec::new();
    for _ in 0..10 {
        let tx = provider
            .send_transaction(TransactionRequest::pay(address, 100u64), None)
            .await
            .unwrap();
        tx_hashes.push(tx);
    }

    let mut nonces = Vec::new();
    for tx_hash in tx_hashes {
        nonces.push(
            provider
                .get_transaction(tx_hash)
                .await
                .unwrap()
                .unwrap()
                .nonce
                .as_u64(),
        );
    }

    assert_eq!(nonces, (nonce..nonce + 10).collect::<Vec<_>>())
}
