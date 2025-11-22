use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;

/// データベースモンスター情報
#[derive(Debug, Clone, sqlx::FromRow, Serialize, Deserialize)]
pub struct Monster {
    pub id: String,

    // モンスター基本情報
    pub name: String,
    pub max_hp: i64,
    pub short_range_attack_power: i64,
    pub long_range_attack_power: i64,
    pub defense_power: i64,
    pub move_speed: i64,
    pub attack_range: i64,
    pub attack_cooldown: i64,
    pub size_type: String, // DBには文字列として保存

    // 3Dモデルファイル情報
    pub file_name: String,
    pub file_path: String,
    pub file_size: i64,
    pub mime_type: String,

    // メタデータ
    pub uploaded_at: String,
    pub is_used: bool,
}

// 後方互換性のためのエイリアス
pub type Model3D = Monster;

impl Monster {
    /// 新規モンスターを作成
    pub fn new(
        id: String,
        name: String,
        max_hp: i64,
        short_range_attack_power: i64,
        long_range_attack_power: i64,
        defense_power: i64,
        move_speed: i64,
        attack_range: i64,
        attack_cooldown: i64,
        size_type: String,
        file_name: String,
        file_path: String,
        file_size: i64,
        mime_type: String,
    ) -> Self {
        Self {
            id,
            name,
            max_hp,
            short_range_attack_power,
            long_range_attack_power,
            defense_power,
            move_speed,
            attack_range,
            attack_cooldown,
            size_type,
            file_name,
            file_path,
            file_size,
            mime_type,
            uploaded_at: Utc::now().to_rfc3339(),
            is_used: false,
        }
    }

    /// モンスターをデータベースに挿入
    pub async fn insert(&self, pool: &SqlitePool) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            INSERT INTO monsters (
                id, name, max_hp, short_range_attack_power, long_range_attack_power,
                defense_power, move_speed, attack_range, attack_cooldown, size_type,
                file_name, file_path, file_size, mime_type, uploaded_at, is_used
            )
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
            self.id,
            self.name,
            self.max_hp,
            self.short_range_attack_power,
            self.long_range_attack_power,
            self.defense_power,
            self.move_speed,
            self.attack_range,
            self.attack_cooldown,
            self.size_type,
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

    /// IDでモンスターを取得
    pub async fn find_by_id(pool: &SqlitePool, id: &str) -> Result<Option<Monster>, sqlx::Error> {
        let monster = sqlx::query_as!(
            Monster,
            r#"
            SELECT
                id as "id!",
                name as "name!",
                max_hp as "max_hp!",
                short_range_attack_power as "short_range_attack_power!",
                long_range_attack_power as "long_range_attack_power!",
                defense_power as "defense_power!",
                move_speed as "move_speed!",
                attack_range as "attack_range!",
                attack_cooldown as "attack_cooldown!",
                size_type as "size_type!",
                file_name as "file_name!",
                file_path as "file_path!",
                file_size as "file_size!",
                mime_type as "mime_type!",
                uploaded_at as "uploaded_at!",
                is_used as "is_used!"
            FROM monsters WHERE id = ?
            "#,
            id
        )
        .fetch_optional(pool)
        .await?;

        Ok(monster)
    }

    /// IDでモンスターを削除
    pub async fn delete_by_id(pool: &SqlitePool, id: &str) -> Result<bool, sqlx::Error> {
        let result = sqlx::query!("DELETE FROM monsters WHERE id = ?", id)
            .execute(pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }

    /// モンスターを使用済みにマーク
    pub async fn mark_as_used(pool: &SqlitePool, id: &str) -> Result<bool, sqlx::Error> {
        let result = sqlx::query!("UPDATE monsters SET is_used = TRUE WHERE id = ?", id)
            .execute(pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }

    /// 未使用のモンスター一覧を取得
    pub async fn list_unused(pool: &SqlitePool) -> Result<Vec<Monster>, sqlx::Error> {
        let monsters = sqlx::query_as!(
            Monster,
            r#"
            SELECT
                id as "id!",
                name as "name!",
                max_hp as "max_hp!",
                short_range_attack_power as "short_range_attack_power!",
                long_range_attack_power as "long_range_attack_power!",
                defense_power as "defense_power!",
                move_speed as "move_speed!",
                attack_range as "attack_range!",
                attack_cooldown as "attack_cooldown!",
                size_type as "size_type!",
                file_name as "file_name!",
                file_path as "file_path!",
                file_size as "file_size!",
                mime_type as "mime_type!",
                uploaded_at as "uploaded_at!",
                is_used as "is_used!"
            FROM monsters
            WHERE is_used = FALSE
            ORDER BY uploaded_at DESC
            "#
        )
        .fetch_all(pool)
        .await?;

        Ok(monsters)
    }
}
