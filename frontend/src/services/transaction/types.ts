export interface Wallet {
  id: string;
  user_id: string;
  name: string;
  balance: number;
  currency: string;
  wallet_type: string;
  created_at: string;
  updated_at: string;
}

export interface CreateWallet {
  name: string;
  balance?: number;
  currency: string;
  wallet_type?: string;
}

export interface UpdateWallet {
  name?: string;
  currency?: string;
  wallet_type?: string;
}

export interface Transaction {
  id: string;
  user_id: string;
  wallet_id: string;
  amount: number;
  transaction_type: string;
  category: string;
  description: string | null;
  transaction_date: string;
  created_at: string;
  updated_at: string;
}

export interface CreateTransaction {
  wallet_id: string;
  amount: number;
  transaction_type: string;
  category: string;
  description?: string;
  transaction_date?: string;
}

export interface UpdateTransaction {
  amount?: number;
  transaction_type?: string;
  category?: string;
  description?: string;
  transaction_date?: string;
}

export interface CustomCategory {
  id: string;
  user_id: string;
  name: string;
  transaction_type: string;
  created_at: string;
  updated_at: string;
}

export interface CreateCustomCategory {
  name: string;
  transaction_type: string;
}

export interface UpdateCustomCategory {
  name?: string;
  transaction_type?: string;
}

