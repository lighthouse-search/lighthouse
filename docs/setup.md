# Prerequisites
- [Elasticsearch](https://www.elastic.co/downloads/elasticsearch)
- [MariaDB](https://mariadb.com)

### Prerequisites via Docker

Here are some Docker commands that can quickly set up prerequisites for you. (**Please change ``MARIADB_ROOT_PASSWORD``!!**)
#### [Elasticsearch](https://hub.docker.com/_/elasticsearch)
```
docker run -d --name elasticsearch --net somenetwork -p 9200:9200 -p 9300:9300 -e "discovery.type=single-node" elasticsearch:tag
```

#### [Mariadb](https://hub.docker.com/_/mariadb)
```
docker run --detach --name some-mariadb --env MARIADB_ROOT_PASSWORD=my-secret-pw --env MARIADB_USER=myuser --env MARIADB_USER_PASSWORD mariadb:latest
```

# Setup

## 1. Set environment variables
These environment variables will be referenced in your configuration. Sensitive information (like tokens) is **never** stored in configurations for security.
```
export lighthouse_config=$(cat ../example/coastguard-dev-config.toml) &&
guard_config=$(cat ../example/guard-dev-config.toml) &&
export mysql_password="[YOUR_MYSQL_PASSWORD]" &&
export smtp_password="[YOUR_SMTP_PASSWORD]" &&
export elastic_user="[YOUR_ELASTIC_USERNAME]" &&
export elastic_password="[YOUR_ELASTIC_PASSWORD]"
```

## 2. Specify your environment variables in your [configuration](/example/coastguard-dev-config.toml)

```
[database.mysql]
username = "example-user"
password_env = "mysql_password"
hostname = "127.0.0.1"
port = 3306
database = "YOUR_SQL_DATABASE"
```

## 2. Run server
### Run server (via executable)
```
cargo run
```