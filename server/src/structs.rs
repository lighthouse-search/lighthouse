use diesel::sql_types::BigInt;
use rocket::serde::{Serialize, Deserialize};
use crate::tables::*;
use rocket_db_pools::{Database, Connection};
use rocket_db_pools::diesel::{MysqlPool, prelude::*};

#[derive(Database)]
#[database("diesel_mysql")]
pub struct Db(MysqlPool);

// Incoming body structs
#[derive(Clone, Debug, Deserialize)]
pub struct Login_body {
    pub email: String
}

#[derive(Clone, Debug, Deserialize)]
pub struct User_update_body {
    pub action: Option<String>,
    pub id: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub permission: Option<i64>
}

#[derive(Clone, Debug, Deserialize)]
pub struct Folder_update_body {
    pub action: Option<String>,
    pub id: Option<String>,
    pub title: Option<String>,
    pub visibility: Option<String>,
    pub inner_folder: Option<String>
}

#[derive(Clone, Debug, Deserialize)]
pub struct Item_update_body {
    pub action: Option<String>,
    pub id: Option<String>,
    pub title: Option<String>,
    pub visibility: Option<String>,
    pub folder: Option<String>
}

#[derive(Clone, Debug, Deserialize)]
pub struct Keyword_update_body {
    pub actions: Option<Vec<Keyword_metadata_action>>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Keyword_metadata_action {
    pub action: Option<String>,
    pub id: Option<String>,
    pub description: Option<String>,
    pub image: Option<String>,
    pub external_image: Option<String>,
    pub external_link: Option<String>,
    pub keywords: Option<Vec<Keyword_action>>
}

#[derive(Clone, Debug, Deserialize)]
pub struct Keyword_action {
    pub action: Option<String>,
    pub word: Option<String>
}

#[derive(Clone, Debug, Deserialize)]
pub struct Item_content_update_body {
    pub actions: Option<Vec<Item_content_action>>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Item_content_action {
    pub action: Option<String>,
    pub item: Option<String>,
    pub row_id: Option<String>,
    pub parent: Option<String>,
    pub rank: Option<i64>,
    pub content: Option<String>,
}

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct Item_content_Authed_item_cache_check {
    pub item_id: String,
    pub user_id: String
}

#[derive(Clone, Debug, Deserialize)]
pub struct Authenticate_Body {
    pub attempt_id: String,
    pub code: Option<i64>,
    pub public_key: String
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct System_users {
    pub username: String,
    pub is_admin: bool,
    pub permissions: String
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Device_startup_struct {
    pub os_type: String,
    pub os_version: Option<i64>,
    pub alias: Option<i64>,
    pub users: Vec<System_users>,
    pub rover_permissions: Vec<String>
}

// Internal structs
#[derive(Debug)]
pub struct Query_string(pub String);

pub struct Request_authentication(pub Option<Request_authentication_output>);

pub struct Request_authentication_output {
    pub returned_connection: Connection<Db>,
    // #[derive(Clone, Debug, Deserialize)]
    pub user_id: String,
    pub device_id: String
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config_sql {
    pub user: Option<String>,
    pub device: Option<String>,
    pub magiclink: Option<String>,
    pub folder: Option<String>,
    pub item: Option<String>,
    pub item_content: Option<String>,
    pub keyword: Option<String>,
    pub keyword_metadata: Option<String>
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config_database_mysql {
    pub username: Option<String>,
    pub password_env: Option<String>,
    pub hostname: Option<String>,
    pub port: Option<i64>,
    pub database: Option<String>
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config_smtp {
    pub host: Option<String>,
    pub port: Option<i64>,
    pub username: Option<String>,
    pub from_alias: Option<String>,
    pub from_header: Option<String>,
    pub reply_to_address: Option<String>,
    pub password_env: Option<String>
}

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Insertable, Selectable)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = posts)]
pub struct Post {
    #[serde(skip_deserializing)]
    id: Option<i64>,
    title: String,
    text: String,
    #[serde(skip_deserializing)]
    published: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Insertable, Selectable)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = user)]
pub struct Mindmap_users {
    #[serde(skip_deserializing)]
    pub id: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub permission: Option<i64>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Insertable, Selectable)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = device)]
pub struct Mindmap_devices {
    // #[serde(skip_deserializing)]
    pub id: String,
    pub user_id: String,
    pub public_key: String,
    pub created: Option<i64>,
    pub active: Option<bool>,
    pub alias: Option<String>
}

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Insertable, Selectable, QueryableByName, Identifiable)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = mindmap_folder)]
pub struct Mindmap_folder {
    #[serde(skip_deserializing)]
    pub id: String,
    pub title: Option<String>,
    pub owner: Option<String>,
    pub created: Option<i64>,
    pub visibility: Option<String>
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Mindmap_folder_public {
    pub id: String,
    pub title: Option<String>,
    pub owner: Option<String>,
    pub created: Option<i64>,
    pub visibility: Option<String>
}

impl From<Mindmap_folder> for Mindmap_folder_public {
    fn from(mindmap_folder: Mindmap_folder) -> Self {
        Mindmap_folder_public {
            id: mindmap_folder.id,
            title: mindmap_folder.title,
            owner: mindmap_folder.owner,
            created: mindmap_folder.created,
            visibility: mindmap_folder.visibility,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Insertable, Selectable, QueryableByName, Identifiable)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = mindmap_item)]
pub struct Mindmap_item {
    #[serde(skip_deserializing)]
    pub id: String,
    pub title: Option<String>,
    pub folder: Option<String>,
    pub owner: Option<String>,
    pub created: Option<i64>,
    pub visibility: Option<String>
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Mindmap_item_public {
    pub id: String,
    pub title: Option<String>,
    pub folder: Option<String>,
    pub owner: Option<String>,
    pub created: Option<i64>,
    pub visibility: Option<String>
}

impl From<Mindmap_item> for Mindmap_item_public {
    fn from(mindmap_item: Mindmap_item) -> Self {
        Mindmap_item_public {
            id: mindmap_item.id,
            title: mindmap_item.title,
            folder: mindmap_item.folder,
            owner: mindmap_item.owner,
            created: mindmap_item.created,
            visibility: mindmap_item.visibility,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Insertable, Selectable, QueryableByName)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = mindmap_item_content)]
pub struct Mindmap_item_content {
    #[serde(skip_deserializing)]
    pub row_id: String,
    pub parent: Option<String>,
    pub item: Option<String>,
    pub rank: Option<i64>,
    pub content: Option<String>,
    pub created: Option<i64>
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Mindmap_item_content_public {
    pub row_id: String,
    pub parent: Option<String>,
    pub item: Option<String>,
    pub rank: Option<i64>,
    pub content: Option<String>,
    pub created: Option<i64>
}

impl From<Mindmap_item_content> for Mindmap_item_content_public {
    fn from(mindmap_item: Mindmap_item_content) -> Self {
        Mindmap_item_content_public {
            row_id: mindmap_item.row_id,
            parent: mindmap_item.parent,
            item: mindmap_item.item,
            rank: mindmap_item.rank,
            content: mindmap_item.content,
            created: mindmap_item.created,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Insertable, Selectable, QueryableByName)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = keyword_sql)]
pub struct Keyword_sql {
    pub id: Option<String>,
    pub keywords: Option<String>,
    pub description: Option<String>,
    pub image: Option<String>,
    pub external_image: Option<String>,
    pub external_link: Option<String>,
    pub owner: Option<String>,
    pub created: Option<i64>
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Rendered_keyword {
    pub id: String,
    pub owner: Option<String>,
    pub description: Option<String>,
    pub external_link: Option<String>,
    pub image: Option<String>,
    pub external_image: Option<String>,
    pub item: Option<String>,
    pub keywords: Option<Vec<String>>,
    pub created: Option<i64>
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Rendered_keyword_public {
    pub id: String,
    pub description: Option<String>,
    pub external_link: Option<String>,
    pub image: Option<String>,
    pub external_image: Option<String>,
    pub keywords: Option<Vec<String>>,
    pub created: Option<i64>
}

impl From<Rendered_keyword> for Rendered_keyword_public {
    fn from(data: Rendered_keyword) -> Self {
        Rendered_keyword_public {
            id: data.id,
            description: data.description,
            external_link: data.external_link,
            image: data.image,
            external_image: data.external_image,
            keywords: data.keywords,
            created: data.created,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Insertable, Selectable, QueryableByName)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = keyword_metadata)]
pub struct Keyword_metadata {
    pub id: String,
    pub owner: Option<String>,
    pub description: Option<String>,
    pub external_link: Option<String>,
    pub image: Option<String>,
    pub external_image: Option<String>,
    pub item: Option<String>,
    pub created: Option<i64>
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Keyword_metadata_public {
    pub id: String,
    pub description: Option<String>,
    pub external_link: Option<String>,
    pub image: Option<String>,
    pub external_image: Option<String>,
    pub created: Option<i64>
}

impl From<Keyword_metadata> for Keyword_metadata_public {
    fn from(data: Keyword_metadata) -> Self {
        Keyword_metadata_public {
            id: data.id,
            description: data.description,
            external_link: data.external_link,
            image: data.image,
            external_image: data.external_image,
            created: data.created,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Insertable, Selectable, QueryableByName)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = keywords)]
pub struct Keyword {
    pub keyword: String,
    pub owner: Option<String>,
    pub item: Option<String>,
    pub created: Option<i64>,
    pub keyword_metadata: String
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Keyword_public {
    pub keyword: String,
    pub created: Option<i64>
}

impl From<Keyword> for Keyword_public {
    fn from(data: Keyword) -> Self {
        Keyword_public {
            keyword: data.keyword,
            created: data.created
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Insertable, Selectable)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = rover_processes)]
pub struct Rover_processes {
    #[serde(skip_deserializing)]
    pub device_id: String,
    pub process: String,
    pub last_seen: Option<i64>,
    pub user: String,
    pub admin_user: String,
    pub is_admin_process: String,
    pub PID: Option<i64>,
    pub publisher: String,
    pub hash: String,
    pub threads: Option<i64>,
    pub size: Option<i64>,
    pub pathname: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Insertable, Selectable)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = magiclink)]
pub struct Login_code_record {
    pub attempt_id: String,
    pub code: Option<i64>,
    pub created: Option<i64>,
    pub attempts: Option<i64>,
    pub user_id: String,
}