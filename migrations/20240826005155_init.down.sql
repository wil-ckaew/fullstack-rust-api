-- Add down migration script here

-- Drop tables in reverse order of creation to handle foreign key constraints
DROP TABLE IF EXISTS student_parents;
DROP TABLE IF EXISTS videos;
DROP TABLE IF EXISTS photos;
DROP TABLE IF EXISTS file_metadata;
DROP TABLE IF EXISTS logs;
DROP TABLE IF EXISTS users;
DROP TABLE IF EXISTS parents;
DROP TABLE IF EXISTS students;

-- Drop tables for tasks and documents
DROP TABLE IF EXISTS documents;
DROP TABLE IF EXISTS tasks;

-- Optionally, you can also drop the extension if you no longer need it
-- Ensure no tables are using UUIDs before dropping the extension
-- DROP EXTENSION IF EXISTS "uuid-ossp";
