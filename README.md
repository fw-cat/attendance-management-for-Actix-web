# Actix-web-Composer-Template

# Outline
* 本プロジェクトはDocker Compose上にActix Web Frameworkを設定するデフォルトテンプレートになります。

# Startup
~~~sh
$ docker compose up -d
~~~

## Recompile Startup
~~~sh
$ docker compose up --build -d
~~~

# Actix-webの起動コマンド
~~~sh
$ docker compose exec rust-app cargo run
~~~
