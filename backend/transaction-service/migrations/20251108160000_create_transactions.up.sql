-- Create transactions table
CREATE TABLE transactions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL,
    wallet_id UUID NOT NULL,
    amount DECIMAL(15, 2) NOT NULL,
    transaction_type VARCHAR(20) NOT NULL,
    category VARCHAR(20) NOT NULL,
    description TEXT,
    transaction_date TIMESTAMP NOT NULL DEFAULT NOW(),
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    CONSTRAINT fk_wallet FOREIGN KEY (wallet_id) REFERENCES wallets(id) ON DELETE CASCADE
);

-- Create indexes for faster queries
CREATE INDEX idx_transactions_user_id ON transactions(user_id);
CREATE INDEX idx_transactions_wallet_id ON transactions(wallet_id);
CREATE INDEX idx_transactions_date ON transactions(transaction_date DESC);
CREATE INDEX idx_transactions_type ON transactions(transaction_type);

-- Add check constraints for transaction_type
ALTER TABLE transactions
ADD CONSTRAINT check_transaction_type CHECK (
    transaction_type IN ('Income', 'Expense', 'Transfer')
);

-- Add check constraints for category
ALTER TABLE transactions
ADD CONSTRAINT check_category CHECK (
    category IN (
        'Salary', 'Freelance', 'Investment', 'Gift',
        'Food', 'Transport', 'Housing', 'Entertainment',
        'Healthcare', 'Shopping', 'Education',
        'Transfer', 'Other'
    )
);
