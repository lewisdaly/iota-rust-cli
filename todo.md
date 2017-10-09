# A simple way to track pending tasks...


## Basic commands:
iotawallet generate-address <SECURITY> <KEYINDEX> - I think this needs to be done locally. Maybe we can use the current rust api for this
iotawallet bundle-show <BUNDLEHASH|TAILHASH>
iotawallet send <RECIPIENT_ADDRESS> <AMOUNT>

## Other
- in api_commands, add defaults for "command" field in structs
- refactor with models, and proper deserialization
- read config from env variables
- better error handling
- implement log levels - controlled with env vars or flags
- implement some sort of test framework, especially for more complex commands
- better imports


## Cleanup:
iotawallet balance <ADDRESS>
iotawallet tx-status <TXHASH>
iotawallet tx-show <TXHASH> (I think this is kind of working...)



## DONE:



Is `bundle-show` the same as findTransactions?
And `tx-show` ?
