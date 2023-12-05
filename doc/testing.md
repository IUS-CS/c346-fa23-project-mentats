Testing
Date: 2023-10-21

Integration Testing
The authentication and data storage features of our code involves methods for SQL queries. Methods for SQL queries are found in queries.rs and are shown below.

pub fn insert_new_user()
pub fn select_all_users()
pub fn select_password_by_username()
pub fn select_user_by_id()
Integration testing is done through setting up a predefined database, running each method for SQL queries, and confirming the result. For the code below, after running this method we would confirm if all fields were received.

pub fn select_all_users(conn: &mut mysql::PooledConn) -> mysql::error::Result<Vec<UserData>> {
conn.query_map(
"
SELECT id, email, username, first_name, last_name
FROM users
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
The persistence.rs file includes error handling for the SQL queries listed. Error handling can be tested to see if the errors can be reproduced.

The routes.rs file contains methods to respond to HTTP requests.The HTTP requests below are handled, put through persistence.rs, and finally a response is received from the corresponding sql query. As with queries.rs, testing is done with a predefined database and is used to establish functionality and usability with other methods (such as error handling.)

    pub enum PersistenceError {
EmptyEmail,
EmptyUsername,
EmptyPassword,
BcryptError(bcrypt::BcryptError),
MysqlError(mysql::Error),
UnknownUser,
Unknown,
}
Future Testing.
Testing considered for this and future sprints includes individual unit tests. Rust has a built-in testing framework for testing individual methods. With testing SQL queries comes the need for a mock database, one that allows testing every request. We expect that testing the higher level features will require individual tests, and requiring that individual tests pass will become a part of pull requests for this reason.
