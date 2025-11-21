-- Re-add category constraint with predefined values only
ALTER TABLE transactions ADD CONSTRAINT check_category CHECK (category IN (
    'Salary', 'Freelance', 'Investment', 'Gift', 'OtherIncome',
    'Food', 'Transportation', 'Housing', 'Utilities', 'Healthcare',
    'Entertainment', 'Shopping', 'Education', 'Travel', 'Insurance', 'OtherExpense'
));
