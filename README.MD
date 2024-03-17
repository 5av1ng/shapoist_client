# shapoist_client
a simple client for `Shapoist`

### compile

#### Testing

on desktop platform
```sh
cargo run --features desktop
```
on android platform
```sh
cargo ndk -t arm64-v8a -o app/src/main/jniLibs/ build
./gradlew build
./gradlew installDebug
```
on wasm platform
note: you need install [trunk](https://trunkrs.dev/) first
```sh
trunk serve --features desktop
```

#### Releasing
on desktop platform
```sh
cargo build --release
```
on android platform
```sh
cargo ndk -t arm64-v8a -o app/src/main/jniLibs/ build --release
./gradlew build
```
on wasm platform
```sh
trunk build --features desktop --release
```