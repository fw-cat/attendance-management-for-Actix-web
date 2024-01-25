# 勤怠管理システム for Actix Web

# 概要
* RustのActix Web Frameworkを利用した勤怠管理システムです

## 環境説明
![Rust](https://img.shields.io/badge/Rust-1.75.0-bule)

# プロジェクトの起動
## 01 - Startup or Recompile Startup
~~~sh
## 通常時の起動
$ docker compose up -d
## 再ビルド後、起動
$ docker compose up --build -d
~~~

## 02 - Actix-webの起動コマンド
~~~sh
$ docker compose exec rust-app cargo run
~~~
