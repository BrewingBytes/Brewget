-- Re-add the check constraint for category
ALTER TABLE transactions
ADD CONSTRAINT check_category CHECK (
    category IN (
        'Salary', 'Freelance', 'Investment', 'Gift',
        'Food', 'Transport', 'Housing', 'Entertainment',
        'Healthcare', 'Shopping', 'Education',
        'Transfer', 'Other'
    )
);
