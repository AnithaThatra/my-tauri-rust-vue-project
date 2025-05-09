# My Tauri App with Rust, Actix Web, Vue.js, and Vite

This project is a desktop application using [Tauri](https://tauri.app/), [Rust](https://www.rust-lang.org/), [Actix Web](https://actix.rs/), [Vue.js](https://vuejs.org/), and [Vite](https://vitejs.dev/). It implements user authentication, role-based access control, and CRUD operations.

## Table of Contents
- [Project Setup](#project-setup)
- [MySQL Table Schema](#mysql-table-schema)
- [Backend (Actix Web)](#backend-actix-web)
- [Frontend (Vue.js + Vite)](#frontend-vuejs--vite)
- [Tauri](#tauri)
- [How to Run](#how-to-run)
- [Contributing](#contributing)

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
