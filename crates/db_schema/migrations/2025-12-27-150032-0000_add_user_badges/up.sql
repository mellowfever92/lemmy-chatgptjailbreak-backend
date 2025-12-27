-- Create badges table
CREATE TABLE badges (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL UNIQUE,
    description TEXT,
    image_url VARCHAR(500),
    is_assignable_by_mods BOOLEAN DEFAULT TRUE NOT NULL,
    is_self_selectable BOOLEAN DEFAULT FALSE NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);

-- Create user_badges junction table
CREATE TABLE user_badges (
    id SERIAL PRIMARY KEY,
    person_id INT NOT NULL REFERENCES person(id) ON DELETE CASCADE,
    badge_id INT NOT NULL REFERENCES badges(id) ON DELETE CASCADE,
    assigned_at TIMESTAMP NOT NULL DEFAULT NOW(),
    assigned_by INT REFERENCES person(id) ON DELETE SET NULL,
    UNIQUE(person_id, badge_id)
);

-- Indexes for performance
CREATE INDEX idx_user_badges_person ON user_badges(person_id);
CREATE INDEX idx_user_badges_badge ON user_badges(badge_id);
CREATE INDEX idx_badges_name ON badges(name);

