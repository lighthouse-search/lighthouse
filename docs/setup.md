# Prerequisites
- A search backend — either [Elasticsearch](https://www.elastic.co/downloads/elasticsearch) or [OpenSearch](https://opensearch.org/downloads.html). Lighthouse speaks to both via the same REST API.
- [MariaDB](https://mariadb.com)

### Prerequisites via Docker

Here are some Docker commands that can quickly set up prerequisites for you. (**Please change ``MARIADB_ROOT_PASSWORD``!!**)
#### [Elasticsearch](https://hub.docker.com/_/elasticsearch)
```
docker run -d --name elasticsearch --net somenetwork -p 9200:9200 -p 9300:9300 -e "discovery.type=single-node" elasticsearch:tag
```

#### [OpenSearch](https://hub.docker.com/r/opensearchproject/opensearch)
```
docker run -d --name opensearch --net somenetwork -p 9200:9200 -p 9600:9600 -e "discovery.type=single-node" -e "OPENSEARCH_INITIAL_ADMIN_PASSWORD=YOUR_STRONG_PASSWORD" opensearchproject/opensearch:2
```

#### [Mariadb](https://hub.docker.com/_/mariadb)
```
docker run --detach --name some-mariadb --env MARIADB_ROOT_PASSWORD=my-secret-pw --env MARIADB_USER=myuser --env MARIADB_USER_PASSWORD mariadb:latest
```

# Setup

## 1. Set environment variables
These environment variables will be referenced in your configuration. Sensitive information (like tokens) is **never** stored in configurations for security.

`search_backend` is optional and defaults to `elasticsearch`; set it to `opensearch` if you're pointing at an OpenSearch cluster. The legacy `elastic_*` variable names are still honored as a fallback for `search_*`.
```
export lighthouse_config=$(cat ../example/coastguard-dev-config.toml) &&
guard_config=$(cat ../example/guard-dev-config.toml) &&
export mysql_password="[YOUR_MYSQL_PASSWORD]" &&
export smtp_password="[YOUR_SMTP_PASSWORD]" &&
export search_backend="elasticsearch" &&  # or "opensearch"
export search_host="https://127.0.0.1:9200" &&
export search_username="[YOUR_SEARCH_USERNAME]" &&
export search_password="[YOUR_SEARCH_PASSWORD]"
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