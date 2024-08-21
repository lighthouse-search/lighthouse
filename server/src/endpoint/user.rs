// use rocket::serde::{Serialize, Deserialize, json::Json};
// use rocket::serde::json::{Value, json};
// use rocket::response::{status, status::Custom};
// use rocket::http::Status;

// use rocket_db_pools::{Database, Connection};
// use rocket_db_pools::diesel::{MysqlPool, prelude::*};

// use crate::global::{ request_authentication, generate_random_id };
// use crate::responses::*;
// use crate::structs::*;
// use crate::tables::*;

// #[get("/list")]
// pub async fn user_list(mut db: Connection<Db>, params: &Query_string) -> Custom<Value> {
//     let request_authentication_output: Request_authentication_output = match request_authentication(db, None, params, "/user/list", false).await {
//         Ok(data) => data,
//         Err(e) => return status::Custom(Status::Unauthorized, not_authorized())
//     };
//     db = request_authentication_output.returned_connection;

//     let results = crate::tables::user::table
//         .select(Mindmap_users::as_select())
//         .load(&mut db)
//         .await.expect("Query failed");

//     let mut user_results_public: Vec<Mindmap_user_data_for_admins> = results
//         .into_iter()
//         .map(Mindmap_user_data_for_admins::from)
//         .collect();

//     status::Custom(Status::Ok, json!({
//         "ok": true,
//         "data": user_results_public
//     }))
// }

// #[post("/update", format = "application/json", data = "<body>")]
// pub async fn user_update(mut db: Connection<Db>, params: &Query_string, mut body: Json<User_update_body>) -> Custom<Value> {
//     let request_authentication_output: Request_authentication_output = match request_authentication(db, None, params, "/user/update", false).await {
//         Ok(data) => data,
//         Err(e) => return status::Custom(Status::Unauthorized, not_authorized())
//     };
//     db = request_authentication_output.returned_connection;

//     // block more than 13 characters for admin_permission_flags.

//     // TODO: There should be an action logic pipeline.
    
//     // Normallly it would matter what the value of unwrap_or was here, since we're trying to check the original value, in this case checking if it's None, but it doesn't matter here because there is a check for 'create' or 'update'.
//     let action = body.action.clone().unwrap_or(String::new());
//     if (action != "create" && action != "update") {
//         return status::Custom(Status::BadRequest, error_message("body.action must be create/update."));
//     }
//     if (action == "update" && body.id.is_none() == true) {
//         return status::Custom(Status::BadRequest, error_message("body.id must be specified when body.action='update'"));
//     }
//     if (action == "create" && body.id.is_none() == false) {
//         return status::Custom(Status::BadRequest, error_message("body.id cannot be specified when body.action='create'"));
//     }

//     if (body.first_name.is_none() == true) {
//         return status::Custom(Status::BadRequest, error_message("body.first_name is null or whitespace."));
//     }
//     if (body.last_name.is_none() == true) {
//         return status::Custom(Status::BadRequest, error_message("body.last_name is null or whitespace."));
//     }
//     if (body.email.is_none() == true) {
//         return status::Custom(Status::BadRequest, error_message("body.email is null or whitespace."));
//     }
//     if (body.permission.is_none() == true) {
//         return status::Custom(Status::BadRequest, error_message("body.permission is null or whitespace."));
//     }

//     let first_name = body.first_name.clone().expect("missing body.first_name");
//     let last_name = body.last_name.clone().expect("missing body.last_name");
//     let email = body.email.clone().expect("missing body.email");
//     let permission = body.permission.clone().expect("missing body.permission");

//     let mut user_id = generate_random_id();
//     // let number: i32 = rand::thread_rng().gen_range(0..999999);

//     if (action == "update") {
//         // We know 'body.id' exists, because we checked when validating the 'body.action'.
//         user_id = body.id.clone().unwrap(); 

//         // TODO: this should be a function, such as user_get().
//         let result: Option<Mindmap_users> = crate::tables::user::table
//         .filter(crate::tables::user::id.eq(user_id.clone()))
//         .first(&mut db)
//         .await
//         .optional().expect("Something went wrong querying the DB.");

//         if (result.is_none() == true) {
//             return status::Custom(Status::BadRequest, error_message(&format!("No user exists with the provided body.id: '{}'", user_id.clone())));
//         }

//         diesel::update(crate::tables::user::table.filter(crate::tables::user::id.eq(user_id.clone())))
//         .set((crate::tables::user::first_name.eq(first_name), crate::tables::user::last_name.eq(last_name), crate::tables::user::email.eq(email), crate::tables::user::permission.eq(permission)))
//         .execute(&mut db).await.expect("Failed to update");
//     } else if (action == "create") {
//         // Check if a user with the provided email already exists, we do not want duplicate emails.

//         // TODO: this should be a function, such as user_get().
//         let result: Option<Mindmap_users> = crate::tables::user::table
//         .filter(crate::tables::user::email.eq(email.clone()))
//         .first(&mut db)
//         .await
//         .optional().expect("Something went wrong querying the DB.");

//         if (result.is_none() == false) {
//             return status::Custom(Status::BadRequest, error_message(&format!("'{}' is already a user. Please use a different email address.", email)));
//         }

//         let user_insert = Mindmap_users {
//             id: user_id.clone(),
//             first_name: Some(first_name.clone()),
//             last_name: Some(last_name.clone()),
//             email: Some(email.clone()),
//             permission: Some(permission)
//         };
//         diesel::insert_into(crate::tables::user::table)
//         .values(&user_insert)
//         .execute(&mut db)
//         .await.expect("fail");
//     }

//     return status::Custom(Status::Ok, json!({
//         "ok": true,
//         "user_id": user_id.clone()
//     }));
// }