-- Add migration script here
CREATE TABLE TableSessions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    table_nr INTEGER NOT NULL,
    customers INTEGER NOT NULL,
    session_start TEXT DEFAULT (DateTime('now')) NOT NULL,
    session_end TEXT,
    active INTEGER DEFAULT (TRUE) NOT NULL
);

CREATE TABLE Items (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    preparation_time INTEGER NOT NULL,
    price_yen INTEGER NOT NULL,
    name TEXT NOT NULL
);

CREATE TABLE Orders (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    table_session_id INTEGER NOT NULL,
    timestamp TEXT DEFAULT (DateTime('now')) NOT NULL,
    FOREIGN KEY(table_session_id) REFERENCES TableSessions(id)
);

CREATE TABLE OrderItems (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    item_id INTEGER NOT NULL,
    order_id INTEGER NOT NULL,
    amount INTEGER NOT NULL,
    FOREIGN KEY(item_id) REFERENCES Items(id),
    FOREIGN KEY(order_id) REFERENCES Orders(id)
);