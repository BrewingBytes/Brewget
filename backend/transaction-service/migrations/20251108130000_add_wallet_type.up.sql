-- Add wallet_type column to wallets table
ALTER TABLE wallets ADD COLUMN wallet_type VARCHAR(50) NOT NULL DEFAULT 'Account';

-- Add comment
COMMENT ON COLUMN wallets.wallet_type IS 'Type of wallet (Account, Savings, Deposit, CreditCard, Loan)';
