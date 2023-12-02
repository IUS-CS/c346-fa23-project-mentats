use crate::models::user::*;
use mysql::{params, prelude::*};


pub fn insert_new_ueser(
    conn: &mut mysql::PooledConn,
    my_email: String,
    my_username: String,
    my_password: String,
    my_first_name: String,
    my_last_name: String,
) -> mysql::error::Result<u64> {
    conn.exec_drop(
        "
        INSERT INTO users (email, username, pass, first_name, last_name)
        VALUES (:email, :username,:pass, :first_name, :last_name)
        ",
        params! {
            "email" => my_email,
            "username" => my_username,
            "pass" => my_password,
            "first_name" => my_first_name,
            "last_name" => my_last_name,
        },
    )
    .map(|_| conn.last_insert_id())
}

//TODO select all fields from the user table
pub fn select_all_users(conn: &mut mysql::PooledConn) -> mysql::error::Result<Vec<UserData>> {
    conn.query_map(
        "
        SELECT id, email, username, first_name, last_name
        FROM users
        WHERE is_deleted = 0
        ",
        |(my_id, my_email, my_username, my_first_name, my_last_name)| UserData {
            id: my_id,
            email: my_email,
            username: my_username,
            first_name: my_first_name,
            last_name: my_last_name,
        },
    )
}

pub fn select_password_by_username(
    conn: &mut mysql::PooledConn,
    username: String,
) -> mysql::error::Result<String> {
    conn.exec_first(
        "
        SELECT pass
        FROM users
        WHERE username = :username AND is_deleted = 0
        ",
        params! {
            "username" => username
        },
    )
    .map(|pass| pass.unwrap())
}

pub fn select_single_user(
    conn: &mut mysql::PooledConn,
    id: u64,
) -> mysql::error::Result<SingleUserUnconvertedResponseData> {
    conn.exec_first(
        "
        SELECT email, username, first_name, last_name, pronouns, bio, profile_pic
        FROM users
        WHERE id = :id AND is_deleted = 0
        ",
        params! {
            "id" => id
        }, //|(email, username, first_name, last_name, pronouns, bio, profile_pic)| SingleUserUnconvertedResponseData {
           //    email: email,
           //    username: username,
           //    first_name: first_name,
           //    last_name: last_name,
           //    pronouns: pronouns,
           //    bio: bio,
           //    profile_pic: profile_pic
           //}
    )
    .map(|single_user_response| single_user_response.unwrap())
}

pub fn select_user_by_id(
    conn: &mut mysql::PooledConn,
    username: String,
) -> mysql::error::Result<UserData> {
    conn.exec_first(
        "
        SELECT id, email, username, first_name, last_name
        FROM users
        WHERE username = :username AND is_deleted = 0
        ",
        params! {
            "username" => username
        },
    )
    .map(|user| user.unwrap())
}

pub fn update_bio_and_profilepic(
    conn: &mut mysql::PooledConn,
    username: String,
    bio: String,
    profile_pic: String,
) -> mysql::error::Result<UserUpdateData> {
    conn.exec_drop(
        r"UPDATE users 
        SET bio = :bio, profile_pic = :profile_pic 
        WHERE username = :username AND is_deleted = 0",
        params! {
            "bio" => bio,
            "profile_pic" => profile_pic,
            "username" => username.clone()
        },
    );
    conn.exec_first(
        "
        SELECT username, bio, profile_pic
        FROM users
        WHERE username = :username AND is_deleted = 0
        ",
        params! {
            "username" => username
        },
    )
    .map(|user_update| user_update.unwrap())
}

pub fn select_userid_by_userstring(
    conn: &mut mysql::PooledConn,
    username: String,
) -> mysql::error::Result<u64> {
    conn.exec_first(
        "
        SELECT id
        FROM users
        WHERE username = :username AND is_deleted = 0
        ",
        params! {
            "username" => username
        },
    )
    .map(|user_id| user_id.unwrap())
}

pub fn select_userstring_by_userid(
    conn: &mut mysql::PooledConn,
    id: u64,
) -> mysql::error::Result<String> {
    conn.exec_first(
        "
        SELECT username
        FROM users
        WHERE id = :id AND is_deleted = 0
        ",
        params! {
            "id" => id
        },
    )
    .map(|username| username.unwrap())
}