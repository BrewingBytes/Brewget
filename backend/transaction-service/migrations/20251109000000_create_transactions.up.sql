-- Create transactions table
CREATE TABLE transactions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL,
    wallet_id UUID NOT NULL,
    amount DECIMAL(15, 2) NOT NULL,
    transaction_type VARCHAR(20) NOT NULL,
    category VARCHAR(50) NOT NULL,
    description TEXT,
    transaction_date TIMESTAMP NOT NULL DEFAULT NOW(),
    destination_wallet_id UUID,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    
    -- Foreign key to wallets table
    CONSTRAINT fk_wallet FOREIGN KEY (wallet_id) REFERENCES wallets(id) ON DELETE CASCADE,
    CONSTRAINT fk_destination_wallet FOREIGN KEY (destination_wallet_id) REFERENCES wallets(id) ON DELETE SET NULL,
    
    -- Check constraints
    CONSTRAINT check_amount_positive CHECK (amount > 0),
    CONSTRAINT check_transaction_type CHECK (transaction_type IN ('Income', 'Expense', 'Transfer')),
    CONSTRAINT check_category CHECK (category IN (
        'Salary', 'Freelance', 'Investment', 'Gift', 'OtherIncome',
        'Food', 'Transportation', 'Housing', 'Utilities', 'Healthcare',
        'Entertainment', 'Shopping', 'Education', 'Travel', 'Insurance', 'OtherExpense'
    )),
    -- For transfers, destination_wallet_id must be set
    CONSTRAINT check_transfer_destination CHECK (
        (transaction_type != 'Transfer') OR (destination_wallet_id IS NOT NULL)
    )
);

-- Create indexes for faster queries
CREATE INDEX idx_transactions_user_id ON transactions(user_id);
CREATE INDEX idx_transactions_wallet_id ON transactions(wallet_id);
CREATE INDEX idx_transactions_destination_wallet_id ON transactions(destination_wallet_id);
CREATE INDEX idx_transactions_transaction_date ON transactions(transaction_date DESC);
CREATE INDEX idx_transactions_user_wallet ON transactions(user_id, wallet_id);
