CREATE TABLE bbses (
    id BIGSERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    path_name TEXT NOT NULL UNIQUE,
    description TEXT NOT NULL,
    category TEXT NOT NULL,
    hidden BOOLEAN NOT NULL DEFAULT false
)
