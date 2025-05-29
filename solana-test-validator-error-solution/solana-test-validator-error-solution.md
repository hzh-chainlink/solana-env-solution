# solana 验证节点报错解决方案

#### 错误参考

------

![image-20250529095949949](.\solana-test-validator-error-solution.assets\image-20250529095949949.png)



#### **快速分析**

------

首先，这个过程涉及了两套东西，一个是 Solana CLI，一个是 Anchor 框架，当同时使用它们时，我们需要让配置保持一致。其中，Solana 的配置在项目根目录执行 `solana config get` 即可得到，Anchor 的配置在根目录的 Anchor.toml 文件夹下。

我们的目的是在本地或 Devnet 部署并测试我们的程序。



#### 0. 环境检查

首先检查 Solana 开发环境是否安装，

```
rustc --version		# 输出 `rustc 1.86.0 (05f9846f8 2025-03-31)`
anchor --version	# 输出 `anchor-cli 0.31.0
```

如果输出报错，请参考 Noah 的：

【Solana教程EP07-Solana开发环境搭建与Anchor框架讲解】 https://www.bilibili.com/video/BV1JkN8esEW8/?share_source=copy_web&vd_source=227a373d3ff986e3c03fd23a03bec702



#### 1. 初始化项目

为了测试，我们需要初始化一个 Anchor 项目（这里选择在当前用户目录下创建），

```
cd ~ && anchor init demo
```

当项目创建完成后，我们通过编译命令来检查项目初始化是否正常，

```
cd demo && anchor build
```

编译生成的相关文件会放在根目录的 `target` 文件夹下，当看到 `Finished` 说明编译成功。

注意，假如您手动移除过 target 文件夹，可能会造成

- target/deploy/demo-keypair.json 项目私钥（注意是项目私钥不是钱包私钥）
- 代码中的 declare_id! 项目公钥
- Anchor.toml 中的 demo 项目地址，即项目公钥 

这三者不一致，这会导致部署时错误，此时执行 `anchor keys sync`



#### 2. 本地测试

我们分析过，关键是让全局工具 Solana CLI 的配置和项目中的 Anchor 配置保持一致，来查看一下项目中的 `Anchor.toml` 文件：

```
[provider]
cluster = "localnet"
wallet = "~/.config/solana/id.json"
```

可以清楚地看到：
	当前的 RPC 设置为 “本地”，其中 cluster 指集群，是 Solana 描述某个网络的专有名词，这里被指定为 localnet，即本地网络；
	当前的钱包设置为用户目录的一个默认钱包，这个钱包实际是通过 `solana-keygen new` 命令生成的，如果你想得到这个钱包私钥，执行 `cat ~/.config/solana/id.json`

接下来检查 Solana CLI 的配置，执行 `solana config get`：

```
user@DESKTOP-832V6GI:~/demo$ solana config get
Config File: /home/user/.config/solana/cli/config.yml
RPC URL: http://localhost:8899 
WebSocket URL: ws://localhost:8900/ (computed)
Keypair Path: /home/user/.config/solana/id.json 
Commitment: confirmed
```

假如你的 RPC URL 和我不一致，应该执行 `solana config set --url localhost`

##### 2.1 使用 Anchor 完成本地测试

直接执行 `anchor test`

在这个过程中，本地会启动一个**临时的**测试 RPC，使用 8899 端口，然后内置水龙头会自动给钱包转账以支付接下来的部署、交互费用，接着执行 /tests 文件夹下的测试文件，最后销毁这一切，就像什么都没发生过。

所以在使用 anchor test 时，不应该执行 solana-test-validator，因为 Solana CLI 会启用一条**持久的**本地测试链，这会持续占用 8899 端口，导致 anchor 的临时测试链无法正常启动。

##### 2.2 使用 Anchor 在持久链上完成本地测试

首先，使用 Solana CLI 启用本地的持久测试链，

```
solana-test-validator
```

然后，使用 Anchor 将程序部署到它上面，

```
anchor test --skip-local-validator
```

这个参数的意思是，跳过启动本地验证节点，意思是 anchor test 默认会启动一个临时的本地链，现在不启动了，那它会部署到哪呢，由 anchor.toml 内的配置已经指定好了，就是本地 Solana CLI 启用的持久链 8899 上。

如果您不相信，这是执行 `anchor test --skip-local-validator` 后的日志：

```
user@DESKTOP-832V6GI:~/demo$ anchor test --skip-local-validator
   Compiling demo v0.1.0 (/home/user/demo/programs/demo)
    Finished `release` profile [optimized] target(s) in 1.27s
   Compiling demo v0.1.0 (/home/user/demo/programs/demo)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.07s
     Running unittests src/lib.rs (/home/user/demo/target/debug/deps/demo-a1b946163146555c)
Deploying cluster: http://127.0.0.1:8899
Upgrade authority: /home/user/.config/solana/id.json
Deploying program "demo"...
Program path: /home/user/demo/target/deploy/demo.so...
Program Id: 9YPonsJ3j4adAY4R9sanfBzctFUsVLCv2XKqjbucGwng

Signature: 4inwdDWE8hTRSVCsfg79bAQV9ADRdeomVNRkUDoLofvifqiRZ1BpdM3WGGQHk1Z6t9w9n4qmRaXYU8KVvY2PybdN

Deploy success

Found a 'test' script in the Anchor.toml. Running it as a test suite!

Running test suite: "/home/user/demo/Anchor.toml"

yarn run v1.22.22
$ /home/user/demo/node_modules/.bin/ts-mocha -p ./tsconfig.json -t 1000000 'tests/**/*.ts'


  demo
Your transaction signature 4eFxtryHreUDtFHBh8raGqzAHgrsXjDVGu1rLtH37KhquDSJorx94ti1zLvikQDbNeD9kn2iZ3ocpCHNqtXW3vqK
    ✔ Is initialized! (331ms)


  1 passing (333ms)

Done in 1.35s.
```

可以看到 Program Id 是：

9YPonsJ3j4adAY4R9sanfBzctFUsVLCv2XKqjbucGwng

我们可以用 Solana CLI 查看这个程序的信息：

```
solana program show 9YPonsJ3j4adAY4R9sanfBzctFUsVLCv2XKqjbucGwng
```

输出的信息表明它已经成功部署在本地持久链上！

```
user@DESKTOP-832V6GI:~/demo$ solana program show 9YPonsJ3j4adAY4R9sanfBzctFUsVLCv2XKqjbucGwng

Program Id: 9YPonsJ3j4adAY4R9sanfBzctFUsVLCv2XKqjbucGwng
Owner: BPFLoaderUpgradeab1e11111111111111111111111
ProgramData Address: 9Zk1yMU4pMEHun54znBubyXcpVjnK8Gpa4R8uLJW1aU6
Authority: 9gpCbQa8zQTbZdfHGXau3yMaz7WowtUyzz8vkHVXrAhz
Last Deployed In Slot: 933
Data Length: 181768 (0x2c608) bytes
Balance: 1.26630936 SOL
```



#### 3. 在线测试

众所周知，Solana 实际上有 4 个网络，对于 Solana CLI，我们使用这些命令切换：

```
solana config set --url mainnet-beta
solana config set --url devnet
solana config set --url localhost
solana config set --url testnet
```

在线测试一般选择 devnet，同样地，首先配置 Solana CLI，

```
solana config set --url devnet
```

此时查看钱包余额，就是线上 Devnet 该钱包的余额，如果没有余额，需要到 Solana 水龙头领取：

https://faucet.solana.com/

```
solana balance # 输出 7.27520828 SOL
```

更改 Anchor 配置的 cluster 为 devnet：

```
[toolchain]
package_manager = "yarn"

[features]
resolution = true
skip-lint = false

[programs.localnet]
demo = "9YPonsJ3j4adAY4R9sanfBzctFUsVLCv2XKqjbucGwng"

[registry]
url = "https://api.apr.dev"

[provider]
cluster = "devnet"
wallet = "~/.config/solana/id.json"

[scripts]
test = "yarn run ts-mocha -p ./tsconfig.json -t 1000000 tests/**/*.ts"

```

执行 `anchor deploy` 尝试在 Devnet 上部署，如果卡住了，说明当前网络到官方 Devnet RPC 不通畅，这里提供两种解决办法。

##### 3.1 检查 WSL 的网络配置

如果您是 Windows 11 系统，在开始菜单搜索 `WSL Settings`，然后将网络模式设置为 Mirrored，这将使 WSL 自动使用 Windows 下的代理软件🪜：

![image-20250529114409115](.\solana-test-validator-error-solution.assets\image-20250529114409115.png)

这种方法配置好以后需要重启 WSL，在 Windows 的 cmd 中执行 `wsl --shutdown`



还有一种方案是找到代理软件的端口，并在 WSL 中配置，现在默认安装的 WSL 版本都是 WSL2：

https://www.cnblogs.com/RioTian/p/17986762



##### 3.2 使用私人 RPC 节点

公共 RPC `https://api.devnet.solana.com` 或因为使用人数过多，而无法响应，我们可以选择一家 RPC 节点提供服务商，比如 HELIUS：

https://dashboard.helius.dev/endpoints?projectId=c492cd4a-9bbc-48f7-aec9-11236c768aef

注册以后，可以找到自己的 Devnet RPC：

![image-20250529123143190](.\solana-test-validator-error-solution.assets\image-20250529123143190.png)

然后修改 Anchor.toml 的 cluster：

```
[provider]
cluster = "https://devnet.helius-rpc.com/?api-key=f251df6c-7f51-4b1c-9394-92daafd620ee"
wallet = "~/.config/solana/id.json"[provider]
cluster = "https://devnet.helius-rpc.com/?api-key=f251df6c-7f51-4b1c-9394-92daafd620ee"
wallet = "~/.config/solana/id.json"
```

确保您处于项目根目录 `/demo`，执行以下命令时仍然可能因网络原因中断，如果发现卡住，只需按 `Ctrl + C` 后，重新执行即可。另外，免费版 RPC 有请求速率限制，频繁请求，也会报错。

###### 3.2.1 部署合约

`anchor deploy`

```
user@DESKTOP-832V6GI:~/demo$ anchor deploy
Deploying cluster: https://devnet.helius-rpc.com/?api-key=f251df6c-7f51-4b1c-9394-92daafd620ee
Upgrade authority: /home/user/.config/solana/id.json
Deploying program "demo"...
Program path: /home/user/demo/target/deploy/demo.so...
Program Id: 9YPonsJ3j4adAY4R9sanfBzctFUsVLCv2XKqjbucGwng

Signature: 2PYCPgtTEPLcJJZX1nWN2XcUmMwHsdopHMg3Gg8bQQAGoDzqq4K2LV7r5StagRR7CpEKaV7QCt1pJXUkyJC82jiU

Deploy success
```

###### 3.2.2 更新合约

`anchor build && anchor deploy`

###### 3.2.3 测试合约

`anchor test` 

```
user@DESKTOP-832V6GI:~/demo$ anchor test
    Finished `release` profile [optimized] target(s) in 0.11s
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.07s
     Running unittests src/lib.rs (/home/user/demo/target/debug/deps/demo-a1b946163146555c)
Deploying cluster: https://devnet.helius-rpc.com/?api-key=f251df6c-7f51-4b1c-9394-92daafd620ee
Upgrade authority: /home/user/.config/solana/id.json
Deploying program "demo"...
Program path: /home/user/demo/target/deploy/demo.so...
Program Id: 9YPonsJ3j4adAY4R9sanfBzctFUsVLCv2XKqjbucGwng

Signature: 3h8wK6FmKsfiVTf4S9g7cDbkUGTQ58utwhgyq9bLeWrWNFapTeCEuSntrKgqPijNb5tfys2SRrhQ4uXzbTYTEW79

Deploy success

Found a 'test' script in the Anchor.toml. Running it as a test suite!

Running test suite: "/home/user/demo/Anchor.toml"

yarn run v1.22.22
$ /home/user/demo/node_modules/.bin/ts-mocha -p ./tsconfig.json -t 1000000 'tests/**/*.ts'


  demo
Your transaction signature 4hedVcZdBSRLiUL82gtvi8RpLeTDLGdevvdjFT54iZAgGcfLKEkHcTXemR1Reo4uj3bTcCaFPZ8wFdoVCngo4279
    ✔ Is initialized! (2525ms)


  1 passing (3s)

Done in 5.25s.
```

###### 3.2.4 验证合约

https://solana.com/developers/guides/advanced/verified-builds

###### 3.2.5 撤销部署

`solana program close 9YPonsJ3j4adAY4R9sanfBzctFUsVLCv2XKqjbucGwng --bypass-warning`

在 Devnet 区块链浏览器查看：

https://explorer.solana.com/address/9YPonsJ3j4adAY4R9sanfBzctFUsVLCv2XKqjbucGwng/verified-build?cluster=devnet

注意，撤销部署后虽然可以回收部署时花费的一些 SOL，但该地址不能再使用，如果您想重新部署该程序，需要手动移除 `/target` 文件夹后，重新编译 `anchor build`，并同步新的地址 `anchor keys sync` ，最后再执行部署命令 `anchor deploy`。

#### 4. 参考资源

至此您以学会了使用 Anchor 框架在本地或 Devnet 上完成 Solana Program（智能合约）的开发、部署、测试流程，进一步参考 Anchor 官方文档：

https://www.anchor-lang.com/docs





> *最后编辑日期：2025年 5月 29日*
