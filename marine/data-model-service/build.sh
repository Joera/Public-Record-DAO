#!/usr/bin/env bash
set -o errexit -o nounset -o pipefail

cd facade
cargo update --aggressive
marine build --release
cd ..

mkdir -p artifacts
rm -f artifacts/*.wasm
cp ../elasticsearch-adapter/artifacts/elasticsearch_adapter.wasm  artifacts/
cp ../curl_adapter/artifacts/curl_adapter.wasm  artifacts/

cp facade/target/wasm32-wasi/release/facade.wasm artifacts/data_model_facade.wasm
curl -X POST -F file=@artifacts/data_model_facade.wasm "http://64.227.70.116:5001/api/v0/add"
output=$(curl -X POST -F file=@artifacts/data_model_facade.wasm "http://64.227.70.116:5001/api/v0/add")
hash=$(echo "$output" | grep -o -P '(?<="Hash":").*(?=","Size")')
echo $hash
cd ~/Documents/chronicles/solidity/service-store-contract/
npx hardhat update --name prdao_data_model_service --cid $hash --network rinkeby;
cd ~/Documents/chronicles/fluence/public-record/data-model-service/