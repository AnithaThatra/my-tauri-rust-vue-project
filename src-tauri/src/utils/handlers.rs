use actix_web::{web, HttpResponse, HttpRequest, Responder};
use sqlx::MySqlPool;
use crate::utils::models::{User, NewUser, LoginRequest, LoginResponse, UpdateUser, PublicUser};
use crate::utils::auth::{create_jwt, Claims};
use bcrypt::{hash, verify, DEFAULT_COST};

/// Register a new user.
pub async fn register(db: web::Data<MySqlPool>, form: web::Json<NewUser>) -> impl Responder {
    // Hash the user's password before storing it
    let hashed = hash(&form.password, DEFAULT_COST).unwrap();

    let result = sqlx::query!(
        "INSERT INTO users (name, email, password, role) VALUES (?, ?, ?, ?)",
        form.name,
        form.email,
        hashed,
        form.role
    )
    .execute(db.get_ref())
    .await;

    match result {
        Ok(_) => HttpResponse::Created().body("User registered"),
        Err(_) => HttpResponse::InternalServerError().body("Failed to register user"),
    }
}

/// User login to authenticate and get JWT token.
pub async fn login(db: web::Data<MySqlPool>, form: web::Json<LoginRequest>) -> impl Responder {
    // Fetch the user from the database
    let user = sqlx::query_as!(
        User,
        "SELECT id, name, email, role, password FROM users WHERE email = ?",
        form.email
    )
    .fetch_optional(db.get_ref())
    .await
    .unwrap();

    match user {
        Some(u) if verify(&form.password, &u.password).unwrap() => {
            match create_jwt(&u) {
                Ok(token) => HttpResponse::Ok().json(LoginResponse { token, name: u.name, role: u.role }),
                Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
            }
        }
        _ => HttpResponse::Unauthorized().body("Invalid credentials"),
    }
}

/// Protected route to test authenticated access.
pub async fn protected(req: HttpRequest) -> impl Responder {
    if let Some(claims) = req.extensions().get::<Claims>() {
        HttpResponse::Ok().json(claims)
    } else {
        HttpResponse::Unauthorized().body("Missing or invalid token")
    }
}

/// Create a new user (Admins only).
pub async fn create_user(
    db_pool: web::Data<MySqlPool>,
    req: HttpRequest,
    user: web::Json<NewUser>,
) -> impl Responder {
    // Check if the request has valid token claims and that the role is "admin"
    let claims_opt = req.extensions().get::<Claims>().cloned();
    if claims_opt.is_none() {
        return HttpResponse::Unauthorized().body("Missing token claims");
    }

    let claims = claims_opt.unwrap();
    if claims.role != "admin" {
        return HttpResponse::Forbidden().body("Admins only can create new users");
    }

    // Validate the input fields
    if user.name.trim().is_empty() || user.email.trim().is_empty() || user.role.trim().is_empty() {
        return HttpResponse::BadRequest().body("All fields are required");
    }

    // Validate role
    if user.role != "admin" && user.role != "user" {
        return HttpResponse::BadRequest().body("Invalid role. Only 'admin' or 'user' roles are allowed.");
    }

    // Hash the password before inserting it into the database
    let hashed_password = hash(&user.password, DEFAULT_COST).unwrap_or_else(|_| "".to_string());

    // Proceed with creating the user
    let result = sqlx::query!(
        "INSERT INTO users (name, email, password, role) VALUES (?, ?, ?, ?)",
        user.name,
        user.email,
        hashed_password,
        user.role
    )
    .execute(db_pool.get_ref())
    .await;

    match result {
        Ok(_) => HttpResponse::Created().body("User created successfully"),
        Err(err) => {
            eprintln!("Error creating user: {}", err); // Log the error for debugging
            HttpResponse::InternalServerError().body("Error creating user")
        }
    }
}

/// Fetch all users (Admins only).
pub async fn fetch_all_users(
    db: web::Data<MySqlPool>,
    req: HttpRequest,
) -> impl Responder {
    // Check that a valid token exists
    let claims_opt = req.extensions().get::<Claims>().cloned();
    if claims_opt.is_none() {
        return HttpResponse::Unauthorized().body("Missing token claims");
    }

    // Log the user fetching data
    let claims = claims_opt.unwrap();
    println!("User '{}' with role '{}' is fetching users", claims.sub, claims.role);

    // Fetch all users
    match sqlx::query_as!(
        PublicUser,
        "SELECT id, name, email, role FROM users"
    )
    .fetch_all(db.get_ref())
    .await
    {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(e) => {
            eprintln!("DB error: {}", e);
            HttpResponse::InternalServerError().body("Failed to fetch users")
        }
    }
}

/// Update a user's data (Admins only).
pub async fn update_user(
    db_pool: web::Data<MySqlPool>,
    req: HttpRequest,
    user: web::Json<UpdateUser>,
) -> impl Responder {
    // Check if token claims are present and validate role
    let claims_opt = req.extensions().get::<Claims>().cloned();
    if claims_opt.is_none() {
        return HttpResponse::Unauthorized().body("Missing token claims");
    }

    let claims = claims_opt.unwrap();
    if claims.role != "admin" {
        return HttpResponse::Forbidden().body("Admins only");
    }

    // Validate input fields
    if user.name.trim().is_empty() || user.email.trim().is_empty() || user.role.trim().is_empty() {
        return HttpResponse::BadRequest().body("All fields are required");
    }

    // Update user in the database
    let result = sqlx::query!(
        "UPDATE users SET name = ?, email = ?, role = ? WHERE id = ?",
        user.name,
        user.email,
        user.role,
        user.id
    )
    .execute(db_pool.get_ref())
    .await;

    match result {
        Ok(_) => HttpResponse::Ok().body("User updated"),
        Err(_) => HttpResponse::InternalServerError().body("Error updating user"),
    }
}

/// Delete a user (Admins only).
pub async fn delete_user(
    db_pool: web::Data<MySqlPool>,
    req: HttpRequest,
    user_id: web::Path<i32>,
) -> impl Responder {
    // Check if token claims are present and validate role
    let claims_opt = req.extensions().get::<Claims>().cloned();
    if claims_opt.is_none() {
        return HttpResponse::Unauthorized().body("Missing token claims");
    }

    let claims = claims_opt.unwrap();
    if claims.role != "admin" {
        return HttpResponse::Forbidden().body("Admins only");
    }

    // Delete user from the database
    match sqlx::query!("DELETE FROM users WHERE id = ?", *user_id)
        .execute(db_pool.get_ref())
        .await
    {
        Ok(_) => HttpResponse::Ok().body("User deleted"),
        Err(_) => HttpResponse::InternalServerError().body("Error deleting user"),
    }
}
