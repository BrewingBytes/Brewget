-- Remove CHECK constraints for Currency and WalletType

ALTER TABLE wallets DROP CONSTRAINT IF EXISTS check_wallet_type_enum;
ALTER TABLE wallets DROP CONSTRAINT IF EXISTS check_currency_enum;
