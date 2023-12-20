```shell
cd ../guest/grpc_service
cargo expand --target wasm32-wasi
cargo build --target wasm32-wasi


export WARG_OPERATOR_KEY="ecdsa-p256:I+UlDo0HxyBBFeelhPPWmD+LnklOpqZDkrFP5VduASk="
./warg-server --content-dir ~/content

./warg config --registry http://127.0.0.1:8090 --registries-dir ./local/registries --content-dir ./local/content ./local/config.json
./warg key new 127.0.0.1
./warg publish init grpc-service
./warg publish release --name grpc-service --version 0.1.0 grpc_service.wasm
./warg info grpc-service

./warg download --version 0.1.0 grpc-service --config ./local/config.json
```

```shell
 ./wit-bindgen rust --macro-export ../wit/wasi-messaging/wit
```