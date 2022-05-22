-- Add migration script here
INSERT INTO items ('preparation_time', 'name', 'price_yen') VALUES
	(328, 'Bresaola and sausage spaghetti', 2972),
	(1374, 'Spinach and squash risotto', 576),
	(749, 'Tea and banana bread', 1119),
	(1023, 'Bean and ginger cake', 2509),
	(1266, 'Crayfish and gruyere toastie', 774),
	(344, 'Strawberry and cider jam', 2315),
	(659, 'Sausage and pigeon wontons', 494),
	(939, 'Honey and prune cupcakes', 1762),
	(1496, 'Fish and cod crumble', 2893),
	(434, 'Blueberry and banana cupcakes', 2462);

INSERT INTO TableSessions ('table_nr', 'customers') VALUES
	(1, 3),
	(2, 5),
	(3, 1);

INSERT INTO Orders ('table_session_id') VALUES
	(1),
	(2),
	(3);

INSERT INTO OrderItems ('order_id', 'item_id', 'amount') VALUES
	(1,3,1),
	(1,3,2),
	(1,8,5),
	(1,5,3),
	(1,8,5),

	(2,4,9),
	(2,7,4),
	(2,7,5),
	(2,3,8),
	(2,1,6),

	(3,7,2);