## 概要

このリポジトリは、CodiMDの全文表示サイトを作成するものです
<img width="929" height="385" alt="image" src="https://github.com/user-attachments/assets/e70c9b44-a85f-4668-8664-f5237b4f10c2" />

## 使い方
[Rust](https://rust-lang.org/ja/tools/install/)をインストール後に
```Shell
git clone https://github.com/8bitTD/codimd_display
cd codimd_display
cargo run --release
```
ウェブブラウザで `http://ユーザー名:ポート番号/codimd_display` にアクセスする
## 補足
* [src/main.rsの6行目~10行目](https://github.com/8bitTD/codimd_display/blob/b3dddab2c82fcbb31527d3496588a8899fd447a1/src/main.rs#L6-L10)を自分の環境に合うように書き換えてください
* `https://ユーザー名:ポート番号/codimd_json`にアクセスするとjsonで取得できるサイトが表示されます
  <img width="861" height="511" alt="image" src="https://github.com/user-attachments/assets/08b71cdb-629d-4f1c-bb46-7eb0f4b75f5a" />


