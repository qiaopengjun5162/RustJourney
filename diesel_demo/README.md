# diesel demo

```shell
diesel_demo on  master [?] via 🦀 1.75.0 via 🅒 base 
➜ echo DATABASE_URL=postgres://username:password@localhost/diesel_demo > .env

diesel_demo on  master [?] via 🦀 1.75.0 via 🅒 base 
➜ diesel setup

Creating migrations directory at: /Users/qiaopengjun/Code/rust/diesel_demo/migrations
Creating database: postgres
could not translate host name "db" to address: nodename nor servname provided, or not known


diesel_demo on  master [?] via 🦀 1.75.0 via 🅒 base 
➜ psql
psql (14.9 (Homebrew))
Type "help" for help.

qiaopengjun=# ^C
qiaopengjun=# 
\q

diesel_demo on  master [?] via 🦀 1.75.0 via 🅒 base took 4m 2.4s 
➜ diesel setup


diesel_demo on  master [?] via 🦀 1.75.0 via 🅒 base 
➜ diesel migration generate create_posts

Creating migrations/2024-01-28-092822_create_posts/up.sql
Creating migrations/2024-01-28-092822_create_posts/down.sql

diesel_demo on  master [?] via 🦀 1.75.0 via 🅒 base 
➜ diesel migration run

Running migration 2024-01-28-092822_create_posts

diesel_demo on  master [?] via 🦀 1.75.0 via 🅒 base 
➜  diesel migration generate --diff-schema create_posts
Creating migrations/2024-01-28-093619_create_posts/up.sql
Creating migrations/2024-01-28-093619_create_posts/down.sql

diesel_demo on  master [?] via 🦀 1.75.0 via 🅒 base 
➜ diesel migration run
Running migration 2024-01-28-093619_create_posts
Failed to run 2024-01-28-093619_create_posts with: Received an empty query

diesel_demo on  master [?] via 🦀 1.75.0 via 🅒 base 
➜ cargo run                  
error: a bin target must be available for `cargo run`

diesel_demo on  master [?] via 🦀 1.75.0 via 🅒 base 
➜ cargo build
    Updating crates.io index
warning: spurious network error (3 tries remaining): [7] Couldn't connect to server (Failed to connect to 127.0.0.1 port 33210 after 0 ms: Couldn't connect to server)
warning: spurious network error (2 tries remaining): [7] Couldn't connect to server (Failed to connect to 127.0.0.1 port 33210 after 0 ms: Couldn't connect to server)
warning: spurious network error (1 tries remaining): [7] Couldn't connect to server (Failed to connect to 127.0.0.1 port 33210 after 0 ms: Couldn't connect to server)
^C
```
