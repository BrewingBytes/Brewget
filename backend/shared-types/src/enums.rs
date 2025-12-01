use serde::{Deserialize, Serialize};

/// Supported currencies in the application
#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Currency {
    /// United States Dollar
    #[serde(rename = "USD")]
    Usd,
    /// Euro
    #[serde(rename = "EUR")]
    Eur,
    /// British Pound
    #[serde(rename = "GBP")]
    Gbp,
    /// Canadian Dollar
    #[serde(rename = "CAD")]
    Cad,
    /// Japanese Yen
    #[serde(rename = "JPY")]
    Jpy,
    /// Romanian Leu
    #[serde(rename = "RON")]
    Ron,
}

impl Currency {
    /// Returns the currency code as a string
    pub fn as_str(&self) -> &'static str {
        match self {
            Currency::Usd => "USD",
            Currency::Eur => "EUR",
            Currency::Gbp => "GBP",
            Currency::Cad => "CAD",
            Currency::Jpy => "JPY",
            Currency::Ron => "RON",
        }
    }

    /// Returns all supported currencies
    pub fn all() -> &'static [Currency] {
        &[
            Currency::Usd,
            Currency::Eur,
            Currency::Gbp,
            Currency::Cad,
            Currency::Jpy,
            Currency::Ron,
        ]
    }
}

impl std::fmt::Display for Currency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Supported languages in the application
#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Language {
    /// English
    #[serde(rename = "en")]
    En,
    /// Spanish
    #[serde(rename = "es")]
    Es,
    /// French
    #[serde(rename = "fr")]
    Fr,
    /// German
    #[serde(rename = "de")]
    De,
    /// Romanian
    #[serde(rename = "ro")]
    Ro,
}

impl Language {
    /// Returns the language code as a string
    pub fn as_str(&self) -> &'static str {
        match self {
            Language::En => "en",
            Language::Es => "es",
            Language::Fr => "fr",
            Language::De => "de",
            Language::Ro => "ro",
        }
    }

    /// Returns all supported languages
    pub fn all() -> &'static [Language] {
        &[
            Language::En,
            Language::Es,
            Language::Fr,
            Language::De,
            Language::Ro,
        ]
    }
}

impl std::fmt::Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Supported wallet types in the application
#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum WalletType {
    /// Regular account wallet
    #[default]
    Account,
    /// Savings account
    Savings,
    /// Deposit account
    Deposit,
    /// Credit card
    CreditCard,
    /// Loan account
    Loan,
}

impl WalletType {
    /// Returns the wallet type as a string
    pub fn as_str(&self) -> &'static str {
        match self {
            WalletType::Account => "Account",
            WalletType::Savings => "Savings",
            WalletType::Deposit => "Deposit",
            WalletType::CreditCard => "CreditCard",
            WalletType::Loan => "Loan",
        }
    }

    /// Returns all supported wallet types
    pub fn all() -> &'static [WalletType] {
        &[
            WalletType::Account,
            WalletType::Savings,
            WalletType::Deposit,
            WalletType::CreditCard,
            WalletType::Loan,
        ]
    }
}

impl std::fmt::Display for WalletType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Supported transaction types in the application
#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransactionType {
    /// Income transaction
    Income,
    /// Expense transaction
    Expense,
    /// Transfer between wallets
    Transfer,
}

impl TransactionType {
    /// Returns the transaction type as a string
    pub fn as_str(&self) -> &'static str {
        match self {
            TransactionType::Income => "Income",
            TransactionType::Expense => "Expense",
            TransactionType::Transfer => "Transfer",
        }
    }

    /// Returns all supported transaction types
    pub fn all() -> &'static [TransactionType] {
        &[
            TransactionType::Income,
            TransactionType::Expense,
            TransactionType::Transfer,
        ]
    }
}

impl std::fmt::Display for TransactionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Supported transaction categories in the application
#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransactionCategory {
    /// Salary income
    Salary,
    /// Freelance/business income
    Freelance,
    /// Investment returns
    Investment,
    /// Gift received
    Gift,
    /// Other income
    OtherIncome,
    /// Food and dining
    Food,
    /// Transportation
    Transportation,
    /// Housing/rent/mortgage
    Housing,
    /// Utilities (electricity, water, etc)
    Utilities,
    /// Healthcare
    Healthcare,
    /// Entertainment
    Entertainment,
    /// Shopping
    Shopping,
    /// Education
    Education,
    /// Travel
    Travel,
    /// Insurance
    Insurance,
    /// Other expense
    OtherExpense,
}

impl TransactionCategory {
    /// Returns the transaction category as a string
    pub fn as_str(&self) -> &'static str {
        match self {
            TransactionCategory::Salary => "Salary",
            TransactionCategory::Freelance => "Freelance",
            TransactionCategory::Investment => "Investment",
            TransactionCategory::Gift => "Gift",
            TransactionCategory::OtherIncome => "OtherIncome",
            TransactionCategory::Food => "Food",
            TransactionCategory::Transportation => "Transportation",
            TransactionCategory::Housing => "Housing",
            TransactionCategory::Utilities => "Utilities",
            TransactionCategory::Healthcare => "Healthcare",
            TransactionCategory::Entertainment => "Entertainment",
            TransactionCategory::Shopping => "Shopping",
            TransactionCategory::Education => "Education",
            TransactionCategory::Travel => "Travel",
            TransactionCategory::Insurance => "Insurance",
            TransactionCategory::OtherExpense => "OtherExpense",
        }
    }

    /// Returns all supported transaction categories
    pub fn all() -> &'static [TransactionCategory] {
        &[
            TransactionCategory::Salary,
            TransactionCategory::Freelance,
            TransactionCategory::Investment,
            TransactionCategory::Gift,
            TransactionCategory::OtherIncome,
            TransactionCategory::Food,
            TransactionCategory::Transportation,
            TransactionCategory::Housing,
            TransactionCategory::Utilities,
            TransactionCategory::Healthcare,
            TransactionCategory::Entertainment,
            TransactionCategory::Shopping,
            TransactionCategory::Education,
            TransactionCategory::Travel,
            TransactionCategory::Insurance,
            TransactionCategory::OtherExpense,
        ]
    }
}

impl std::fmt::Display for TransactionCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_currency_serialization() {
        let usd = Currency::Usd;
        let json = serde_json::to_string(&usd).unwrap();
        assert_eq!(json, r#""USD""#);

        let eur = Currency::Eur;
        let json = serde_json::to_string(&eur).unwrap();
        assert_eq!(json, r#""EUR""#);
    }

    #[test]
    fn test_currency_deserialization() {
        let json = r#""USD""#;
        let currency: Currency = serde_json::from_str(json).unwrap();
        assert_eq!(currency, Currency::Usd);

        let json = r#""EUR""#;
        let currency: Currency = serde_json::from_str(json).unwrap();
        assert_eq!(currency, Currency::Eur);
    }

    #[test]
    fn test_currency_display() {
        assert_eq!(Currency::Usd.to_string(), "USD");
        assert_eq!(Currency::Eur.to_string(), "EUR");
        assert_eq!(Currency::Gbp.to_string(), "GBP");
    }

    #[test]
    fn test_currency_all() {
        let all = Currency::all();
        assert_eq!(all.len(), 6);
        assert!(all.contains(&Currency::Usd));
        assert!(all.contains(&Currency::Eur));
    }

    #[test]
    fn test_language_serialization() {
        let en = Language::En;
        let json = serde_json::to_string(&en).unwrap();
        assert_eq!(json, r#""en""#);

        let es = Language::Es;
        let json = serde_json::to_string(&es).unwrap();
        assert_eq!(json, r#""es""#);
    }

    #[test]
    fn test_language_deserialization() {
        let json = r#""en""#;
        let language: Language = serde_json::from_str(json).unwrap();
        assert_eq!(language, Language::En);

        let json = r#""es""#;
        let language: Language = serde_json::from_str(json).unwrap();
        assert_eq!(language, Language::Es);
    }

    #[test]
    fn test_language_display() {
        assert_eq!(Language::En.to_string(), "en");
        assert_eq!(Language::Es.to_string(), "es");
        assert_eq!(Language::Fr.to_string(), "fr");
    }

    #[test]
    fn test_language_all() {
        let all = Language::all();
        assert_eq!(all.len(), 5);
        assert!(all.contains(&Language::En));
        assert!(all.contains(&Language::Es));
    }

    #[test]
    fn test_wallet_type_serialization() {
        let account = WalletType::Account;
        let json = serde_json::to_string(&account).unwrap();
        assert_eq!(json, r#""Account""#);

        let savings = WalletType::Savings;
        let json = serde_json::to_string(&savings).unwrap();
        assert_eq!(json, r#""Savings""#);
    }

    #[test]
    fn test_wallet_type_deserialization() {
        let json = r#""Account""#;
        let wallet_type: WalletType = serde_json::from_str(json).unwrap();
        assert_eq!(wallet_type, WalletType::Account);

        let json = r#""CreditCard""#;
        let wallet_type: WalletType = serde_json::from_str(json).unwrap();
        assert_eq!(wallet_type, WalletType::CreditCard);
    }

    #[test]
    fn test_wallet_type_display() {
        assert_eq!(WalletType::Account.to_string(), "Account");
        assert_eq!(WalletType::Savings.to_string(), "Savings");
        assert_eq!(WalletType::CreditCard.to_string(), "CreditCard");
    }

    #[test]
    fn test_wallet_type_all() {
        let all = WalletType::all();
        assert_eq!(all.len(), 5);
        assert!(all.contains(&WalletType::Account));
        assert!(all.contains(&WalletType::Savings));
    }

    #[test]
    fn test_wallet_type_default() {
        let default = WalletType::default();
        assert_eq!(default, WalletType::Account);
    }

    #[test]
    fn test_transaction_type_serialization() {
        let income = TransactionType::Income;
        let json = serde_json::to_string(&income).unwrap();
        assert_eq!(json, r#""Income""#);

        let expense = TransactionType::Expense;
        let json = serde_json::to_string(&expense).unwrap();
        assert_eq!(json, r#""Expense""#);
    }

    #[test]
    fn test_transaction_type_deserialization() {
        let json = r#""Income""#;
        let transaction_type: TransactionType = serde_json::from_str(json).unwrap();
        assert_eq!(transaction_type, TransactionType::Income);

        let json = r#""Transfer""#;
        let transaction_type: TransactionType = serde_json::from_str(json).unwrap();
        assert_eq!(transaction_type, TransactionType::Transfer);
    }

    #[test]
    fn test_transaction_type_display() {
        assert_eq!(TransactionType::Income.to_string(), "Income");
        assert_eq!(TransactionType::Expense.to_string(), "Expense");
        assert_eq!(TransactionType::Transfer.to_string(), "Transfer");
    }

    #[test]
    fn test_transaction_category_serialization() {
        let food = TransactionCategory::Food;
        let json = serde_json::to_string(&food).unwrap();
        assert_eq!(json, r#""Food""#);

        let salary = TransactionCategory::Salary;
        let json = serde_json::to_string(&salary).unwrap();
        assert_eq!(json, r#""Salary""#);
    }

    #[test]
    fn test_transaction_category_deserialization() {
        let json = r#""Food""#;
        let category: TransactionCategory = serde_json::from_str(json).unwrap();
        assert_eq!(category, TransactionCategory::Food);

        let json = r#""Salary""#;
        let category: TransactionCategory = serde_json::from_str(json).unwrap();
        assert_eq!(category, TransactionCategory::Salary);
    }

    #[test]
    fn test_transaction_category_display() {
        assert_eq!(TransactionCategory::Food.to_string(), "Food");
        assert_eq!(TransactionCategory::Salary.to_string(), "Salary");
        assert_eq!(
            TransactionCategory::Entertainment.to_string(),
            "Entertainment"
        );
    }
}
