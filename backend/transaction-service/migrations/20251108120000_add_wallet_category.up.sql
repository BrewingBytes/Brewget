-- Add category to wallets table
ALTER TABLE wallets ADD COLUMN category VARCHAR(100);

-- Create index for faster queries by category
CREATE INDEX idx_wallets_category ON wallets(user_id, category);
