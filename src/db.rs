use rocket::{Rocket, Build};
use rocket::fairing::AdHoc;
use rocket_db_pools::{sqlx, Database, Connection};
use sqlx::prelude::Row;
use crate::models;

#[derive(Database)]
#[database("order_db")]
pub struct Db(sqlx::SqlitePool);

type Result<T, E = sqlx::Error> = std::result::Result<Option<T>, E>;

pub async fn get_items(db: &mut Connection<Db>) -> Result<Vec<models::Item>> {
	let items = sqlx::query(r#"
		SELECT id, preparation_time, price_yen, name 
		FROM Items
		"#)
		.map(|record: sqlx::sqlite::SqliteRow| {
			models::Item {
				id: record.get_unchecked(0),
				preparation_time: record.get_unchecked(1),
				price_yen: record.get_unchecked(2),
				name: record.get_unchecked(3)
			}
		})
		.fetch_all(&mut **db)
		.await?;
	Ok(Some(items))
}

pub async fn get_item(db: &mut Connection<Db>, item_id: i64) -> Result<models::Item> {
	let query_result = sqlx::query(r#"
		SELECT id, preparation_time, price_yen, name 
		FROM Items 
		WHERE id = ?
		"#)
		.bind(item_id)
		.map(|record: sqlx::sqlite::SqliteRow| {
			models::Item {
				id: record.get_unchecked(0),
				preparation_time: record.get_unchecked(1),
				price_yen: record.get_unchecked(2),
				name: record.get_unchecked(3)
			}
		})
		.fetch_optional(&mut **db)
		.await?;
	match query_result {
		Some(item) => Ok(Some(item)),
		None => Ok(None),
	}
}

pub async fn create_order(db: &mut Connection<Db>, active_session_id: i64, order: &models::Order) -> Result<models::Order> {
	let result = sqlx::query("INSERT INTO Orders ('table_session_id') VALUES (?)")
		.bind(active_session_id)
		.execute(&mut **db)
		.await?;
	let order_id = result.last_insert_rowid();
	for item in &order.order_items {
		sqlx::query("INSERT INTO OrderItems ('item_id', 'order_id', 'amount') VALUES (?, ?, ?)")
		.bind(item.item_id)
		.bind(order_id)
		.bind(item.amount)
		.execute(&mut **db)
		.await?;
	}
	let order = retrieve_order(db, active_session_id, order_id).await?;
	Ok(order)
}

pub async fn retrieve_orders(db: &mut Connection<Db>, table_session_id: i64) -> Result<Vec<models::Order>> {
	let mut orders = sqlx::query(r#"
		SELECT id, timestamp
		FROM Orders 
		WHERE table_session_id = ?
		"#)
		.bind(table_session_id)
		.map(|record: sqlx::sqlite::SqliteRow| {
			models::Order {
				id: record.get_unchecked(0),
				timestamp: record.get_unchecked(1),
				table_session_id,
				order_items: Default::default()
			}
		})
		.fetch_all(&mut **db)
		.await?;

	for mut order in &mut orders {
		let items = sqlx::query(r#"
			SELECT Items.id, amount 
			FROM Items, OrderItems
			WHERE Items.id = OrderItems.item_id 
			AND OrderItems.order_id = ?
			"#)
			.bind(order.id)
			.map(|record: sqlx::sqlite::SqliteRow| {
				models::OrderItem {
					item_id: record.get_unchecked(0),
					amount: record.get_unchecked(1)
				}
			})
			.fetch_all(&mut **db)
			.await?;
		order.order_items = items;
	}
	Ok(Some(orders))
}

pub async fn retrieve_order(db: &mut Connection<Db>, table_session_id: i64, order_id: i64) -> Result<models::Order> {
	let items = sqlx::query(r#"
		SELECT Items.id, amount 
		FROM Items, OrderItems
		WHERE Items.id = OrderItems.item_id 
		AND OrderItems.order_id = ?
		"#)
		.bind(order_id)
		.map(|record: sqlx::sqlite::SqliteRow| {
			models::OrderItem {
				item_id: record.get_unchecked(0),
				amount: record.get_unchecked(1)
			}
		})
		.fetch_all(&mut **db)
		.await?;

	let query_result = sqlx::query(r#"
		SELECT id, timestamp
		FROM Orders 
		WHERE table_session_id = ?
		AND id = ?
		"#)
		.bind(table_session_id)
		.bind(order_id)
		.map(|record: sqlx::sqlite::SqliteRow| {
			models::Order {
				id: record.get_unchecked(0),
				timestamp: record.get_unchecked(1),
				table_session_id,
				order_items: Default::default()
			}
		})
		.fetch_optional(&mut **db)
		.await?;
	
	match query_result {
		Some(mut order) => {
			order.order_items = items;
			Ok(Some(order))
		}
		None => Ok(None),
	}
}

pub async fn delete_order(db: &mut Connection<Db>, table_session_id: i64, order_id: i64) -> std::result::Result<bool, sqlx::Error> {
	sqlx::query("DELETE FROM OrderItems WHERE order_id = ?")
		.bind(order_id)
		.execute(&mut **db)
		.await?;
	let result = sqlx::query("DELETE FROM Orders WHERE id = ? and table_session_id = ?")
		.bind(order_id)
		.bind(table_session_id)
		.execute(&mut **db)
		.await?;
	Ok(result.rows_affected() == 1)
}

pub async fn create_session(db: &mut Connection<Db>, table_nr: u8, session: &models::TableSession) -> Result<models::TableSession> {
	let result = sqlx::query("INSERT INTO TableSessions ('table_nr', 'customers') VALUES (?, ?)")
		.bind(table_nr)
		.bind(session.customers)
		.execute(&mut **db)
		.await?;
	let table_session_id = result.last_insert_rowid();
	let query_result = sqlx::query(r#"
		SELECT id, table_nr, customers, session_start, session_end, active 
		FROM TableSessions
		WHERE id = ?
		"#)
		.bind(table_session_id)
		.map(|record: sqlx::sqlite::SqliteRow| {
			models::TableSession {
				id: record.get_unchecked(0),
				table_nr: record.get_unchecked(1),
				customers: record.get_unchecked(2),
				session_start: record.get_unchecked(3),
				session_end: record.get_unchecked(4),
				active: record.get_unchecked(5)
			}
		})
		.fetch_optional(&mut **db)
		.await?;
	
	match query_result {
		Some(session) => Ok(Some(session)),
		None => Ok(None),
	}
}

pub async fn retrieve_active_table_sessions(db: &mut Connection<Db>) -> Result<Vec<models::TableSession>> {
	let sessions = sqlx::query(r#"
		SELECT id, table_nr, customers, session_start, session_end, active 
		FROM TableSessions
		WHERE active = TRUE
		"#)
		.map(|record: sqlx::sqlite::SqliteRow| {
			models::TableSession {
				id: record.get_unchecked(0),
				table_nr: record.get_unchecked(1),
				customers: record.get_unchecked(2),
				session_start: record.get_unchecked(3),
				session_end: record.get_unchecked(4),
				active: record.get_unchecked(5)
			}
		})
		.fetch_all(&mut **db)
		.await?;
	Ok(Some(sessions))
}

pub async fn retrieve_table_sessions(db: &mut Connection<Db>, table_nr: u8) -> Result<Vec<models::TableSession>> {
	let sessions = sqlx::query(r#"
		SELECT id, table_nr, customers, session_start, session_end, active 
		FROM TableSessions
		WHERE table_nr = ?
		"#)
		.bind(table_nr)
		.map(|record: sqlx::sqlite::SqliteRow| {
			models::TableSession {
				id: record.get_unchecked(0),
				table_nr: record.get_unchecked(1),
				customers: record.get_unchecked(2),
				session_start: record.get_unchecked(3),
				session_end: record.get_unchecked(4),
				active: record.get_unchecked(5)
			}
		})
		.fetch_all(&mut **db)
		.await?;
	Ok(Some(sessions))
}

pub async fn retrieve_active_table_session(db: &mut Connection<Db>, table_nr: u8) -> Result<models::TableSession> {
	let query_result = sqlx::query(r#"
		SELECT id, table_nr, customers, session_start, session_end, active 
		FROM TableSessions
		WHERE table_nr = ?
		and active = TRUE
		"#)
		.bind(table_nr)
		.map(|record: sqlx::sqlite::SqliteRow| {
			models::TableSession {
				id: record.get_unchecked(0),
				table_nr: record.get_unchecked(1),
				customers: record.get_unchecked(2),
				session_start: record.get_unchecked(3),
				session_end: record.get_unchecked(4),
				active: record.get_unchecked(5)
			}
		})
		.fetch_optional(&mut **db)
		.await?;
	
	match query_result {
		Some(session) => Ok(Some(session)),
		None => Ok(None),
	}
}

pub async fn deactivate_table_session(db: &mut Connection<Db>, table_session_id: i64) -> std::result::Result<bool, sqlx::Error> {
	let result = sqlx::query(r#"
		UPDATE TableSessions 
		SET active = FALSE, session_end = Datetime('now') 
		WHERE id = ?
		"#)
		.bind(table_session_id)
		.execute(&mut **db)
		.await?;
	Ok(result.rows_affected() == 1)
}

pub async fn delete_item_from_order(db: &mut Connection<Db>, order_id: i64, item_id: i64) -> std::result::Result<bool, sqlx::Error> {
	let result = sqlx::query(r#"
		DELETE FROM OrderItems 
		WHERE order_id = ?
		AND item_id = ?
		"#)
		.bind(order_id)
		.bind(item_id)
		.execute(&mut **db)
		.await?;
	Ok(result.rows_affected() == 1)
}

async fn run_migrations(rocket: Rocket<Build>) -> rocket::fairing::Result {
    match Db::fetch(&rocket) {
        Some(db) => match sqlx::migrate!().run(&**db).await {
            Ok(_) => {
				Ok(rocket)
			},
            Err(e) => {
                rocket::error!("Failed to initialize SQLx database: {}", e);
                Err(rocket)
            }
        }
        None => Err(rocket),
    }
}

pub fn stage() -> AdHoc {
	AdHoc::on_ignite("Stage database", |rocket| async {
		rocket.attach(Db::init())
		.attach(AdHoc::try_on_ignite("Database migrations", run_migrations))
	})
}