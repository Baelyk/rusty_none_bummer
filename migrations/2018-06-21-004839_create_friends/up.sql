CREATE TABLE friends (
  id SERIAL PRIMARY KEY,
  name TEXT NOT NULL,
  points INTEGER NOT NULL,
  activities JSONB
)
