-- Remove category constraint to allow custom categories
ALTER TABLE transactions DROP CONSTRAINT IF EXISTS check_category;
