#![allow(unused)]
use sqlx::{PgPool, Row, postgres::PgPoolOptions};
use uuid::Uuid;

use crate::models::my_models::{Person, NewPerson};

pub struct PostgresRepository {
    pool: PgPool,
}

impl PostgresRepository {
    pub async fn connect(url: String) -> Self {
        PostgresRepository {
            pool: PgPoolOptions::new()
            .max_connections(5)
            .connect(&url)
            .await
            .unwrap(),
        }
    }

    pub async fn find_person(&self, id: Uuid) -> Result<Option<Person>, sqlx::Error> {
        sqlx::query_as(
            "
            SELECT id, name, nick, birth_date, stack
            FROM people
            WHERE id = $1
            ",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
            
    }

    pub async fn create_person(&self, new_person: NewPerson) -> Result<Person, sqlx::Error> {
        print!("teste 1");
        print!("{}", new_person.name.as_str());
        print!("{}", new_person.nick.as_str());
        print!("{}", new_person.birth_date);

        sqlx::query_as("
        INSERT INTO people (id, name, nick, birth_date, stack)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING id, name, nick, birth_date, stack
        ",
        )
        .bind(Uuid::now_v7())
        .bind(new_person.name.as_str())
        .bind(new_person.nick.as_str())
        .bind(new_person.birth_date)
        .bind(new_person
            .stack
            .map(|stack| stack.into_iter().map(String::from).collect::<Vec<String>>()), 
        )
        .fetch_one(&self.pool)
        .await
    }

    pub async fn search_people(&self, query: String) -> Result<Vec<Person>, sqlx::Error> {
        sqlx::query_as!(
            Person, 
            "
            SELECT id, name, nick, birth_date, stack
            FROM people
            WHERE search ILIKE $1
            LIMIT 50
            ",
            format!("%{query}%"),
        )
        .fetch_all(&self.pool)
        .await
    }

    pub async fn count_people(&self) -> Result<i64, sqlx::Error> {
        sqlx::query("SELECT count(*)FROM people")
            .fetch_one(&self.pool)
            .await
            .map(|row| row.get(0))        
    }
}