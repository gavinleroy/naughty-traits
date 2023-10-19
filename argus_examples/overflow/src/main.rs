use diesel::prelude::*; // diesel 2.0.4, features = ["postgres"]

table! {
    users {
        id -> Integer,
        posts -> Text,
    }
}

fn main() {
    let mut conn = PgConnection::establish("_").unwrap();

    users::table
        .into_boxed()
        .group_by(users::id)
        .load::<(i32, String)>(&mut conn);
}
