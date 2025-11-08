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
