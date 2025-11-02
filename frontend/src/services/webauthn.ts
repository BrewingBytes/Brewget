import { base64URLStringToBuffer, bufferToBase64URLString } from "@/utils/base64";

/**
 * Represents browser support for passkeys
 */
export interface PasskeySupport {
  /** Whether WebAuthn API is available */
  available: boolean;
  /** Whether platform authenticator (Face ID, Touch ID, etc.) is available */
  platformAuthenticator: boolean;
}

/**
 * Check browser support for passkeys
 * @returns Promise resolving to PasskeySupport object
 */
export async function checkPasskeySupport(): Promise<PasskeySupport> {
  const available = window.PublicKeyCredential !== undefined;

  if (!available) {
    return { available: false, platformAuthenticator: false };
  }

  try {
    const platformAuthenticator =
      await PublicKeyCredential.isUserVerifyingPlatformAuthenticatorAvailable();

    return { available: true, platformAuthenticator };
  } catch (error) {
    console.error("Error checking passkey support:", error);
    return { available: true, platformAuthenticator: false };
  }
}

/**
 * Register a new passkey with the authenticator
 * @param creationOptions - WebAuthn creation options from the server
 * @returns Promise resolving to the created credential
 */
export async function registerPasskey(
  creationOptions: Record<string, unknown>,
): Promise<PublicKeyCredential> {
  // Convert base64url strings to ArrayBuffers
  const publicKeyOptions = creationOptions.publicKey as Record<string, unknown>;
  const challenge = base64URLStringToBuffer(publicKeyOptions.challenge as string);
  const user = publicKeyOptions.user as Record<string, unknown>;
  const userId = base64URLStringToBuffer(user.id as string);

  const options: PublicKeyCredentialCreationOptions = {
    ...publicKeyOptions,
    challenge,
    user: {
      ...user,
      id: userId,
    } as PublicKeyCredentialUserEntity,
  } as PublicKeyCredentialCreationOptions;

  const credential = (await navigator.credentials.create({
    publicKey: options,
  })) as PublicKeyCredential;

  if (!credential) {
    throw new Error("Failed to create passkey");
  }

  return credential;
}

/**
 * Authenticate with a passkey
 * @param requestOptions - WebAuthn authentication options from the server
 * @returns Promise resolving to the authentication credential
 */
export async function authenticateWithPasskey(
  requestOptions: Record<string, unknown>,
): Promise<PublicKeyCredential> {
  // Convert base64url strings to ArrayBuffers
  const publicKeyOptions = requestOptions.publicKey as Record<string, unknown>;
  const challenge = base64URLStringToBuffer(publicKeyOptions.challenge as string);

  // Convert allowCredentials if present
  let allowCredentials;
  if (publicKeyOptions.allowCredentials) {
    allowCredentials = (publicKeyOptions.allowCredentials as Array<Record<string, unknown>>).map(
      (cred) => ({
        ...cred,
        id: base64URLStringToBuffer(cred.id as string),
      }),
    );
  }

  const options: PublicKeyCredentialRequestOptions = {
    ...publicKeyOptions,
    challenge,
    ...(allowCredentials && { allowCredentials }),
  } as PublicKeyCredentialRequestOptions;

  const credential = (await navigator.credentials.get({
    publicKey: options,
  })) as PublicKeyCredential;

  if (!credential) {
    throw new Error("Failed to authenticate with passkey");
  }

  return credential;
}

/**
 * Convert a credential registration response to JSON format for the server
 * @param credential - The PublicKeyCredential from registration
 * @returns JSON-serializable credential object
 */
export function credentialToJSON(credential: PublicKeyCredential): Record<string, unknown> {
  const response = credential.response as AuthenticatorAttestationResponse;

  return {
    id: credential.id,
    rawId: bufferToBase64URLString(credential.rawId),
    type: credential.type,
    response: {
      clientDataJSON: bufferToBase64URLString(response.clientDataJSON),
      attestationObject: bufferToBase64URLString(response.attestationObject),
    },
  };
}

/**
 * Convert a credential assertion response to JSON format for the server
 * @param credential - The PublicKeyCredential from authentication
 * @returns JSON-serializable credential object
 */
export function assertionToJSON(credential: PublicKeyCredential): Record<string, unknown> {
  const response = credential.response as AuthenticatorAssertionResponse;

  return {
    id: credential.id,
    rawId: bufferToBase64URLString(credential.rawId),
    type: credential.type,
    response: {
      clientDataJSON: bufferToBase64URLString(response.clientDataJSON),
      authenticatorData: bufferToBase64URLString(response.authenticatorData),
      signature: bufferToBase64URLString(response.signature),
      userHandle: response.userHandle
        ? bufferToBase64URLString(response.userHandle)
        : null,
    },
  };
}
