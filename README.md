# My Tauri App with Rust, Actix Web, Vue.js, and Vite

This project is a desktop application using [Tauri](https://tauri.app/), [Rust](https://www.rust-lang.org/), [Actix Web](https://actix.rs/), [Vue.js](https://vuejs.org/), and [Vite](https://vitejs.dev/). It implements user authentication, role-based access control, and CRUD operations.

## Table of Contents
- [Project Setup](#project-setup)
- [MySQL Table Schema](#mysql-table-schema)
- [Backend (Actix Web)](#backend-actix-web)
- [Frontend (Vue.js + Vite)](#frontend-vuejs--vite)
- [Tauri](#tauri)
- [How to Run](#how-to-run)

---

## Project Setup

This project is divided into two main parts:

1. **Backend**: Rust, Actix Web with SQLx for database interactions  
2. **Frontend**: Vue.js with Vite as the build tool

The backend handles user authentication, role management, and CRUD operations, while the frontend interacts with the backend via API requests.

---

## MySQL Table Schema

### `users` Table

```sql
CREATE TABLE users (
  id INT AUTO_INCREMENT PRIMARY KEY,
  name VARCHAR(255) NOT NULL,
  email VARCHAR(255) NOT NULL UNIQUE,
  password VARCHAR(255) NOT NULL,
  role VARCHAR(50) NOT NULL
);

## Backend (Actix Web)
The backend is built using Actix Web, a powerful web framework in Rust. It uses:
JWT for authentication
bcrypt for password hashing
SQLx for MySQL database interaction

Features:
User Registration
Login with Email/Password
JWT-based Authentication
Role-based Access Control
CRUD for Users (Admin Only)

## Frontend (Vue.js + Vite)
The frontend is developed using Vue 3 and Vite for fast development and hot reload.

Features:
Login & Register Forms
Dashboard Page
User List (Paginated)
CRUD Actions (Admin Role)

## Tauri
Tauri turns this project into a native desktop app. It embeds the frontend and allows secure Rust backend access.

Benefits:
Small App Size
Secure APIs via Tauri Commands
Cross-platform Desktop Support

## How to Run
1. Clone the Repository
bash
Copy
Edit
git clone https://github.com/yourusername/my-tauri-rust-vue-project.git
cd my-tauri-rust-vue-project
2. Setup Backend
Ensure Rust is installed. Then:

bash
Copy
Edit
cd backend
cargo build
cargo run
Runs the backend at http://localhost:8080.

3. Setup Frontend
Ensure Node.js is installed. Then:

bash
Copy
Edit
cd frontend
npm install
npm run dev
Runs the frontend at http://localhost:3000.

4. Run Tauri App
From the frontend/ folder:

bash
Copy
Edit
npm run tauri dev
Launches the native desktop version of the app.



