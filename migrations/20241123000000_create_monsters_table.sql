-- モンスター情報テーブル
CREATE TABLE IF NOT EXISTS monsters (
    id TEXT PRIMARY KEY,

    -- モンスター基本情報
    name TEXT NOT NULL,
    max_hp INTEGER NOT NULL,
    short_range_attack_power INTEGER NOT NULL,
    long_range_attack_power INTEGER NOT NULL,
    defense_power INTEGER NOT NULL,
    move_speed INTEGER NOT NULL,
    attack_range INTEGER NOT NULL,
    attack_cooldown INTEGER NOT NULL,
    size_type TEXT NOT NULL,  -- 'Small', 'Medium', 'Large'

    -- 3Dモデルファイル情報
    file_name TEXT NOT NULL,
    file_path TEXT NOT NULL,
    file_size INTEGER NOT NULL,
    mime_type TEXT NOT NULL,

    -- メタデータ
    uploaded_at TEXT NOT NULL,
    is_used BOOLEAN NOT NULL DEFAULT 0
);
