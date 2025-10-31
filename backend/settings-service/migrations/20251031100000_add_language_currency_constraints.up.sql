-- Add CHECK constraints for supported languages and currencies
ALTER TABLE user_settings
ADD CONSTRAINT check_language CHECK (language IN ('en', 'es', 'fr', 'de', 'ro'));

ALTER TABLE user_settings
ADD CONSTRAINT check_currency CHECK (currency IN ('usd', 'eur', 'ron'));
