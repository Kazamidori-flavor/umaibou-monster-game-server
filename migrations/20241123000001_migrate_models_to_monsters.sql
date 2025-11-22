-- modelsテーブルからmonstersテーブルへのデータ移行と削除

-- Step 1: 既存のmodelsデータがあればmonstersに移行
INSERT INTO monsters (
    id, name, max_hp, short_range_attack_power, long_range_attack_power,
    defense_power, move_speed, attack_range, attack_cooldown, size_type,
    file_name, file_path, file_size, mime_type, uploaded_at, is_used
)
SELECT
    id,
    'Legacy Model' as name,          -- デフォルトのモンスター名
    100 as max_hp,                    -- デフォルトHP
    10 as short_range_attack_power,  -- デフォルト近距離攻撃力
    5 as long_range_attack_power,    -- デフォルト遠距離攻撃力
    5 as defense_power,               -- デフォルト防御力
    10 as move_speed,                 -- デフォルト移動速度
    2 as attack_range,                -- デフォルト攻撃範囲
    1000 as attack_cooldown,          -- デフォルト攻撃クールダウン(ms)
    'Medium' as size_type,            -- デフォルトサイズ
    file_name,
    file_path,
    file_size,
    mime_type,
    uploaded_at,
    is_used
FROM models
WHERE NOT EXISTS (
    SELECT 1 FROM monsters WHERE monsters.id = models.id
);

-- Step 2: modelsテーブルを削除
DROP TABLE IF EXISTS models;
