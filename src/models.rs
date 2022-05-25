use rocket::serde::{Serialize, Deserialize};
use serde_with::skip_serializing_none;

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(crate="rocket::serde")]
pub struct Item {
    pub id: i64,
    pub preparation_time: u32,
    pub price_yen: u32,
    pub name: String,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(crate="rocket::serde")]
pub struct TableSession {
    #[serde(skip_deserializing)]
    pub id: i64,
    #[serde(skip_deserializing)]
    pub table_nr: u8,
    pub customers: u8,
    #[serde(skip_deserializing)]
    pub session_start: String,
    #[serde(skip_deserializing)]
    pub session_end: String,
    #[serde(skip_deserializing)]
    pub active: bool
}

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(crate="rocket::serde")]
pub struct Order {
    pub id: Option<i64>,
    #[serde(skip_deserializing)]
    pub table_session_id: i64,
    #[serde(skip_deserializing)]
    pub timestamp: String,
    pub order_items: Vec<OrderItem>
}

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(crate="rocket::serde")]
pub struct OrderItem {
    pub item_id: i64,
    pub amount: u8
}