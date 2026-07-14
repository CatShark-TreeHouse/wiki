-- Seed the controlled-content lists with the legacy entries from the old
-- zuri-cat-tree-rules repo (rules/careful-with-these.md).
--
-- Aliases follow the network convention: single tokens, no whitespace.
-- Idempotent: `INSERT OR IGNORE` keys on the UNIQUE alias, so re-running (or
-- running after moderators have added entries) never duplicates or clobbers.
-- Apply with `just db-seed`.

CREATE TABLE IF NOT EXISTS controlled_content (
    id             INTEGER PRIMARY KEY AUTOINCREMENT,
    alias          TEXT NOT NULL UNIQUE,
    content_type   TEXT NOT NULL,
    control_method TEXT NOT NULL,
    reason         TEXT,
    created_at     TEXT NOT NULL,
    updated_at     TEXT NOT NULL,
    deleted_at     TEXT
);

INSERT OR IGNORE INTO controlled_content
    (alias, content_type, control_method, reason, created_at, updated_at, deleted_at)
VALUES
    -- Banned artists (stickers included)
    ('Canynekhai', 'artist', 'banned', 'Cub', '2026-07-13T00:00:00+00:00', '2026-07-13T00:00:00+00:00', NULL),
    ('JailBird', 'artist', 'banned', 'Cub', '2026-07-13T00:00:00+00:00', '2026-07-13T00:00:00+00:00', NULL),
    ('Nozomy', 'artist', 'banned', 'Zoophilia', '2026-07-13T00:00:00+00:00', '2026-07-13T00:00:00+00:00', NULL),
    ('RedRusker', 'artist', 'banned', 'Cub', '2026-07-13T00:00:00+00:00', '2026-07-13T00:00:00+00:00', NULL),
    ('Sabuke', 'artist', 'banned', 'Drawn zoophilia', '2026-07-13T00:00:00+00:00', '2026-07-13T00:00:00+00:00', NULL),
    ('Zackary911', 'artist', 'banned', 'Drawing young / cub characters', '2026-07-13T00:00:00+00:00', '2026-07-13T00:00:00+00:00', NULL),
    ('Zaush', 'artist', 'banned', 'Draws young (underage) characters', '2026-07-13T00:00:00+00:00', '2026-07-13T00:00:00+00:00', NULL),

    -- Banned characters (stickers and any art they appear in)
    ('GearFox', 'character', 'banned', 'Cub / young art', '2026-07-13T00:00:00+00:00', '2026-07-13T00:00:00+00:00', NULL),
    ('Foxgear', 'character', 'banned', 'Alias of GearFox', '2026-07-13T00:00:00+00:00', '2026-07-13T00:00:00+00:00', NULL),
    ('Huskgear', 'character', 'banned', 'Alias of GearFox', '2026-07-13T00:00:00+00:00', '2026-07-13T00:00:00+00:00', NULL),

    -- Banned kinks
    ('Cub', 'tag', 'banned', NULL, '2026-07-13T00:00:00+00:00', '2026-07-13T00:00:00+00:00', NULL),
    ('Gore', 'tag', 'banned', NULL, '2026-07-13T00:00:00+00:00', '2026-07-13T00:00:00+00:00', NULL),
    ('Scat', 'tag', 'banned', NULL, '2026-07-13T00:00:00+00:00', '2026-07-13T00:00:00+00:00', NULL),
    ('Rape', 'tag', 'banned', NULL, '2026-07-13T00:00:00+00:00', '2026-07-13T00:00:00+00:00', NULL),
    ('RealFeral', 'tag', 'banned', 'Feral depicting real animals. Can be considered soft zoo; also falls into rape. Fantastical creatures (gryphons, dragons) are generally allowed since they do not exist and are depicted as intelligent, consensual creatures.', '2026-07-13T00:00:00+00:00', '2026-07-13T00:00:00+00:00', NULL),

    -- Spoilered kinks (content warning + spoiler required)
    ('Vore', 'tag', 'spoilered', 'Rare kink. Can be considered uncomfortable.', '2026-07-13T00:00:00+00:00', '2026-07-13T00:00:00+00:00', NULL),
    ('Watersports', 'tag', 'spoilered', 'Rare kink. Can be found gross.', '2026-07-13T00:00:00+00:00', '2026-07-13T00:00:00+00:00', NULL),
    ('ExtremeHyper', 'tag', 'spoilered', 'Rare kink. Can be considered uncomfortable.', '2026-07-13T00:00:00+00:00', '2026-07-13T00:00:00+00:00', NULL);
