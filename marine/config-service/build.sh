#!/usr/bin/env bash
set -o errexit -o nounset -o pipefail


cd ./facade
cargo update --aggressive
marine build --release
cd ..

mkdir -p artifacts
rm -f artifacts/*.wasm

cp ../elasticsearch-adapter/artifacts/elasticsearch_adapter.wasm  artifacts/
cp ../curl_adapter/artifacts/curl_adapter.wasm artifacts/

cp facade/target/wasm32-wasi/release/facade.wasm artifacts/config-service_facade.wasm
curl -X POST -F file=@artifacts/config-service_facade.wasm "http://64.227.70.116:5001/api/v0/add"