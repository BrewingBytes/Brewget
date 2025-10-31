-- Remove CHECK constraints for language and currency
ALTER TABLE user_settings
DROP CONSTRAINT IF EXISTS check_language;

ALTER TABLE user_settings
DROP CONSTRAINT IF EXISTS check_currency;
