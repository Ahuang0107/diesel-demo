# readme

```shell
brew install mysql
```

```shell
cargo install diesel_cli --no-default-features --features mysql
```

```shell
mysql -u root -p
```

```mysql
ALTER USER 'root'@'localhost' IDENTIFIED BY 'root-password';
```

```shell
diesel setup
diesel migration generate create_tables
```

```shell
diesel migration run
diesel migration redo
```