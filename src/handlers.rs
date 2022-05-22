use rocket::fairing::AdHoc;
use rocket_db_pools::Connection;
use rocket::serde::json::Json;
use crate::db;
use crate::models;
use crate::error_handler::Error;

type Result<T, E = Error> = std::result::Result<Json<T>, E>;

#[rocket::get("/items/<item_id>")]
pub async fn get_item(mut db: Connection<db::Db>, item_id: i64) -> Result<models::Item> {
	let db_result = db::get_item(&mut db, item_id).await?;
	match db_result {
		Some(item) => Ok(Json(item)),
		None => Err(Error::Api{
			msg: format!("Unable to get item with ID {}", item_id)
		})
	}
}

#[rocket::get("/items")]
pub async fn get_items(mut db: Connection<db::Db>) -> Result<Vec<models::Item>> {
	let db_result = db::get_items(&mut db).await?;
	match db_result {
		Some(items) => Ok(Json(items)),
		None => Err(Error::Api{
			msg: String::from("Unable to get items")
		})
	}
}

#[rocket::post("/tables/<table_nr>/orders", data = "<order>")]
pub async fn new_order(mut db: Connection<db::Db>, table_nr: u8, order: Json<models::Order>) -> Result<models::Order> {
	let db_result = db::retrieve_active_table_session(&mut db, table_nr).await?;
	match db_result {
		Some(active_session) => {
			let db_result = db::create_order(&mut db, active_session.id, &order).await?;
			match db_result {
				Some(new_order) => Ok(Json(new_order)),
				None => Err(Error::Api{
					msg: String::from("Unable to create order")
				})
			}
		},
		None => Err(Error::Api{
			msg: format!("No active session for table #{}", table_nr)
		})
	}
}

#[rocket::get("/tables/<table_nr>/orders")]
pub async fn get_orders(mut db: Connection<db::Db>, table_nr: u8) -> Result<Vec<models::Order>> {
	let db_result = db::retrieve_active_table_session(&mut db, table_nr).await?;
	match db_result {
		Some(active_session) => {
			let db_result = db::retrieve_orders(&mut db, active_session.id).await?;
			match db_result {
				Some(orders) => Ok(Json(orders)),
				None => Err(Error::Api{
					msg: String::from("Unable to get orders")
				})
			}
		},
		None => Err(Error::Api{
			msg: format!("No active session for table #{}", table_nr)
		})
	}
}

#[rocket::get("/tables/<table_nr>/orders/<order_id>")]
pub async fn get_order(mut db: Connection<db::Db>, table_nr: u8, order_id: i64) -> Result<models::Order> {
	let db_result = db::retrieve_active_table_session(&mut db, table_nr).await?;
	match db_result {
		Some(active_session) => {
			let db_result = db::retrieve_order(&mut db, active_session.id, order_id).await?;
			match db_result {
				Some(order) => Ok(Json(order)),
				None => Err(Error::Api{
					msg: format!("Unable to get order with ID {}", order_id)
				})
			}
		},
		None => Err(Error::Api{
			msg: format!("No active session for table #{}", table_nr)
		})
	}
}

#[rocket::delete("/tables/<table_nr>/orders/<order_id>")]
pub async fn remove_order(mut db: Connection<db::Db>, table_nr: u8, order_id: i64) -> Result<String> {
	let db_result = db::retrieve_active_table_session(&mut db, table_nr).await?;
	match db_result {
		Some(active_session) => {
			let db_result = db::delete_order(&mut db, active_session.id, order_id).await?;
			if db_result {
				Ok(Json(String::from("success")))
			}
			else {
				Ok(Json(String::from("failed")))
			}
		},
		None => Err(Error::Api{
			msg: format!("No active session for table #{}", table_nr)
		})
	}
	
}

#[rocket::post("/tables/<table_nr>", data = "<session>")]
pub async fn new_session(mut db: Connection<db::Db>, table_nr: u8, session: Json<models::TableSession>) -> Result<models::TableSession> {
	let db_result = db::retrieve_active_table_session(&mut db, table_nr).await?;
	match db_result {
		Some(_) => Err(Error::Api{
			msg: format!("Active session for table #{} already exists", table_nr)
		}),
		None => {
			let db_result = db::create_session(&mut db, table_nr, &session).await?;
			match db_result {
				Some(session) => Ok(Json(session)),
				None => Err(Error::Api{
					msg: format!("Unable to create session for table #{}", table_nr)
				})
			}
		}
	}
}

#[rocket::get("/tables")]
pub async fn get_active_sessions(mut db: Connection<db::Db>) -> Result<Vec<models::TableSession>> {
	let db_result = db::retrieve_active_table_sessions(&mut db).await?;
	match db_result {
		Some(sessions) => Ok(Json(sessions)),
		None => Err(Error::Api{
			msg: String::from("Unable to get table sessions")
		})
	}
}

#[rocket::get("/tables/<table_nr>")]
pub async fn get_sessions(mut db: Connection<db::Db>, table_nr: u8) -> Result<Vec<models::TableSession>> {
	let db_result = db::retrieve_table_sessions(&mut db, table_nr).await?;
	match db_result {
		Some(sessions) => Ok(Json(sessions)),
		None => Err(Error::Api{
			msg: format!("Unable to get session for table #{}", table_nr)
		})
	}
}

#[rocket::get("/tables/<table_nr>/active")]
pub async fn get_active_session(mut db: Connection<db::Db>, table_nr: u8) -> Result<models::TableSession> {
	let db_result = db::retrieve_active_table_session(&mut db, table_nr).await?;
	match db_result {
		Some(active_session) => Ok(Json(active_session)),
		None => Err(Error::Api{
			msg: format!("No active session for table #{}", table_nr)
		})
	}
}

#[rocket::delete("/tables/<table_nr>")]
pub async fn end_session(mut db: Connection<db::Db>, table_nr: u8) -> Result<String> {
	let db_result = db::retrieve_active_table_session(&mut db, table_nr).await?;
	match db_result {
		Some(active_session) => {
			let db_result = db::deactivate_table_session(&mut db, active_session.id).await?;
			if db_result {
				Ok(Json(String::from("success")))
			}
			else {
				Ok(Json(String::from("failed")))
			}
		},
		None => Err(Error::Api{
			msg: format!("No active session for table #{}", table_nr)
		})
	}
}

#[rocket::delete("/tables/<table_nr>/orders/<order_id>/<item_id>")]
pub async fn remove_item(mut db: Connection<db::Db>, table_nr: u8, order_id: i64, item_id: i64) -> Result<String> {
	let db_result = db::retrieve_active_table_session(&mut db, table_nr).await?;
	match db_result {
		Some(active_session) => {
			let db_result = db::delete_item_from_order(&mut db, active_session.id, order_id, item_id).await?;
			if db_result {
				Ok(Json(String::from("success")))
			}
			else {
				Ok(Json(String::from("failed")))
			}
		},
		None => Err(Error::Api{
			msg: format!("No active session for table #{}", table_nr)
		})
	}
}

pub fn stage() -> AdHoc {
	AdHoc::on_ignite("Register handlers", |rocket| async {
		rocket.mount("/", rocket::routes![
			get_items,
			get_item,
			new_order,
			get_orders,
			get_order,
			remove_order,
			new_session,
			get_active_sessions,
			get_sessions,
			get_active_session,
			end_session,
			remove_item
		])
	})
}