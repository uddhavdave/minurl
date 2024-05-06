use std::error::Error;

use chrono::Utc;
use crud::prepare_insert_to_url;
use scylla::prepared_statement::PreparedStatement;
use scylla::transport::errors::NewSessionError;
use scylla::transport::errors::QueryError;
use scylla::QueryResult;
use scylla::Session;
use scylla::SessionBuilder;

pub type DbResult<T> = Result<T, QueryError>;

use self::crud::{prepare_get_url_by_long_url, prepare_get_url_by_uri};

pub mod crud;

#[derive(Debug)]
pub struct DbExecutor {
    session: Session,
    insert_to_url: PreparedStatement,
    get_url_by_uri: PreparedStatement,
    get_url_by_long_url: PreparedStatement,
}

impl DbExecutor {
    pub async fn build() -> Result<Self, Box<dyn Error>> {
        let session = connect().await?;

        session
            .query(
                "create keyspace if not exists minurl 
with replication = { 
'class' : 'SimpleStrategy',
'replication_factor' : 1
};",
                &[],
            )
            .await?;

        session
            .query(
                "
create table if not exists minurl.urls (
    uri        text primary key,
    expiration timestamp,
    longurl    text);",
                &[],
            )
            .await?;

        session
            .query(
                "
create index if not exists longurlindex
    on minurl.urls (longurl);
",
                &[],
            )
            .await?;

        let insert_to_url = prepare_insert_to_url(&session).await?;
        let get_url_by_uri = prepare_get_url_by_uri(&session).await?;
        let get_url_by_long_url = prepare_get_url_by_long_url(&session).await?;

        Ok(DbExecutor {
            session,
            insert_to_url,
            get_url_by_uri,
            get_url_by_long_url,
        })
    }

    pub async fn insert_to_url(
        &self,
        long_url: String,
        short_url: String,
        expiration_ts: chrono::DateTime<Utc>,
    ) -> DbResult<QueryResult> {
        self.session
            .execute(&self.insert_to_url, (short_url, expiration_ts, long_url))
            .await
    }

    pub async fn get_url_by_uri(&self, short_url: String) -> DbResult<QueryResult> {
        self.session
            .execute(&self.get_url_by_uri, (short_url,))
            .await
    }

    pub async fn get_url_by_long_url(&self, long_url: String) -> DbResult<QueryResult> {
        self.session
            .execute(&self.get_url_by_long_url, (long_url,))
            .await
    }
}

async fn connect() -> Result<Session, NewSessionError> {
    SessionBuilder::new()
        .known_node("cassandra:9042")
        .build()
        .await
}
