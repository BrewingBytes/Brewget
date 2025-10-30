-- Remove language and currency constraints
ALTER TABLE user_settings
DROP CONSTRAINT check_language_supported;

ALTER TABLE user_settings
DROP CONSTRAINT check_currency_supported;
