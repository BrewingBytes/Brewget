-- Add CHECK constraints for Currency and WalletType to enforce data integrity at database level

-- Add CHECK constraint for currency column to match shared-types Currency enum
ALTER TABLE wallets ADD CONSTRAINT check_currency_enum 
    CHECK (currency IN ('USD', 'EUR', 'GBP', 'CAD', 'JPY', 'RON'));

-- Add CHECK constraint for wallet_type column to match shared-types WalletType enum
ALTER TABLE wallets ADD CONSTRAINT check_wallet_type_enum 
    CHECK (wallet_type IN ('Account', 'Savings', 'Deposit', 'CreditCard', 'Loan'));
