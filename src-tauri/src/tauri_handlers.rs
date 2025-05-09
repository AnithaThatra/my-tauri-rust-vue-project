use tauri::State;
use std::sync::Arc;
use sqlx::MySqlPool;
use bcrypt::{hash, verify, DEFAULT_COST};

use crate::utils::models::{PublicUser, NewUser, UpdateUser, LoginRequest, LoginResponse, User};
use crate::utils::auth::{create_jwt, validate_jwt};

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<MySqlPool>,
}

// Register a new user
#[tauri::command]
pub async fn register_tauri(user: NewUser, state: State<'_, AppState>) -> Result<(), String> {
    

    // Hash password using bcrypt
    let hashed_password = hash(user.password.as_bytes(), DEFAULT_COST)
        .map_err(|e| e.to_string())?;

    // Insert new user into the database
    sqlx::query!(
        "INSERT INTO users (name, email, password, role) VALUES (?, ?, ?, ?)",
        user.name,
        user.email,
        hashed_password,
        user.role
    )
    .execute(&*state.db)
    .await
    .map_err(|e| e.to_string())?;

    Ok(())
}


// Login a user
#[tauri::command]
pub async fn login_tauri(
    form: LoginRequest,
    state: State<'_, AppState>,
) -> Result<LoginResponse, String> {
    if form.email.trim().is_empty() || form.password.trim().is_empty() {
        return Err("Email and password cannot be empty.".into());
    }


    // Fetch user from the database by email
    let user = sqlx::query_as!(
        User,
        "SELECT id, name, email, role, password FROM users WHERE email = ?",
        form.email
    )
    .fetch_optional(&*state.db)
    .await
    .map_err(|e| e.to_string())?;

    // Verify password and generate JWT token if valid
    match user {
        Some(u) => {
            let is_valid = verify(form.password.as_bytes(), &u.password)
                .map_err(|e| e.to_string())?;
            if is_valid {
                let token = create_jwt(&u).map_err(|e| e.to_string())?;
                Ok(LoginResponse { token, name: u.name,  role: u.role, })
            } else {
                Err("Invalid credentials.".into())
            }
        }
        None => Err("Invalid credentials.".into()),
    }
}


// Create a new user (admin only)
#[tauri::command]
pub async fn create_user_tauri(
    user: NewUser,
    token: String,
    state: State<'_, AppState>
) -> Result<(), String> {
    // Validate JWT token and check role
    let claims = validate_jwt(&token).map_err(|_| "Invalid or expired token")?;
    if claims.role != "admin" {
        return Err("Access denied: only admins can create users".to_string());
    }

    // Hash password using bcrypt
    let hashed_password = hash(user.password.as_bytes(), DEFAULT_COST)
        .map_err(|e| e.to_string())?;

    // Insert the new user into the database
    sqlx::query!(
        "INSERT INTO users (name, email, password, role) VALUES (?, ?, ?, ?)",
        user.name,
        user.email,
        hashed_password,
        user.role
    )
    .execute(&*state.db)
    .await
    .map_err(|e| e.to_string())?;

    Ok(())
}

// Fetch all users
#[tauri::command]
pub async fn fetch_all_users_tauri(state: State<'_, AppState>) -> Result<Vec<PublicUser>, String> {
    sqlx::query_as!(PublicUser, "SELECT id, name, email, role FROM users")
        .fetch_all(&*state.db)
        .await
        .map_err(|e| e.to_string())
}

// Update user information (admin only)
#[tauri::command]
pub async fn update_user_tauri(
    user: UpdateUser,
    token: String,
    state: State<'_, AppState>
) -> Result<(), String> {
    // Validate JWT token and check role
    let claims = validate_jwt(&token).map_err(|_| "Invalid or expired token")?;
    if claims.role != "admin" {
        return Err("Access denied: only admins can update users".to_string());
    }


    // Update the user in the database
    sqlx::query!(
        "UPDATE users SET name = ?, email = ?, role = ? WHERE id = ?",
        user.name,
        user.email,
        user.role,
        user.id
    )
    .execute(&*state.db)
    .await
    .map_err(|e| format!("DB error: {}", e))?;

    Ok(())
}

// Delete a user (admin only)
#[tauri::command]
pub async fn delete_user_tauri(
    id: i32,
    token: String,
    state: State<'_, AppState>
) -> Result<(), String> {
    // Validate JWT token and check role
    let claims = validate_jwt(&token).map_err(|_| "Invalid or expired token")?;
    if claims.role != "admin" {
        return Err("Access denied: only admins can delete users".to_string());
    }

    // Delete the user from the database
    sqlx::query!("DELETE FROM users WHERE id = ?", id)
        .execute(&*state.db)
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}
