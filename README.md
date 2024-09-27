# Документация к проекту

## Описание
Проект представляет собой веб-сервис, реализованный на Rust с использованием фреймворка Axum. Он предоставляет API для работы с заказами, платежами и товарами, а также использует PostgreSQL для хранения данных и Redis для кэширования.

## Технологии
- **Axum**: фреймворк для создания веб-приложений на Rust.
- **Tokio**: асинхронный runtime для выполнения задач.
- **Tokio-Postgres**: асинхронный клиент для PostgreSQL.
- **Redis**: in-memory база данных для кэширования.
- **Clap**: библиотека для обработки командной строки.
- **Env-logger**: библиотека для логирования с использованием переменных окружения.

## Установка и запуск

### Требования
- Rust 1.56 и выше
- Docker (для запуска PostgreSQL и Redis)
- PostgreSQL и Redis должны быть настроены и доступны для подключения.

### Инструкции по установке

1. Склонируйте репозиторий:
   ```bash
   git clone <repository_url>
   cd <repository_name>
   ```
2. Установите зависимости:
   ```bash
   cargo build
   ```
3. Настройте PostgreSQL и Redis. Можно запустить через Docker:
    ```bash
        docker-compose up -d
    ```
4. Запустите проект:
   ```bash
   cargo run -- -p 8080
    ```

5. После запуска, веб-сервер будет доступен на указанном порту, например:

    ```
   http://127.0.0.1:8080
   ```


## Конфигурация

### Переменные окружения
- `RUST_LOG`: уровень логирования (например, `info`, `debug`). По умолчанию используется уровень `info`.

### Аргументы командной строки
- `-p` или `--port`: порт, на котором будет работать веб-сервер.

### Пример запуска:

```bash
  cargo run -- -p 8080
```

## Эндпоинты

### 1. Получение заказа по ID

**Метод:** `GET`  
**URL:** `/order/:order_id`

**Описание:**  
Возвращает информацию о заказе по указанному идентификатору `order_id`.

**Параметры URL:**
- `order_id` (string): ID заказа, который необходимо получить.

**Ответ:**
- **200 OK**: Успешно возвращает данные о заказе.
  ```json
  {
  "order_uid": "b563feb7b2b84b6test",
  "track_number": "WBILMTESTTRACK",
  "entry": "WBIL",
  "delivery": {
    "name": "Test Testov",
    "phone": "+9720000000",
    "zip": "2639809",
    "city": "Kiryat Mozkin",
    "address": "Ploshad Mira 15",
    "region": "Kraiot",
    "email": "test@gmail.com"
  },
  "payment": {
    "transaction": "b563feb7b2b84b6test",
    "request_id": "",
    "currency": "USD",
    "provider": "wbpay",
    "amount": 1817,
    "payment_dt": 1637907727,
    "bank": "alpha",
    "delivery_cost": 1500,
    "goods_total": 317,
    "custom_fee": 0
  },
  "items": [
    {
      "chrt_id": 9934930,
      "track_number": "WBILMTESTTRACK",
      "price": 453,
      "rid": "ab4219087a764ae0btest",
      "name": "Mascaras",
      "sale": 30,
      "size": "0",
      "total_price": 317,
      "nm_id": 2389212,
      "brand": "Vivienne Sabo",
      "status": 202
    }
  ],
  "locale": "en",
  "internal_signature": "",
  "customer_id": "test",
  "delivery_service": "meest",
  "shardkey": "9",
  "sm_id": 99,
  "date_created": "2021-11-26T06:22:19Z",
  "oof_shard": "1"
}
    ```

## 2. Получение товара по ID
**Метод:** `GET`  
**URL:** `/item/:chrt_id`

**Описание:**  
Возвращает информацию о заказе по указанному идентификатору `order_id`.

**Параметры URL:**
- `chrt_id` (string): уникальный идентификатор товара (число).

**Ответ:**
- **200 OK**: информация о товаре в формате JSON.
  ```json
  {
  "chrt_id": 9934930,
  "track_number": "WBILMTESTTRACK",
  "price": 453,
  "rid": "ab4219087a764ae0btest",
  "name": "Mascaras",
  "sale": 30,
  "size": "0",
  "total_price": "317",
  "nm_id": 2389212,
  "brand": "Vivienne Sabo",
  "status": 202
  } 
    ```
  ## 3. Получение информации о платеже по ID
**Метод:** `GET`  
**URL:** `/payment/:payment_id`

**Описание:**  
Возвращает информацию о заказе по указанному идентификатору `order_id`.

**Параметры URL:**
- `payment_id` (string): уникальный идентификатор платежа (строка).

**Ответ:**
- **200 OK**: информация о платеже в формате JSON.
  ```json
  {
  "transaction": "b563feb7b2b84b6test",
  "request_id": "",
  "currency": "USD",
  "provider": "wbpay",
  "amount": 1817,
  "payment_dt": 1637907727,
  "bank": "alpha",
  "delivery_cost": 1500,
  "goods_total": 317,
  "custom_fee": 0
  }
    ```