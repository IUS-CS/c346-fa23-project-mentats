# Testing
Date: 2023-10-21

## Integration Testing
The authentication and data storage features of our code involves methods for SQL queries. Methods for SQL queries are 
found in queries.rs and are shown below. 

    pub fn insert_new_user()
    pub fn select_all_users()
    pub fn select_password_by_username()
    pub fn select_user_by_id()

The persistence.rs 
file includes error handling for the SQL queries listed. Error handling can be tested to see if the errors can be
reproduced.

The routes.rs file contains methods to respond to HTTP requests.The HTTP requests below are handled, put through 
persistence.rs, and finally a response is received from the corresponding sql query.

    pub enum PersistenceError {
    EmptyEmail,
    EmptyUsername,
    EmptyPassword,
    BcryptError(bcrypt::BcryptError),
    MysqlError(mysql::Error),
    UnknownUser,
    Unknown,
    }

Integration testing is contained within test.rs, tests for individual database queries as well as


## Future Testing.
Testing considered for this and future sprints includes individual unit tests. Rust has a built-in testing framework for
testing individual methods. With testing SQL queries comes the need for a mock database, one that allows testing every 
request. We expect that testing the higher level features will require individual tests, and requiring that individual 
tests pass will become a part of pull requests for this reason.
