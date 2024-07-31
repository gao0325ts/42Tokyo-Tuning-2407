-- このファイルに記述されたSQLコマンドが、マイグレーション時に実行されます。

CREATE INDEX car_value ON orders
CREATE INDEX status ON orders
CREATE INDEX order_time ON orders
CREATE INDEX node_id ON orders

CREATE INDEX area_id ON nodes
CREATE INDEX id ON nodes

CREATE INDEX status ON tow_trucks
CREATE INDEX area_id ON tow_trucks
CREATE INDEX driver_id ON tow_trucks
CREATE INDEX id ON tow_trucks

CREATE INDEX id ON users

CREATE INDEX tow_truck_id ON locations
CREATE INDEX timestamp ON locations

