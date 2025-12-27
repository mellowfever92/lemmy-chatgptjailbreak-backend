-- Create badges table for storing badge definitions
CREATE TABLE badge (
    id serial PRIMARY KEY,
    name varchar(64) NOT NULL UNIQUE,
    description text,
    image_url text NOT NULL,
    is_assignable_by_mods boolean NOT NULL DEFAULT FALSE,
    is_self_selectable boolean NOT NULL DEFAULT FALSE,
    published timestamptz NOT NULL DEFAULT now(),
    updated timestamptz
);

-- Create user_badge junction table for badge assignments
CREATE TABLE person_badge (
    id serial PRIMARY KEY,
    person_id int REFERENCES person ON UPDATE CASCADE ON DELETE CASCADE NOT NULL,
    badge_id int REFERENCES badge ON UPDATE CASCADE ON DELETE CASCADE NOT NULL,
    assigned_at timestamptz NOT NULL DEFAULT now(),
    assigned_by int REFERENCES person ON UPDATE CASCADE ON DELETE SET NULL,
    UNIQUE (person_id, badge_id)
);

-- Add indexes for common queries
CREATE INDEX idx_person_badge_person_id ON person_badge (person_id);

CREATE INDEX idx_person_badge_badge_id ON person_badge (badge_id);

CREATE INDEX idx_badge_name ON badge (lower(name::text));
