```
# ローカル確認
$ cargo run --release
# Linuxターゲットインストール
$ rustup target add x86_64-unknown-linux-musl
# Linuxビルド
$ cargo build --release --target=x86_64-unknown-linux-musl
```