# A simple way to track pending tasks...


## Basic commands:
 iotawallet tx-show <TXHASH>
  #tx-show is essentially `getTrytes` + transactionObject. You'll need https://github.com/iotaledger/iota.rs/blob/master/models/src/v1/view.rs for this.
 iotawallet bundle-show <BUNDLEHASH|TAILHASH>
 iotawallet generate-address <SECURITY> <KEYINDEX> - I think this needs to be done locally. Maybe we can use the current rust api for this
 iotawallet send <RECIPIENT_ADDRESS> <AMOUNT>

## Other
- refactor with models, and proper deserialization
- read config from env variables
- better error handling
- implement log levels - controlled with env vars or flags
- implement some sort of test framework, especially for more complex commands
- better imports


##DONE:
iotawallet balance <ADDRESS>
iotawallet tx-status <TXHASH>





Is `bundle-show` the same as findTransactions?
And `tx-show` ?
