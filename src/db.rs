use std::error::Error;

use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHasher,
};
use scylla::Session;
use uuid::Uuid;

use crate::data::{Queries, User};

#[macro_export]
macro_rules! query {
    ($s:expr, $q:expr, $d:expr, $e:literal) => {{
        let result = $s.execute($q, $d).await;
        if let Err(error) = result {
            #[cfg(debug_assertions)]
            println!("Query returned an error: {}", error.to_string());
            return Err($e.into());
        } else {
            result.unwrap()
        }
    }};
}

#[macro_export]
macro_rules! query_one {
    ($s:expr, $q:expr, $d:expr, $e:literal) => {{
        let result = $s.execute($q, $d).await;
        let result = if let Err(error) = result {
            #[cfg(debug_assertions)]
            println!("Query returned an error: {}", error.to_string());
            return Err($e.into());
        } else {
            result.unwrap()
        };

        let rows = if let None = result.rows {
            #[cfg(debug_assertions)]
            println!("Query returned no rows");
            return Err($e.into());
        } else {
            result.rows.unwrap()
        };

        let row = rows.into_iter().next();
        let row = if let None = row {
            #[cfg(debug_assertions)]
            println!("Query returned no rows");
            return Err($e.into());
        } else {
            row.unwrap()
        };

        let value = row.into_typed();
        if let Err(error) = value {
            #[cfg(debug_assertions)]
            println!(
                "Query deserialization returned an error: {}",
                error.to_string()
            );
            return Err($e.into());
        } else {
            value.unwrap()
        }
    }};
}

#[macro_export]
macro_rules! query_all {
    ($s:expr, $q:expr, $d:expr, $e:literal) => {{
        let result = $s.execute($q, $d).await;
        let result = if let Err(error) = result {
            #[cfg(debug_assertions)]
            println!("Query returned an error: {}", error.to_string());
            return Err($e.into());
        } else {
            result.unwrap()
        };

        let rows = if let None = result.rows {
            #[cfg(debug_assertions)]
            println!("Query returned no rows");
            return Err($e.into());
        } else {
            result.rows.unwrap()
        };

        let value = rows.into_iter().map(|v| v.into_typed()).collect();
        if let Err(error) = value {
            #[cfg(debug_assertions)]
            println!(
                "Query deserialization returned an error: {}",
                error.to_string()
            );
            return Err($e.into());
        } else {
            value.unwrap()
        }
    }};
}

#[macro_export]
macro_rules! query_one_checked {
    ($s:expr, $q:expr, $d:expr) => {{
        let result = $s
            .execute($q, $d)
            .await
            .map_err(|e| format!("Query returned an error: {}", e.to_string()))
            .and_then(|v| v.rows.ok_or("Query returned no rows".into()))
            .and_then(|v| v.into_iter().next().ok_or("Query returned no rows".into()))
            .and_then(|v| {
                v.into_typed().map_err(|e| {
                    format!("Query deserialization returned an error: {}", e.to_string())
                })
            });
        result
    }};
}

pub async fn create_user(
    session: &Session,
    queries: &Queries,
    username: String,
    password: String,
) -> Result<User, Box<dyn Error + 'static>> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let hashed = if let Ok(result) = argon2.hash_password(password.as_bytes(), &salt) {
        result.to_string()
    } else {
        return Err("Could not hash the password".into());
    };

    let user = User {
        username: username,
        password: hashed,
        access_token: Uuid::new_v4(),
        group_scope: None,
    };

    query!(session, &queries.add_user, &user, "Cannot add new user");
    query!(
        session,
        &queries.add_user_composite,
        (&user.access_token, &user.username),
        "Cannot add new user"
    );

    Ok(user)
}
