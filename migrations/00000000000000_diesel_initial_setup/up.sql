-- This file was automatically created by Diesel to setup helper functions
-- and other internal bookkeeping. This file is safe to edit, any future
-- changes will be added to existing projects as new migrations.
-- Your SQL goes here
CREATE TABLE cinemas (
  id SERIAL PRIMARY KEY,
  name TEXT UNIQUE NOT NULL
);
CREATE TABLE movies (
  id SERIAL PRIMARY KEY,
  name TEXT NOT NULL,
  description TEXT,
  pixels_box FLOAT [],
  path TEXT
);
CREATE TABLE tags (id SERIAL PRIMARY KEY, name TEXT NOT NULL);
CREATE TABLE tags_values (
  id SERIAL PRIMARY KEY,
  value TEXT NOT NULL,
  tag_id INTEGER NOT NULL REFERENCES tags ON DELETE CASCADE
);
CREATE TABLE movies_tags (
  id SERIAL PRIMARY KEY,
  movie_id INTEGER NOT NULL REFERENCES movies ON DELETE CASCADE,
  tag_id INTEGER NOT NULL REFERENCES tags ON DELETE CASCADE
);
CREATE TABLE vector_styles (
  id SERIAL PRIMARY KEY,
  name TEXT NOT NULL,
  style JSONB NOT NULL
);
CREATE TABLE vector_movies (
  id SERIAL PRIMARY KEY,
  default_style INTEGER REFERENCES vector_styles
);
CREATE TABLE vector_data (
  id SERIAL PRIMARY KEY,
  time TIMESTAMP NOT NULL,
  properties JSONB,
  vector_movie_id INTEGER NOT NULL REFERENCES vector_movies ON DELETE CASCADE
);
CREATE TABLE vector_styles_vector_movies (
  id SERIAL PRIMARY KEY,
  vector_movie_id INTEGER NOT NULL REFERENCES vector_movies ON DELETE CASCADE,
  vector_style_id INTEGER NOT NULL REFERENCES vector_styles ON DELETE CASCADE
);
CREATE TABLE colormaps (
  id SERIAL PRIMARY KEY,
  name TEXT UNIQUE NOT NULL,
  colors TEXT [] NOT NULL,
  positions FLOAT [] NOT NULL
);
CREATE TABLE color_movies (
  id SERIAL PRIMARY KEY,
  format SMALLINT NOT NULL,
  default_colormap INTEGER NOT NULL REFERENCES colormaps
);
CREATE TABLE color_movie_colormap (
  id SERIAL PRIMARY KEY,
  color_movie_id INTEGER NOT NULL REFERENCES color_movies ON DELETE CASCADE,
  colormap_id INTEGER NOT NULL REFERENCES colormaps ON DELETE CASCADE
);
CREATE TABLE images (
  id SERIAL PRIMARY KEY,
  time TIMESTAMP NOT NULL,
  path TEXT NOT NULL,
  box FLOAT [],
  color_movie_id INTEGER NOT NULL REFERENCES color_movies ON DELETE CASCADE
);
CREATE TABLE images_tags_values (
  id SERIAL PRIMARY KEY,
  tags_value_id INTEGER NOT NULL REFERENCES tags_values ON DELETE CASCADE,
  image_id INTEGER NOT NULL REFERENCES images ON DELETE CASCADE
);
CREATE TABLE cinemas_movies (
  id SERIAL PRIMARY KEY,
  exposed_format SMALLINT,
  pixels_box FLOAT [],
  cinema_id INTEGER NOT NULL REFERENCES cinemas ON DELETE CASCADE,
  movie_id INTEGER NOT NULL REFERENCES movies ON DELETE CASCADE
);
INSERT INTO colormaps
VALUES(
    DEFAULT,
    'greys',
    '{"#000000", "#FFFFFF"}',
    '{0, 1}'
  );
INSERT INTO colormaps
VALUES(
    DEFAULT,
    'rainbow',
    '{"#96005A","#0000C8","#0019FF","#0098FF","#2CFF96","#97FF00","#FFEA00","#FF6F00","#FF0000"}',
    '{0,0.125,0.25,0.375,0.5,0.625,0.75,0.875,1}'
  );