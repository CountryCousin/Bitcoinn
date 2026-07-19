# Bitcoinn

# clone the Repo

# install Rust and confirm the version

```curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y```

```source $HOME/.cargo/env```

```rustc --version```

## Running Bitcoin Core 

# Download Bitcoin Core
`wget https://bitcoincore.org/bin/bitcoin-core-27.0/bitcoin-27.0-x86_64-linux-gnu.tar.gz`

# Extract
`tar -xzf bitcoin-27.0-x86_64-linux-gnu.tar.gz`

# Move to a convenient location
`sudo mv bitcoin-27.0/bin/* /usr/local/bin/`

# Verify
`bitcoind --version`

# Create config and env files 
1. locate `bitcoin.conf` at `Bitcoinn/bitcoin-cli-rs/bitcoin-27.0/bitcoin.conf`
2. replace the content of `bitcoin.cong` with:
   ```
   Global options
   server=1
   daemon=1

   Options for mainnet
   [main]

   Options for testnet
   [test]

   Options for signet
   [signet]

   Options for regtest

   [regtest]

   rpcport=18443

   rpcuser=<replace_with_user>

   rpcpassword=<replace_with_your_password>

   ```
# create env file 
1. at `Bitcoinn/.env`, populate it with the values below:
```
RPC_URL=http://127.0.0.1:18443

RPC_USER=<replace with preferred user>
RPC_PASSWORD=<replace_with_your password>

WALLET=wallet.dat
```
# Copy the configuration(content of bitcoin.conf) to Bitcoin Core's default location
1. Create directory: ` mkdir -p ~/.bitcoin`
2. run `cp /workspaces/codespaces-blank/Bitcoinn/bitcoin-cli-rs/bitcoin-27.0/bitcoin.conf \
   ~/.bitcoin/bitcoin.conf` to make the copy (obtain the absolute path of your own bitcoin.conf)
   
4. Verify the content matches the content of the first `bitcoin.conf` using:
`cat ~/.bitcoin/bitcoin.conf`

# Start bitcoind
`bitcoind -regtest -daemon`

#  Test wallet functionality
1. Create a wallet using: `bitcoin-cli -regtest createwallet "wallet"`
2. If it says wallet already exists, load it up using: `bitcoin-cli -regtest loadwallet "wallet"` 

# Generate some blocks

1. Create your wallet address using:
 `bitcoin-cli -regtest getnewaddress`

 2. Generate the number of blocks you want. In this case, we created 300:

` bitcoin-cli -regtest generatetoaddress 300 < address_gotten_from_"bitcoin-cli -regtest getnewaddress">`

# check your balance
`cargo run -- balance`

# Test more RPC methods
`cargo run -- rpc getdifficulty `

`cargo run -- rpc getblockchaininfo`

`cargo run -- rpc getbestblockhash`

`cargo run -- rpc getblockcount `

`cargo run -- rpc getnetworkinfo`

`cargo run -- rpc getblockhash 200`

`cargo run -- rpc getblock <hash>`

use `cargo clean` if the need be
