use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;

/// データベース3Dモデル
#[derive(Debug, Clone, sqlx::FromRow, Serialize, Deserialize)]
pub struct Model3D {
    pub id: String,
    pub file_name: String,
    pub file_path: String,
    pub file_size: i64,
    pub mime_type: String,
    pub uploaded_at: String,
    pub is_used: bool,
}

impl Model3D {
    /// 新規モデルを作成
    pub fn new(
        id: String,
        file_name: String,
        file_path: String,
        file_size: i64,
        mime_type: String,
    ) -> Self {
        Self {
            id,
            file_name,
            file_path,
            file_size,
            mime_type,
            uploaded_at: Utc::now().to_rfc3339(),
            is_used: false,
        }
    }

    /// モデルをデータベースに挿入
    pub async fn insert(&self, pool: &SqlitePool) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            INSERT INTO models (id, file_name, file_path, file_size, mime_type, uploaded_at, is_used)
            VALUES (?, ?, ?, ?, ?, ?, ?)
            "#,
            self.id,
            self.file_name,
            self.file_path,
            self.file_size,
            self.mime_type,
            self.uploaded_at,
            self.is_used
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    /// IDでモデルを取得
    pub async fn find_by_id(pool: &SqlitePool, id: &str) -> Result<Option<Model3D>, sqlx::Error> {
        let model = sqlx::query_as!(
            Model3D,
            r#"
            SELECT
                id as "id!",
                file_name as "file_name!",
                file_path as "file_path!",
                file_size as "file_size!",
                mime_type as "mime_type!",
                uploaded_at as "uploaded_at!",
                is_used as "is_used!"
            FROM models WHERE id = ?
            "#,
            id
        )
        .fetch_optional(pool)
        .await?;

        Ok(model)
    }

    /// IDでモデルを削除
    pub async fn delete_by_id(pool: &SqlitePool, id: &str) -> Result<bool, sqlx::Error> {
        let result = sqlx::query!("DELETE FROM models WHERE id = ?", id)
            .execute(pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }

    /// モデルを使用済みにマーク
    pub async fn mark_as_used(pool: &SqlitePool, id: &str) -> Result<bool, sqlx::Error> {
        let result = sqlx::query!("UPDATE models SET is_used = TRUE WHERE id = ?", id)
            .execute(pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }

    /// 未使用のモデル一覧を取得
    pub async fn list_unused(pool: &SqlitePool) -> Result<Vec<Model3D>, sqlx::Error> {
        let models = sqlx::query_as!(
            Model3D,
            r#"
            SELECT
                id as "id!",
                file_name as "file_name!",
                file_path as "file_path!",
                file_size as "file_size!",
                mime_type as "mime_type!",
                uploaded_at as "uploaded_at!",
                is_used as "is_used!"
            FROM models
            WHERE is_used = FALSE
            ORDER BY uploaded_at DESC
            "#
        )
        .fetch_all(pool)
        .await?;

        Ok(models)
    }
}
