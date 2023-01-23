#!/usr/bin/env bash
set -o errexit -o nounset -o pipefail

cd curl_adapter
cargo update --aggressive
marine build --release

cd ../facade
cargo update --aggressive
marine build --release
cd ..

mkdir -p artifacts
rm -f artifacts/*.wasm

cp curl_adapter/target/wasm32-wasi/release/curl_adapter.wasm artifacts/
cp facade/target/wasm32-wasi/release/facade.wasm artifacts/fs_facade.wasm
## curl -X POST -F file=@artifacts/facade.wasm "http://62.195.116.80:5001/api/v0/add"
curl -X POST -F file=@artifacts/fs_facade.wasm "http://64.227.70.116:5001/api/v0/add"
output=$(curl -X POST -F file=@artifacts/fs_facade.wasm "http://64.227.70.116:5001/api/v0/add")
hash=$(echo "$output" | grep -o -P '(?<="Hash":").*(?=","Size")')
echo $hash
cd ~/Documents/chronicles/solidity/service-store-contract/
npx hardhat update --name prdao_fs_service --cid $hash --network rinkeby;
cd ~/Documents/chronicles/fluence/public-record/fs-service/