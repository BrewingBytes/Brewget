-- Create wallets table
CREATE TABLE IF NOT EXISTS wallets (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL,
    name VARCHAR(100) NOT NULL,
    balance DECIMAL(15, 2) NOT NULL DEFAULT 0.00,
    currency VARCHAR(20) NOT NULL DEFAULT 'usd',
    wallet_type VARCHAR(50) NOT NULL DEFAULT 'general',
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT fk_user FOREIGN KEY (user_id) REFERENCES user_settings(user_id) ON DELETE CASCADE,
    CONSTRAINT check_wallet_currency CHECK (currency IN ('usd', 'eur', 'ron')),
    CONSTRAINT check_wallet_type CHECK (wallet_type IN ('general', 'savings', 'business', 'personal'))
);

-- Create index on user_id for faster queries
CREATE INDEX idx_wallets_user_id ON wallets(user_id);
