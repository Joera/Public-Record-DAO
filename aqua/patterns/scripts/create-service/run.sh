
#!/usr/bin/env bash
set -o errexit -o nounset -o pipefail

# compile service 
cd hello-service/facade
cargo update --aggressive
marine build --release
cd ..

mkdir -p artifacts
rm -f artifacts/*.wasm
#cp ../elasticsearch-adapter/artifacts/elasticsearch_adapter.wasm  artifacts/
#cp ../curl_adapter/artifacts/curl_adapter.wasm  artifacts/
cp ./facade/target/wasm32-wasi/release/facade.wasm artifacts/

output=$(curl -X POST -F file=@artifacts/facade.wasm "http://64.227.70.116:5001/api/v0/add")
cid=$(echo "$output" | grep -o -P '(?<="Hash":").*(?=","Size")')

cd ..
aqua -i ./main.aqua -o ./ --air --no-xor --no-relay --const 'CID = "'$cid'"' 
# upload to ipfs 
output=$(curl -X POST -F file=@./main.create_service.air 'http://64.227.70.116:5001/api/v0/add')
cid=$(echo "$output" | grep -o -P '(?<="Hash":").*(?=","Size")')
echo $cid
aqua run -i ./main.aqua -f 'deploy_script('$cid')' --addr krasnodar-01