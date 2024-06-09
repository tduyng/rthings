use axum::{
    body::Body,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use sqlx::{postgres::PgQueryResult, PgExecutor};
use uuid::Uuid;

#[derive(Debug)]
pub struct Secret {
    pub uuid: Uuid,
    pub file_name: Option<String>,
    pub contents: Option<Vec<u8>>,
}

impl Secret {
    pub async fn find<'e>(db: impl PgExecutor<'e>, uuid: Uuid) -> Result<Self, sqlx::Error> {
        sqlx::query_as!(
            Self,
            "select uuid, file_name, contents from secrets where uuid = $1",
            uuid
        )
        .fetch_one(db)
        .await
    }

    pub async fn update_contents<'e>(
        &mut self,
        db: impl PgExecutor<'e>,
        contents: Vec<u8>,
    ) -> Result<PgQueryResult, sqlx::Error> {
        self.contents = Some(contents);

        sqlx::query_as!(
            Self,
            "update secrets set contents = $1 where uuid = $2",
            self.contents,
            self.uuid
        )
        .execute(db)
        .await
    }
}

impl IntoResponse for Secret {
    fn into_response(self) -> axum::response::Response {
        let dispo_header = match self.file_name {
            Some(file_name) => format!("attachment; filename=\"{}\"", file_name),
            None => "attachment".to_string(),
        };

        match self.contents {
            None => Response::builder()
                .status(StatusCode::NO_CONTENT)
                .body(Body::empty()),
            Some(contents) => Response::builder()
                .header("Content-Disposition", dispo_header)
                .body(Body::from(contents)),
        }
        .expect("Response Builder with known setup should not fail")
    }
}
