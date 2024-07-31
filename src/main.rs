use web3::transports::Http;
use web3::Web3;
use web3::types::{TransactionParameters, U256, H160};
use web3::signing::{Key, SecretKey as Web3SecretKey};
use secp256k1::Secp256k1;
use std::str::FromStr;
use tokio;

#[tokio::main]
async fn main() -> web3::Result<()> {
    // 连接到 Sepolia 测试网络的 HTTP 提供者
    let http = Http::new("https://sepolia.infura.io/v3/cd35b901304d47d0b4976e1fa2cb8bad")?;
    let web3 = Web3::new(http);

    // 直接使用私钥（请使用你自己的私钥）
    let private_key_hex = ""; // 这里填写你的私钥
    let private_key_bytes = hex::decode(private_key_hex).expect("无效的私钥格式");
    let web3_secret_key = Web3SecretKey::from_slice(&private_key_bytes)
        .expect("无效的 web3 私钥");

    // 获取账户地址
    let secp = Secp256k1::new();
    let secp_secret_key = secp256k1::SecretKey::from_slice(&private_key_bytes).expect("无效的私钥");
    let public_key = secp256k1::PublicKey::from_secret_key(&secp, &secp_secret_key);
    let public_key_bytes = public_key.serialize_uncompressed();
    let address = H160::from_slice(&web3::signing::keccak256(&public_key_bytes[1..])[12..]);

    // 获取账户余额
    let balance = web3.eth().balance(address, None).await?;
    println!("余额: {}", balance);

    // 获取下一个交易的 nonce
    let nonce = web3.eth().transaction_count(address, None).await?;

    // 构造交易
    let tx_object = TransactionParameters {
        to: Some(H160::from_str("0x19F47e792660Da7D4aE59585eD0Dd88F0097C1fa").unwrap()), // 替换为目标地址
        value: U256::from(0_050_000_000_000_000_000u64), // 0.1 ETH
        gas: U256::from(21_000),
        gas_price: Some(U256::from(20_000_000_000u64)),
        nonce: Some(nonce),
        data: web3::types::Bytes(Vec::new()),
        chain_id: Some(11155111), // Sepolia 网络的链 ID
        ..Default::default()
    };

    // 签名交易
    let signed_tx = web3.accounts().sign_transaction(tx_object, &web3_secret_key).await?;
    
    // 发送交易
    let result = web3.eth().send_raw_transaction(signed_tx.raw_transaction).await?;
    println!("交易哈希: {:?}", result);

    Ok(())
}
