# paypig

Automated payout executor for Polkadot treasury spends.
Scans treasury proposals, filters for valid/eligible ones, and submits payouts via signed transactions.

## Install
```sh
wget -qO /usr/local/bin/paypig https://github.com/rotkonetworks/paypig/releases/latest/download/paypig-x86_64
chmod +x /usr/local/bin/paypig
mkdir -p /etc/paypig && cp .env .keyfile /etc/paypig && chmod 600 /etc/paypig/.keyfile
(crontab -l 2>/dev/null; echo "0 0 1-7,15-21 * 3 cd /etc/paypig && /usr/local/bin/paypig pay >> /var/log/paypig.log 2>&1") | sort -u | crontab -
```

## Development
```sh
git clone https://github.com/rotkonetworks/paypig && cd paypig
cargo install subxt-cli
subxt metadata -f bytes > polkadot.scale
cargo run --release pay

# (optional) useful for development
# subxt codegen --url wss://rpc.polkadot.io > polkadot.rs && cargo fmt -- polkadot.rs
```
