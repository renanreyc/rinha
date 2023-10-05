// Define your structs here
pub mod my_models {

use sqlx::{FromRow, postgres::PgRow, Row};
use uuid::Uuid;
use serde::{Serialize, Deserialize};
use time::Date;
time::serde::format_description!(date_format, Date, "[year]-[month]-[day]");

macro_rules! new_string_type {
    ($type: ident, max_lengh = $max_length:expr, error = $error_message:expr) => {
        #[derive(Clone, Deserialize)]
        #[serde(try_from = "String")]
        pub struct $type(String);

        impl $type {
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl TryFrom<String> for $type {
            type Error = &'static str;
        
            fn try_from(value: String) -> Result<Self, Self::Error> {
                if value.len() <= $max_length {
                    Ok($type(value))
                } else {
                    Err($error_message)
                }
            }
        }
    };    
}

new_string_type!(PersonName, max_lengh = 100, error = "name is too big");
new_string_type!(Nick, max_lengh = 32, error = "nick is too big");

#[derive(Clone, Deserialize)]
#[serde(try_from = "String")]

pub struct Tech(String);

impl TryFrom<String> for Tech {
    type Error = &'static str;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() <= 32 {
            Ok(Self(value))
        } else {
            Err("tech is too big")
        }
    }
}

impl  From<Tech> for String {
    fn from(value: Tech) -> Self {
        value.0
    }
}


#[derive(Clone, Serialize, Deserialize)]
    pub struct Person {
        pub id: Uuid,
        #[serde(rename = "nome")]
        pub name: String,
        #[serde(rename = "apelido")]
        pub nick: String,
        #[serde(rename = "nascimento", with="date_format")]
        pub birth_date: Date,
        pub stack: Option<Vec<String>>,
    }

    #[derive(Clone, Deserialize)]
    pub struct NewPerson {
        #[serde(rename = "nome")]
        pub name: PersonName,
        #[serde(rename = "apelido")]
        pub nick: Nick,
        #[serde(rename = "nascimento", with="date_format")]
        pub birth_date: Date,
        pub stack: Option<Vec<Tech>>,
    }

    #[derive(Deserialize)]
    pub struct PersonSearchQuery {
    #[serde(rename= "t")]
    query: String,
}

impl PersonSearchQuery {
    pub fn query(&self) -> &str {
        &self.query
    }
}

impl<'a>  FromRow<'a, PgRow> for Person {
    fn from_row(row: &PgRow) -> Result<Self, sqlx::Error> {
        Ok(Person { 
            id: row.try_get("id")?, 
            name: row.try_get("name")?, 
            nick: row.try_get("nick")?, 
            birth_date: row.try_get("birth_date")?, 
            stack: row.try_get("stack")?, 
        })
    }    
}

}


// ... Other models