-- Drop wallet type index
DROP INDEX IF EXISTS idx_wallets_type;

-- Revert wallet_type column comment
COMMENT ON COLUMN wallets.wallet_type IS 'Type of wallet (Account, Savings, Deposit, CreditCard, Loan)';
ALTER TABLE wallets ALTER COLUMN wallet_type TYPE VARCHAR(50);

-- Revert currency column comment
COMMENT ON COLUMN wallets.currency IS NULL;
ALTER TABLE wallets ALTER COLUMN currency TYPE VARCHAR(20);

-- Re-add category column
ALTER TABLE wallets ADD COLUMN category VARCHAR(100);
CREATE INDEX idx_wallets_category ON wallets(user_id, category);
