-- Drop indexes
DROP INDEX IF EXISTS idx_badge_name;

DROP INDEX IF EXISTS idx_person_badge_badge_id;

DROP INDEX IF EXISTS idx_person_badge_person_id;

-- Drop tables in reverse order (child first, then parent)
DROP TABLE IF EXISTS person_badge;

DROP TABLE IF EXISTS badge;
