-- Таблица для структуры Delivery
CREATE TABLE deliveries (
                            id SERIAL PRIMARY KEY,
                            name VARCHAR(255) NOT NULL,
                            phone VARCHAR(20) NOT NULL,
                            zip VARCHAR(20) NOT NULL,
                            city VARCHAR(255) NOT NULL,
                            address TEXT NOT NULL,
                            region VARCHAR(255) NOT NULL,
                            email VARCHAR(255) NOT NULL
);

-- Таблица для структуры Payment
CREATE TABLE payments (
                          id SERIAL PRIMARY KEY,
                          transaction VARCHAR(255) NOT NULL,
                          request_id VARCHAR(255),
                          currency VARCHAR(10) NOT NULL,
                          provider VARCHAR(255) NOT NULL,
                          amount BIGINT NOT NULL,
                          payment_dt BIGINT NOT NULL,
                          bank VARCHAR(255) NOT NULL,
                          delivery_cost BIGINT NOT NULL,
                          goods_total INT NOT NULL,
                          custom_fee BIGINT NOT NULL
);

-- Создаем последовательность
CREATE SEQUENCE IF NOT EXISTS deliveries_id_seq START 1;

-- Таблица для структуры Order
CREATE TABLE orders (
                        order_uid varchar (255) PRIMARY KEY DEFAULT nextval('deliveries_id_seq'),
                        track_number VARCHAR(255) NOT NULL,
                        entry VARCHAR(255) NOT NULL,
                        delivery_id SERIAL REFERENCES deliveries(id),  -- Связь с таблицей deliveries
                        payment_id SERIAL REFERENCES payments(id),    -- Связь с таблицей payments
                        locale VARCHAR(50) NOT NULL,
                        internal_signature VARCHAR(255),
                        customer_id VARCHAR(255) NOT NULL,
                        delivery_service VARCHAR(255) NOT NULL,
                        shardkey VARCHAR(255) NOT NULL,
                        sm_id BIGINT NOT NULL,
                        date_created TIMESTAMP NOT NULL,
                        oof_shard VARCHAR(255) NOT NULL
);

ALTER SEQUENCE deliveries_id_seq OWNED BY deliveries.id;

-- Таблица для структуры Item
CREATE TABLE items (
                       id SERIAL PRIMARY KEY,
                       order_uid varchar(255) REFERENCES orders(order_uid) ON DELETE CASCADE,  -- Связь с таблицей orders
                       chrt_id BIGINT NOT NULL,
                       track_number VARCHAR(255) NOT NULL,
                       price BIGINT NOT NULL,
                       rid VARCHAR(255) NOT NULL,
                       name VARCHAR(255) NOT NULL,
                       sale INT NOT NULL,
                       size VARCHAR(50) NOT NULL,
                       total_price VARCHAR(255) NOT NULL,
                       nm_id BIGINT NOT NULL,
                       brand VARCHAR(255) NOT NULL,
                       status INT NOT NULL
);

CREATE INDEX idx_items_order_uid ON items (order_uid);