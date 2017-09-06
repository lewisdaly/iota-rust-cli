# iota-rust-cli
A set of cli binaries that expose the wallet functionality.


Scope
Develop a set of standalone Rust CLI binaries that expose wallet functionality, similar to the way the `git` command works.

## Links
- https://github.com/iotaledger/iota.rs

## Features:

- Address generation (`iotawallet generate-address <SECURITY> <KEYINDEX>`)  
- Balance checking (`iotawallet balance <SEED>`)  
- Transaction confirmation status check (`iotawallet tx-status <TXHASH>`)  
- Bundle details (`iotawallet bundle-show <BUNDLEHASH|TAILHASH>`)  
- Transaction details (`iotawallet tx-show <TXHASH>`)  
- Transfer (`iotawallet send <RECIPIENT> <AMOUNT>`)  

The output could just be pretty-printed JSON for the bundledetails/transactiondetails.

## Example:
```
> iotawallet-generate-address
IOTA CLI Wallet (generate address)

USAGE:
    iotawallet-generate-address [FLAGS] [OPTIONS] <SECURITY> <KEYINDEX> [SUBCOMMAND]

FLAGS:
    -h, --help       Prints this message
    -v               Sets the level of verbosity
    -V, --version    Prints version information

OPTIONS:
    -s, --seed <SEED> Sets the seed to use
    -p, --promt Will prompt for the seed to use

ARGS:
    KEYINDEX the key index to generate
    SECURITY the security level to use for address generation

SUBCOMMANDS:
    help    Prints this message
    test    Controls testing features
```
