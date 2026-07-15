# Bitcoinn
# install Rust and confirm the version

```curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y```
```source $HOME/.cargo/env```
```rustc --version```

## Runing Bitcoin Core 

# Download Bitcoin Core
`wget https://bitcoincore.org/bin/bitcoin-core-27.0/bitcoin-27.0-x86_64-linux-gnu.tar.gz`

# Extract
`tar -xzf bitcoin-27.0-x86_64-linux-gnu.tar.gz`

# Move to a convenient location
`sudo mv bitcoin-27.0/bin/* /usr/local/bin/`

# Verify
`bitcoind --version`
