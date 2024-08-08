useHttp
useWeb3
use{}
use{}
useSecp256k1
useFromStr
use

[]
async fn main()(){
    // 连接到 Sepolia 测试网络的 HTTP 提供者
    let("https://sepolia.infura.io/v3/cd35b901304d47d0b4976e1fa2cb8bad")
    let()

    // 直接使用私钥
    let""// 这里填写你的私钥
    let()expect("无效的私钥格式")
    let()
expect("无效的 web3 私钥")

    // 获取账户地址
    let()
    let()expect("无效的私钥")
    let()
    let()
    let(([1])[12])

    // 获取账户余额
    let()balance()await
("余额: {}")

    // 获取下一个交易的 nonce
    let()transaction_count()await

    // 构造交易
    let{
        toSome(("0x19F47e792660Da7D4aE59585eD0Dd88F0097C1fa")())// 替换为目标地址
(0_050_000_000_000_000_000u64)// 0.1 ETH
(21_000)
((20_000_000_000u64))
()
(())
(11155111)// Sepolia 网络的链 ID
()
    }

    // 签名交易
    let()()await
    
    // 发送交易
    let()()await
("交易哈希: {:?}")

(())
}
