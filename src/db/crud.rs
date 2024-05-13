use super::DbResult;
use scylla::{prepared_statement::PreparedStatement, *};

pub async fn prepare_insert_to_url(session: &Session) -> DbResult<PreparedStatement> {
    session
        .prepare(
            "INSERT INTO minurl.urls
        (uri, expiration, longurl)
        VALUES (?, ?, ?) if not exists",
        )
        .await
}

pub async fn prepare_get_url_by_uri(session: &Session) -> DbResult<PreparedStatement> {
    session
        .prepare("SELECT * FROM minurl.urls WHERE uri = ?")
        .await
}

pub async fn prepare_get_url_by_long_url(session: &Session) -> DbResult<PreparedStatement> {
    session
        .prepare("SELECT * FROM minurl.urls WHERE longurl = ?")
        .await
}
