use std::error::Error;

use scylla::prepared_statement::PreparedStatement;
use scylla::Session;

pub struct Queries {
    pub get_user: PreparedStatement,
    pub get_user_composite: PreparedStatement,
    pub get_group: PreparedStatement,
    pub get_all_groups_without_schedule: PreparedStatement,
    pub add_user: PreparedStatement,
    pub add_user_composite: PreparedStatement,
    pub add_group: PreparedStatement,
    pub delete_group: PreparedStatement,
    pub update_group_epoch: PreparedStatement,
    pub update_schedule: PreparedStatement,
    pub update_user_password: PreparedStatement,
    pub append_group_scope: PreparedStatement,
    pub get_internal: PreparedStatement,
    pub add_internal: PreparedStatement,
    pub update_internal_epoch: PreparedStatement,
}

impl Queries {
    pub async fn new(session: &Session) -> Result<Self, Box<dyn Error + 'static>> {
        Ok(Self {
            get_user: session.prepare("SELECT * FROM users WHERE username = ?").await?,
            get_user_composite: session.prepare("SELECT * FROM users_access_token_username_composite WHERE access_token = ?").await?,
            get_group: session.prepare("SELECT * FROM groups WHERE id = ?").await?,
            get_all_groups_without_schedule: session.prepare("SELECT id, epoch, name FROM groups").await?,
            add_user: session.prepare("INSERT INTO users (username, access_token, group_scope, password) VALUES (?, ?, ?, ?)").await?,
            add_user_composite: session.prepare("INSERT INTO users_access_token_username_composite (access_token, username) VALUES (?, ?)").await?,
            add_group: session.prepare("INSERT INTO groups (id, epoch, name, schedule) VALUES (?, ?, ?, ?)").await?,
            delete_group: session.prepare("DELETE FROM groups WHERE id = ?").await?,
            update_group_epoch: session.prepare("UPDATE groups SET epoch = ? WHERE id = ?").await?,
            update_schedule: session.prepare("UPDATE groups SET schedule = ? WHERE id = ?").await?,
            update_user_password: session.prepare("UPDATE users SET password = ? WHERE username = ?").await?,
            append_group_scope: session.prepare("UPDATE users SET group_scope = group_scope + ? WHERE username = ?").await?,
            get_internal: session.prepare("SELECT * FROM internal WHERE id = ?").await?,
            add_internal: session.prepare("INSERT INTO internal (id, epoch) VALUES (?, ?)").await?,
            update_internal_epoch: session.prepare("UPDATE internal SET epoch = ? WHERE id = ?").await?,
        })
    }
}
