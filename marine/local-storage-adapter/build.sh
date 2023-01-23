#!/usr/bin/env bash
set -o errexit -o nounset -o pipefail


## cd ../facade
cargo update --aggressive
marine build --release
## cd ..

mkdir -p artifacts
rm -f artifacts/*.wasm

cp target/wasm32-wasi/release/local_storage.wasm artifacts/localstorage_adapter.wasm
curl -X POST -F file=@artifacts/localstorage_adapter.wasm "http://64.227.70.116:5001/api/v0/add"
## curl -X POST -F file=@artifacts/elasticsearch_adapter.wasm "http://62.195.116.80:5001/api/v0/add"
output=$(curl -X POST -F file=@artifacts/localstorage_adapter.wasm "http://64.227.70.116:5001/api/v0/add")
hash=$(echo "$output" | grep -o -P '(?<="Hash":").*(?=","Size")')
echo $hash
cd ~/Documents/chronicles/solidity/service-store-contract/
npx hardhat update --name prdao_localstorage_adapter --cid $hash --network rinkeby;
cd ~/Documents/chronicles/fluence/public-record/local-storage-adapter/