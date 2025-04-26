use diesel::prelude::*;
use crate::tables::*;
use diesel::r2d2::{self, ConnectionManager};

use serde::{Serialize, Deserialize};
use serde_json::{Value, json};
use rocket::serde::json::Json;

type DbPool = r2d2::Pool<ConnectionManager<MysqlConnection>>;

// #[database("mysql_db")]
// struct DbConn(MysqlConnection);

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
    pub permission: Option<i64>,
    pub suspended: Option<bool>
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Crawler_index_body {
    pub actions: Option<Vec<Crawler_index_body_action>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Crawler_index_body_action {
    pub url: Option<String>,
    pub content: Option<Crawler_index_body_action_content>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Crawler_index_body_action_content {
    pub title: Option<String>,
    pub text: Option<String>,
    pub urls: Option<Vec<String>>,
    pub metatag: Option<Value>
}

#[derive(Clone, Debug, Deserialize)]
pub struct Admin_index_update_body {
    pub actions: Option<Vec<Admin_index_update_action>>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Admin_index_update_action {
    pub action: Option<String>,
    pub id: Option<String>,
    pub url: Option<Vec<String>>
}

#[derive(Clone, Debug, Deserialize)]
pub struct Discussion_update_body {
    pub actions: Option<Vec<Discussion_update_action>>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Discussion_update_action {
    pub action: Option<String>,
    pub id: Option<String>,
    pub metadata: Option<Value>,
    pub tags: Option<Vec<Value>>,
    pub content: Option<String>,
    pub attachments: Option<String>,
    pub nonce: Option<String>
}

#[derive(Clone, Debug, Deserialize)]
pub struct User_rating_update_body {
    pub actions: Option<Vec<User_rating_update_action>>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct User_rating_update_action {
    pub action: Option<String>,
    pub id: Option<String>,
    pub emoji: Option<String>
}

#[derive(Clone, Debug, Deserialize)]
pub struct Event_update_body {
    pub actions: Option<Vec<Event_update_action>>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Event_update_action {
    pub r#type: Option<String>,
    pub nonce: Option<String>,
    pub metadata: Option<Value>,
    pub alias: Option<String>,
    pub content: Option<String>,
    pub tags: Option<Vec<Value>>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Event_list_filter {
    pub r#type: Option<Vec<String>>,
    pub nonce: Option<Vec<String>>,
    pub nonce_hash: Option<Vec<String>>,
    pub created_before: Option<Vec<String>>,
    pub created_after: Option<Vec<String>>,
    pub user_rating: Option<bool>,
    pub distinct: Option<bool>
}

#[derive(Clone, Debug, Deserialize)]
pub struct Discussion_list_filter {
    pub nonce: Option<Vec<String>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User_rating_item {
    pub id: String,
    pub emoji: String,
    pub count: i64
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

#[derive(Clone, Debug, Deserialize)]
pub struct Request_authentication_output {
    // pub returned_connection: &MysqlConnection,
    // #[derive(Clone, Debug, Deserialize)]
    pub account_id: String,
    pub device_id: String,
    pub project_id: String
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config_sql {
    pub user: Option<String>,
    pub device: Option<String>,
    pub magiclink: Option<String>,
    pub network: Option<String>,
    pub process: Option<String>
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

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Insertable, Selectable, QueryableByName, Identifiable)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = accounts)]
pub struct Accounts {
    pub id: String,
    pub name: Option<String>,
    pub username: Option<String>,
    pub email: Option<String>,
    pub profile_pic: Option<String>,
    pub pronouns: Option<String>,
    pub created: Option<i64>,
    pub locked: Option<bool>,
    pub suspended: Option<bool>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Accounts_admin {
    pub id: String,
    pub name: Option<String>,
    pub username: Option<String>,
    pub profile_pic: Option<String>,
    pub pronouns: Vec<String>,
    pub created: Option<i64>,
    pub email: Option<String>
}

impl From<Accounts> for Accounts_admin {
    fn from(account: Accounts) -> Self {
        let pronouns: Vec<String> = match account.pronouns {
            Some(id_string) => {
                // Split by commas if it's a list, or use as a single item
                id_string.split('/').map(|s| s.trim().to_string()).collect()
            }
            None => Vec::new(), // No `id` provided
        };

        Accounts_admin {
            id: account.id,
            name: account.name,
            username: account.username,
            profile_pic: Some(account.profile_pic.unwrap_or("/default-pfp.png".to_string())),
            pronouns: pronouns,
            created: account.created,
            email: account.email
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Accounts_me {
    pub id: String,
    pub name: Option<String>,
    pub username: Option<String>,
    pub profile_pic: Option<String>,
    pub pronouns: Vec<String>,
    pub created: Option<i64>,
    pub email: Option<String>,
}

impl From<Accounts> for Accounts_me {
    fn from(account: Accounts) -> Self {
        let pronouns: Vec<String> = match account.pronouns {
            Some(id_string) => {
                // Split by commas if it's a list, or use as a single item
                id_string.split('/').map(|s| s.trim().to_string()).collect()
            }
            None => Vec::new(), // No `id` provided
        };

        Accounts_me {
            id: account.id,
            name: account.name,
            username: account.username,
            profile_pic: Some(account.profile_pic.unwrap_or("/default-pfp.png".to_string())),
            pronouns: pronouns,
            created: account.created,
            email: account.email
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Insertable, Selectable, QueryableByName)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = discussions)]
pub struct Discussions {
    pub message_id: String,
    pub discussion: String,
    pub author: String,
    pub content: Option<String>,
    pub attachments: Option<String>,
    pub created: Option<i64>,
    pub project: String,
    pub nonce: Option<String>
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Discussions_public {
    pub message_id: String,
    pub discussion: String,
    pub author: Accounts_admin,
    pub content: Option<String>,
    pub attachments: Option<String>,
    pub created: Option<i64>,
    pub nonce: Option<String>
}

impl From<(Discussions, Accounts)> for Discussions_public {
    fn from((discussions, author): (Discussions, Accounts)) -> Self {
        Discussions_public {
            message_id: discussions.message_id,
            discussion: discussions.discussion,
            author: author.into(),
            content: discussions.content,
            attachments: discussions.attachments,
            created: discussions.created,
            nonce: discussions.nonce,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Insertable, Selectable, QueryableByName)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = user_rating)]
pub struct User_rating {
    pub id: String,
    pub emoji: String,
    pub author: String,
    pub project: String,
    pub created: Option<i64>
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct User_rating_public {
    pub id: String,
    pub emoji: String,
    pub author: Accounts_admin,
    pub project: String,
    pub created: Option<i64>
}

impl From<(User_rating, Accounts)> for User_rating_public {
    fn from((user_rating, author): (User_rating, Accounts)) -> Self {
        User_rating_public {
            id: user_rating.id,
            emoji: user_rating.emoji,
            author: author.into(),
            project: user_rating.project,
            created: user_rating.created
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Insertable, Selectable, QueryableByName, Identifiable)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = issue)]
pub struct Issue {
    pub id: String,
    pub title: Option<String>,
    pub created: Option<i64>,
    pub project: String,
    pub discussion: Option<String>
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Issue_public {
    pub id: String,
    pub title: Option<String>,
    pub created: Option<i64>,
    pub project: Project_public,
    pub discussion: Option<String>
}

impl From<(Issue, Project, Namespace, Org)> for Issue_public {
    fn from((issue, project, namespace, org): (Issue, Project, Namespace, Org)) -> Self {
        Issue_public {
            id: issue.id,
            title: issue.title,
            created: issue.created,
            project: (project, namespace, org).into(),
            discussion: issue.discussion,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Insertable, Selectable, QueryableByName, Identifiable)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = bug)]
pub struct Bug {
    pub id: String,
    pub author: String,
    pub title: Option<String>,
    pub created: Option<i64>
}

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Insertable, Selectable, QueryableByName, Identifiable)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = project)]
pub struct Project {
    pub id: String,
    pub namespace: String,
    pub name: Option<String>,
    pub icon: Option<String>,
    pub created: Option<i64>
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Project_public {
    pub id: String,
    pub namespace: Namespace_public,
    pub name: Option<String>,
    pub icon: Option<String>,
    pub created: Option<i64>
}

impl From<(Project, Namespace, Org)> for Project_public {
    fn from((project, namespace, org): (Project, Namespace, Org)) -> Self {
        Project_public {
            id: project.id,
            namespace: (namespace, org).into(),
            name: project.name,
            icon: Some(project.icon.unwrap_or("/default-pfp.png".to_string())),
            created: project.created,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Insertable, Selectable, QueryableByName, Identifiable)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = namespaces)]
pub struct Namespace {
    pub id: String,
    pub org: String,
    pub name: Option<String>,
    pub icon: Option<String>,
    pub created: Option<i64>
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Namespace_public {
    pub id: String,
    pub org: Org_public,
    pub name: Option<String>,
    pub icon: Option<String>,
    pub created: Option<i64>
}

impl From<(Namespace, Org)> for Namespace_public {
    fn from((namespace, org): (Namespace, Org)) -> Self {
        Namespace_public {
            id: namespace.id,
            org: org.into(),
            name: namespace.name,
            icon: Some(namespace.icon.unwrap_or("/default-pfp.png".to_string())),
            created: namespace.created,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Insertable, Selectable, QueryableByName, Identifiable)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = orgs)]
pub struct Org {
    pub id: String,
    pub name: Option<String>,
    pub icon: Option<String>,
    pub created: Option<i64>
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Org_public {
    pub id: String,
    pub name: Option<String>,
    pub icon: Option<String>,
    pub created: Option<i64>
}

impl From<Org> for Org_public {
    fn from((org): (Org)) -> Self {
        Org_public {
            id: org.id,
            name: org.name,
            icon: Some(org.icon.unwrap_or("/default-pfp.png".to_string())),
            created: org.created,
        }
    }
}

// #[derive(Debug, Clone, Deserialize, Serialize)]
// pub struct Rover_users_data_for_admins {
//     pub id: String,
//     pub first_name: Option<String>,
//     pub last_name: Option<String>,
//     pub email: Option<String>,
//     pub permission: Option<i64>,
//     pub suspended: Option<bool>
// }

// #[derive(Debug, Clone, Deserialize, Serialize)]
// pub struct Rover_network_data_for_admins {
//     pub device: Option<Rover_devices_data_for_admins>,
//     pub user: Option<Rover_users_data_for_admins>,
//     pub domain: String,
//     pub ip_address: String,
//     pub destination_country: String,
//     pub destination_registrant: String,
//     pub protocol: String,
//     pub size: Option<i64>,
//     pub info: String,
//     pub created: Option<i64>,
// }

// impl From<(Rover_network, Option<Rover_devices>, Option<Rover_users>)> for Rover_network_data_for_admins {
//     fn from((network, device, user): (Rover_network, Option<Rover_devices>, Option<Rover_users>)) -> Self {
//         Rover_network_data_for_admins {
//             device: device.map(|d| {
//                 Rover_devices_data_for_admins::from((d, user.clone()))
//             }),
//             user: user.map(|d| d.into()),
//             domain: network.domain,
//             ip_address: network.ip_address,
//             destination_country: network.destination_country,
//             destination_registrant: network.destination_registrant,
//             protocol: network.protocol,
//             size: network.size,
//             info: network.info,
//             created: network.created
//         }
//     }
// }

// Websocket_event_process
#[derive(Debug, Clone, Deserialize)]
pub struct Websocket_event_process {
    pub processes: Vec<Websocket_event_process_item>
}

// Websocket_event_process
#[derive(Debug, Clone, Deserialize)]
pub struct Websocket_event_process_item {
    pub pid: Option<i64>,
    pub parent: Option<i64>,
    pub name: Option<String>,
    pub path: Option<String>,
    pub current_working_directory: Option<String>,
    pub status: Option<String>,
    pub run_time: Option<i64>,
    pub start_time: Option<i64>,
    pub hash: Option<String>,
    pub threads: Option<i64>,
    pub size: Option<i64>,
    pub pathname: Option<String>,
    pub created: Option<i64>
}

// #[derive(Debug, Clone, Deserialize, Serialize)]
// pub struct Rover_processes_data_for_admins {
//     pub device: Option<Rover_devices_data_for_admins>,
//     pub user: Option<Rover_users_data_for_admins>,
//     pub process: Option<String>,
//     pub last_seen: Option<i64>,
//     pub admin_user: Option<bool>,
//     pub is_admin_process: Option<bool>,
//     pub PID: Option<i64>,
//     pub publisher: Option<String>,
//     pub hash: Option<String>,
//     pub threads: Option<i64>,
//     pub size: Option<i64>,
//     pub pathname: Option<String>,
//     pub created: Option<i64>
// }

// impl From<(Rover_processes, Option<Rover_devices>, Option<Rover_users>)> for Rover_processes_data_for_admins {
//     fn from((process, device, user): (Rover_processes, Option<Rover_devices>, Option<Rover_users>)) -> Self {
//         Rover_processes_data_for_admins {
//             device: device.map(|d| {
//                 Rover_devices_data_for_admins::from((d, user.clone()))
//             }),
//             user: user.map(|d| d.into()),
//             process: process.process,
//             last_seen: process.last_seen,
//             admin_user: process.admin_user,
//             is_admin_process: process.is_admin_process,
//             PID: process.PID,
//             publisher: process.publisher,
//             hash: process.hash,
//             threads: process.threads,
//             size: process.size,
//             pathname: process.pathname,
//             created: process.created
//         }
//     }
// }

#[derive(Debug, Clone, Deserialize)]
pub struct Websocket_event_hades_websocket {
    pub event: Option<String>,
    pub jwt: Option<String>
}

#[derive(Debug, Clone, Deserialize)]
pub struct Websocket_event {
    pub body: Option<String>,
    pub _hades_websocket: Option<Websocket_event_hades_websocket>
}

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Insertable, Selectable, QueryableByName, Identifiable)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = device)]
pub struct Device {
    pub id: String,
    pub account_id: String,
    pub name: Option<String>,
    pub public_key: String,
    pub created: i64
}