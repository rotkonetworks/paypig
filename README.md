# paypig

Automated payout executor for Polkadot treasury spends.  
Scans treasury proposals, filters for valid/eligible ones, and submits payouts via signed transactions.


## Purpose
Tired of spending time on reading unnessecary reproposals due to poor polkadot governance ux requiring manual user actions.  
Full problem statement in Leemo's [thread](https://x.com/LeemoXD/status/1909201896314642615).

## Install
```sh
wget -qO /usr/local/bin/paypig https://github.com/rotkonetworks/paypig/releases/latest/download/paypig-x86_64
chmod +x /usr/local/bin/paypig
mkdir -p /etc/paypig && cp .env .keyfile /etc/paypig && chmod 600 /etc/paypig/.keyfile
cargo install subxt-cli && subxt metadata -f bytes > /etc/paypig/polkadot.scale
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
## Tips

To fund the bot 1pay2L9bxFYXYNFUmTtWMCT5n9iUUHA2rCoDdDKFx1cUtnF   
To fund the developer 1Qrotkokp6taAeLThuwgzR7Mu3YQonZohwrzixwGnrD1QDT  

## Screenshot
![image](https://github.com/user-attachments/assets/75d38abe-4395-4c3f-8050-892ba57d5500)
