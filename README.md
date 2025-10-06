## 概要

このリポジトリは、CodiMDの全文表示サイトを作成するためのソースコードです
<img width="1697" height="925" alt="image" src="https://github.com/user-attachments/assets/cb52679e-9fba-4afa-ba10-702d157ce494" />

## 使い方
[Rust](https://rust-lang.org/ja/tools/install/)をインストール後に
```Shell
git clone https://github.com/8bitTD/codimd_display
cd codimd_display
cargo run --release
```
ウェブブラウザで `http://ユーザー名:ポート番号/codimd_display` にアクセスする
## 補足
[main.rsの6行目~10行目](https://github.com/8bitTD/codimd_display/blob/84b7d7c0d7ada673ca1fed7e42b59479edd2d92d/src/main.rs#L6-L10)を自分の環境に合うように書き換えてください

