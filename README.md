# iota-rust-cli
A set of cli binaries that expose the wallet functionality.


Scope
Develop a set of standalone Rust CLI binaries that expose wallet functionality, similar to the way the `git` command works.

## Links
- https://github.com/iotaledger/iota.rs
- https://github.com/iotaledger/iota.lib.js#standard-api

## Features:

- Address generation (`iotawallet generate-address <SECURITY> <KEYINDEX>`)  
- Balance checking (`iotawallet balance <ADDRESS>`)  
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


## Completed


`iotawallet balance <ADDRESS>`  

examples:

./iotawallet -h eugene.iota.community balance -a \
  UBMZUJIEQEJOJREZEI9JQ9RFYKEIAZFYS9DQYLIZJ9SBUKNSPY9CRRRZYNEVDZDEOLMLHPJTSEFFSMQKZU9ZJRFCFZ

/iotawallet -h eugene.iota.community  tx-status \
  QHBYXQWRAHQJZEIARWSQGZJTAIITOZRMBFICIPAVD9YRJMXFXBDPFDTRAHHHP9YPDUVTNOFWZGFGWMYHEKNAGNJHMW

./build_and_test.sh tx-status \
  QHBYXQWRAHQJZEIARWSQGZJTAIITOZRMBFICIPAVD9YRJMXFXBDPFDTRAHHHP9YPDUVTNOFWZGFGWMYHEKNAGNJHMW

./build_and_test.sh tx-show \
  UBMZUJIEQEJOJREZEI9JQ9RFYKEIAZFYS9DQYLIZJ9SBUKNSPY9CRRRZYNEVDZDEOLMLHPJTSEFFSMQKZU9ZJRFCFZ


## TODO:

`iotawallet generate-address <SECURITY> <KEYINDEX>`  
`iotawallet balance <ADDRESS>`  
`iotawallet tx-status <TXHASH>`  
`iotawallet bundle-show <BUNDLEHASH|TAILHASH>`  
`iotawallet tx-show <TXHASH>`  
`iotawallet send <RECIPIENT> <AMOUNT>`  
