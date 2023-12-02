use bcrypt::{hash, verify, DEFAULT_COST};

use crate::models::user::*;
use crate::queries::{campaign::*, game::*, session::*, user::*};
use crate::persistence::error::*;
use crate::persistence::tools::*;

pub fn create_user_verify(
    pool: &mysql::Pool,
    email: String,
    username: String,
    password: String,
    first_name: String,
    last_name: String,
) -> Result<(), PersistenceError> {
    if email.replace(' ', "").trim().is_empty() {
        return Err(PersistenceError::EmptyEmail);
    }

    if username.replace(' ', "").trim().is_empty() {
        return Err(PersistenceError::EmptyUsername);
    }

    if password.replace(' ', "").trim().is_empty() {
        return Err(PersistenceError::EmptyPassword);
    }

    let mut conn = pool.get_conn()?;
    let hashed_password = hash(password, DEFAULT_COST)?;

    let last_insert_id = insert_new_ueser(
        &mut conn,
        email,
        username,
        hashed_password,
        first_name,
        last_name,
    )?;

    if last_insert_id > 0 {
        Ok(())
    } else {
        Err(PersistenceError::Unknown)
    }
}

pub fn login_user_verify(
    pool: &mysql::Pool,
    username: String,
    password: String,
) -> Result<UserData, PersistenceError> {
    if username.replace(' ', "").trim().is_empty() {
        return Err(PersistenceError::EmptyUsername);
    }

    if password.replace(' ', "").trim().is_empty() {
        return Err(PersistenceError::EmptyPassword);
    }

    let mut conn = pool.get_conn()?;
    let hashed_password = select_password_by_username(&mut conn, username.clone())?;

    if verify(password, &hashed_password)? {
        Ok(select_user_by_id(&mut conn, username)?)
    } else {
        Err(PersistenceError::UnknownUser)
    }
}

pub fn get_users_verify(pool: &mysql::Pool) -> Result<UserResponseData, PersistenceError> {
    let mut conn = pool.get_conn()?;

    Ok(UserResponseData {
        user_data: select_all_users(&mut conn)?,
    })
}

pub fn get_single_user_persistence(
    pool: &mysql::Pool,
    username: String,
) -> Result<SingleUserConvertedResponseData, PersistenceError> {
    let mut conn = pool.get_conn()?;

    if username.replace(' ', "").trim().is_empty() {
        return Err(PersistenceError::EmptyUsername);
    }
    //get user_id
    let user_id = select_userid_by_userstring(&mut conn, username).unwrap();
    let single_user = select_single_user(&mut conn, user_id).unwrap();
    //let create_time_vec: Vec<&str> = single_user.create_time.split(',').collect();
    //let year = create_time_vec[0].parse::<i32>();
    //let month = create_time_vec[1].parse::<u32>();
    //let day = create_time_vec[2].parse::<u32>();
    //let hour = create_time_vec[3].parse::<u32>();
    //let minute = create_time_vec[4].parse::<u32>();
    //let second = create_time_vec[5].parse::<u32>();
    //let date = NaiveDate::from_ymd_opt(year.unwrap(), month.unwrap(), day.unwrap()).unwrap();
    //let time = NaiveTime::from_hms_opt(hour.unwrap(), minute.unwrap(), second.unwrap()).unwrap();
    //let create_time_datetime = NaiveDateTime::new(date, time);

    let single_user_converted = SingleUserConvertedResponseData {
        //create_time: create_time_datetime,
        email: single_user.email,
        username: single_user.username,
        first_name: single_user.first_name,
        last_name: single_user.last_name,
        pronouns: single_user.pronouns,
        bio: single_user.bio,
        profile_pic: single_user.profile_pic,
    };

    Ok(single_user_converted)
}


//function that checks if user exists and calls the query to update
pub fn update_user(
    pool: &mysql::Pool,
    username: String,
    bio: String,
    profile_pic: String,
) -> Result<UserUpdateData, PersistenceError> {
    let mut conn = pool.get_conn()?;

    if username.replace(' ', "").trim().is_empty() {
        Err(PersistenceError::EmptyUsername)
    } else {
        Ok(update_bio_and_profilepic(
            &mut conn,
            username,
            bio,
            profile_pic,
        )?)
    }
}