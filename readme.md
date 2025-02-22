# 🚀 Ticket Manager API  
> A Rust-based issue and project tracking API built with Axum and Diesel ORM.

![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)
![Axum](https://img.shields.io/badge/Axum-000000?style=for-the-badge&logo=rust&logoColor=white)
![PostgreSQL](https://img.shields.io/badge/PostgreSQL-336791?style=for-the-badge&logo=postgresql&logoColor=white)
![Diesel](https://img.shields.io/badge/Diesel-000000?style=for-the-badge&logo=rust&logoColor=white)

A **lightweight project and issue tracking API** built in **Rust**, using **Axum** for the web framework and **Diesel ORM** for PostgreSQL database integration. This service provides **CRUD operations for managing projects and issues**, supports **real-time updates, detailed issue tracking, and secure data storage**, making it ideal for **team collaboration and agile development workflows**.

---

## 🌟 Features
✔ **Create, update, and delete projects**  
✔ **Manage issues within projects**  
✔ **Track project statuses**  
✔ **RESTful API design with structured JSON responses**  
✔ **High-performance Rust backend using Axum**  
✔ **PostgreSQL database with Diesel ORM**  

---

## 🛠 Tech Stack

| Technology | Purpose |
|------------|---------|
| **Rust** 🦀 | Systems programming language |
| **Axum** ⚡ | Web framework |
| **Diesel** 🛢️ | ORM for PostgreSQL |
| **Tokio** ⏳ | Async runtime |
| **Serde** 📦 | Serialization/Deserialization |

---

## 🚀 Getting Started

### **1️⃣ Install Prerequisites**
Install **Rust** using [Rustup](https://rustup.rs/):
```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Install **Diesel CLI**:
```sh
cargo install diesel_cli --no-default-features --features postgres
```

Ensure **Docker** is installed:  
🔗 [Docker Installation Guide](https://docs.docker.com/get-docker/)

---

## 🐳 **Database Setup with Docker**
To simplify database setup, use **Docker** to run PostgreSQL and pgAdmin.

### **1️⃣ Run PostgreSQL and pgAdmin**
```sh
docker-compose up -d
```
This will:
- Start a **PostgreSQL 16** database container.
- Start **pgAdmin 4** for managing the database.
- Persist data using Docker **volumes**.

### **2️⃣ PostgreSQL Connection Details**
| Key             | Value             |
|----------------|------------------|
| **Host**       | `localhost`       |
| **Port**       | `5432`            |
| **Database**   | `mydb`            |
| **Username**   | `admin`           |
| **Password**   | `adminpassword`   |

### **3️⃣ Access pgAdmin**
1. Open **pgAdmin**: `http://localhost:5050`
2. **Login Credentials**:
   - Email: `admin@admin.com`
   - Password: `admin`
3. Click **"Add New Server"**, enter:
   - **Host**: `postgres_db`
   - **Username**: `admin`
   - **Password**: `adminpassword`

---

## 🔄 **Running Diesel Migrations**
After setting up the database, apply migrations:

1. **Set up Diesel**
```sh
diesel setup
```

2. **Run Migrations**
```sh
diesel migration run
```

3. **Reset Database (Caution)**
```sh
diesel database reset
diesel migration run
```

---

## 🛠 **Installation**
1. Clone the repository:
```sh
git clone https://github.com/danielphilipjohnson/rust-ticket-manager.git
cd ticket-manager
```

2. Set up environment variables (`.env` file):
```sh
DATABASE_URL=postgres://admin:adminpassword@localhost/mydb
```

3. Build the project:
```sh
cargo build
```

4. Run the tests:
```sh
cargo test
```

5. Start the server:
```sh
cargo run
```
The server will start on `http://localhost:3000`.

---

## 📡 **API Endpoints**

### **1️⃣ Projects**
#### **Create a Project (`POST /api/projects`)**
```sh
curl -X POST http://localhost:3000/api/projects \
     -H "Content-Type: application/json" \
     -d '{"name": "New Project", "description": "Project description"}'
```

#### **Get All Projects (`GET /api/projects`)**
```sh
curl -X GET http://localhost:3000/api/projects
```

#### **Update a Project (`PUT /api/projects/{project_id}`)**
```sh
curl -X PUT http://localhost:3000/api/projects/1 \
     -H "Content-Type: application/json" \
     -d '{"name": "Updated Name", "description": "Updated description"}'
```

#### **Delete a Project (`DELETE /api/projects/{project_id}`)**
```sh
curl -X DELETE http://localhost:3000/api/projects/1
```

---

### **2️⃣ Issues**
#### **Create an Issue (`POST /api/projects/{project_id}/issues`)**
```sh
curl -X POST http://localhost:3000/api/projects/1/issues \
     -H "Content-Type: application/json" \
     -d '{"title": "Bug", "description": "Issue details", "created_by": "dev@example.com", "status": "open"}'
```

#### **Get All Issues for a Project (`GET /api/projects/{project_id}/issues`)**
```sh
curl -X GET http://localhost:3000/api/projects/1/issues
```

#### **Update an Issue (`PUT /api/projects/{project_id}/issues/{issue_id}`)**
```sh
curl -X PUT http://localhost:3000/api/projects/1/issues/1 \
     -H "Content-Type: application/json" \
     -d '{"status": "resolved"}'
```

#### **Delete an Issue (`DELETE /api/projects/{project_id}/issues/{issue_id}`)**
```sh
curl -X DELETE http://localhost:3000/api/projects/1/issues/1
```

---

## 🔍 **Error Handling**
```json
{
  "error": "Project with ID 1 not found"
}
```
Possible Errors:
- **404 Not Found**: Resource not found.
- **400 Bad Request**: Invalid request.
- **500 Internal Server Error**: Unexpected failure.

---

## 🧪 **Testing**
```sh
cargo test
```

Run specific test:
```sh
cargo test test_create_project
```

## 🤝 **Contributing**
1. Fork the repository  
2. Create a feature branch  
3. Commit your changes  
4. Push to the branch  
5. Create a Pull Request  

---

## 📖 **Tutorial & Blog**
📌 **Read the guide on my blog:**  
**👉 [Building a Ticket Manager API with Rust and Axum](https://dev.to/yourusername/building-a-ticket-manager-api-with-rust-and-axum)**

---

## 🔗 **Contact**
- Blog: [dev.to/danielphilipjohnson](https://dev.to/danielphilipjohnson)

