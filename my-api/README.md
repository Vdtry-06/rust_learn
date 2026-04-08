# My API - Ứng Dụng Quản Lý Người Dùng

Một API RESTful được xây dựng bằng **Rust** với Axum framework để quản lý dữ liệu người dùng.

## 📋 Tổng Quan Dự Án

`my-api` là một ứng dụng máy chủ hiệu suất cao, được xây dựng bằng Rust sử dụng các công nghệ hiện đại:

- **Web Framework**: Axum 0.7
- **Async Runtime**: Tokio với các tính năng đầy đủ
- **Database**: PostgreSQL 16
- **ORM/Query Builder**: SQLx
- **Serialization**: Serde JSON
- **Environment Management**: Dotenvy

## 🏗️ Cấu Trúc Dự Án

```
my-api/
├── src/
│   ├── main.rs              # Entry point của ứng dụng
│   ├── models/
│   │   └── user.rs          # Model User và các request structs
│   ├── handlers/
│   │   └── user.rs          # Handler cho các endpoint liên quan User
│   ├── db/
│   │   └── mod.rs           # Database query functions
│   ├── routes/
│   │   └── mod.rs           # Định nghĩa các route
│   ├── models.rs            # Export models module
│   ├── db.rs                # Export db module
│   ├── handlers.rs          # Export handlers module
│   └── routes.rs            # Export routes module
├── migrations/
│   └── 20260408152821_create_users_table.sql  # Migration tạo bảng users
├── docker-compose.yml       # Cấu hình Docker cho PostgreSQL
├── Cargo.toml               # Định nghĩa dependencies
└── .env                     # Environment variables (local)
```

## 🚀 Bắt Đầu Nhanh

### Yêu Cầu

- Rust 1.70+ (cài từ [rustup.rs](https://rustup.rs/))
- Docker & Docker Compose (hoặc PostgreSQL cài sẵn)

### 1. Clone/Setup dự án

```bash
cd my-api
```

### 2. Cấu hình Environment

Tạo file `.env` trong thư mục `my-api`:

```env
DATABASE_URL=postgres://admin:secret123@localhost:5434/mydb
```

### 3. Khởi động Database

```bash
docker-compose up -d
```

Điều này sẽ:

- Khởi động PostgreSQL container trên port `5434`
- Tạo database `mydb`
- Tài khoản mặc định: `admin` / `secret123`

### 4. Chạy Migrations

```bash
sqlx migrate run
```

### 5. Build & Chạy Ứng Dụng

```bash
cargo run
```

Server sẽ chạy tại: **http://localhost:3000**

## 📡 API Endpoints

### User Management

#### 1. Lấy tất cả người dùng

```http
GET /users
```

**Response** (200 OK):

```json
[
  {
    "id": 1,
    "name": "Nguyễn Văn A",
    "email": "nguyena@example.com",
    "age": 25
  }
]
```

#### 2. Tạo người dùng mới

```http
POST /users
Content-Type: application/json

{
  "name": "Trần Thị B",
  "email": "tranb@example.com",
  "age": 30
}
```

**Response** (201 Created):

```json
{
  "id": 2,
  "name": "Trần Thị B",
  "email": "tranb@example.com",
  "age": 30
}
```

#### 3. Lấy thông tin người dùng theo ID

```http
GET /users/{id}
```

**Response** (200 OK):

```json
{
  "id": 1,
  "name": "Nguyễn Văn A",
  "email": "nguyena@example.com",
  "age": 25
}
```

#### 4. Cập nhật người dùng

```http
PUT /users/{id}
Content-Type: application/json

{
  "email": "newemail@example.com",
  "age": 26
}
```

**Response** (200 OK):

```json
{
  "id": 1,
  "name": "Nguyễn Văn A",
  "email": "newemail@example.com",
  "age": 26
}
```

#### 5. Xóa người dùng

```http
DELETE /users/{id}
```

**Response** (204 No Content)

## 📊 Database Schema

### Bảng `users`

```sql
CREATE TABLE users (
    id    SERIAL PRIMARY KEY,
    name  TEXT NOT NULL,
    email TEXT NOT NULL UNIQUE,
    age   INTEGER NOT NULL
);
```

**Columns:**

- `id`: Khóa chính, tự động tăng
- `name`: Tên người dùng
- `email`: Email duy nhất
- `age`: Tuổi

## 🔧 Dependencies

| Package    | Version | Mục Đích              |
| ---------- | ------- | --------------------- |
| axum       | 0.7     | Web framework         |
| tokio      | 1.x     | Async runtime         |
| sqlx       | 0.7     | Database query        |
| serde      | 1.x     | Serialization         |
| serde_json | 1.x     | JSON support          |
| dotenvy    | 0.15    | Environment variables |

## 📝 Workflow Phát Triển

### Tạo Migration Mới

```bash
sqlx migrate add -r <tên_migration>
```

Chỉnh sửa file trong `migrations/` và chạy:

```bash
sqlx migrate run
```

### Build Release

```bash
cargo build --release
```

### Chạy Tests

```bash
cargo test
```

### Format Code

```bash
cargo fmt
```

### Lint Code

```bash
cargo clippy
```

## 🐛 Troubleshooting

### Lỗi: "Không thể kết nối đến database"

- Kiểm tra Docker Compose đã khởi động: `docker-compose ps`
- Kiểm tra DATABASE_URL trong `.env` có đúng không
- Chờ database startup hoàn tất (vài giây)

### Lỗi: "Migration thất bại"

- Xóa toàn bộ data: `docker-compose down -v`
- Khởi động lại: `docker-compose up -d`
- Chạy migration: `sqlx migrate run`

### Port 5434 đã được sử dụng

Sửa trong `docker-compose.yml`:

```yaml
ports:
  - "5435:5432" # Thay đổi port ngoài
```

Cập nhật `DATABASE_URL`:

```env
DATABASE_URL=postgres://admin:secret123@localhost:5435/mydb
```

## 📚 Tài Liệu Tham Khảo

- [Axum Documentation](https://docs.rs/axum/)
- [SQLx Documentation](https://github.com/launchbadge/sqlx)
- [Tokio Documentation](https://tokio.rs/)
- [Rust Book](https://doc.rust-lang.org/book/)

## 📄 License

MIT

---

**Được tạo**: April 2026 | **Phiên bản**: 0.1.0
