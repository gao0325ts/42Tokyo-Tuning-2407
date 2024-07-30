-- このファイルに記述されたSQLコマンドが、マイグレーション時に実行されます。

CREATE INDEX car_value ON orders
CREATE INDEX status ON orders
CREATE INDEX order_time ON orders
CREATE INDEX node_id ON orders

CREATE INDEX area_id ON nodes
CREATE INDEX id ON nodes
