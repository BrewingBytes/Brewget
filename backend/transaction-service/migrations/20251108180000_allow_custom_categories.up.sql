-- Drop the check constraint on category to allow custom values
ALTER TABLE transactions
DROP CONSTRAINT IF EXISTS check_category;

-- Category can now be any string value (built-in or custom)
