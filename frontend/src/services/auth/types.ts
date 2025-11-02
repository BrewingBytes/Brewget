export interface LoginResponse {
  token: string;
}

export interface TranslationKeyMessage {
  translation_key: string;
}

export type RegisterResponse = TranslationKeyMessage;
export type ForgotPasswordResponse = TranslationKeyMessage;
export type ChangePasswordResponse = TranslationKeyMessage;
export type LogoutResponse = TranslationKeyMessage;
export type ActivateResponse = TranslationKeyMessage;
export type VerifyResponse = TranslationKeyMessage;

// WebAuthn public key credential types
export interface WebAuthnCredential {
  id: string;
  rawId: string;
  type: string;
  response: {
    clientDataJSON: string;
    attestationObject?: string;
    authenticatorData?: string;
    signature?: string;
    userHandle?: string | null;
  };
}

export interface PasskeyRegisterStartResponse {
  user_id: string;
  creation_options: Record<string, unknown>;
}

export interface PasskeyLoginStartResponse {
  request_options: Record<string, unknown>;
}
