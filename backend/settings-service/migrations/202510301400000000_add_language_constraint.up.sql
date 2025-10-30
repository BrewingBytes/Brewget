-- Add constraints to only allow supported languages and currencies
ALTER TABLE user_settings
ADD CONSTRAINT check_language_supported
CHECK (language IN ('en', 'es', 'fr', 'de', 'ro'));

ALTER TABLE user_settings
ADD CONSTRAINT check_currency_supported
CHECK (currency IN ('usd', 'eur', 'ron'));
