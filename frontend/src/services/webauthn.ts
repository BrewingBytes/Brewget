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
  creationOptions: any
): Promise<PublicKeyCredential> {
  // Convert base64url strings to ArrayBuffers
  const challenge = base64URLStringToBuffer(creationOptions.publicKey.challenge);
  const userId = base64URLStringToBuffer(creationOptions.publicKey.user.id);

  const publicKeyOptions: PublicKeyCredentialCreationOptions = {
    ...creationOptions.publicKey,
    challenge,
    user: {
      ...creationOptions.publicKey.user,
      id: userId,
    },
  };

  const credential = (await navigator.credentials.create({
    publicKey: publicKeyOptions,
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
  requestOptions: any
): Promise<PublicKeyCredential> {
  // Convert base64url strings to ArrayBuffers
  const challenge = base64URLStringToBuffer(requestOptions.publicKey.challenge);

  const allowCredentials = requestOptions.publicKey.allowCredentials?.map(
    (cred: any) => ({
      ...cred,
      id: base64URLStringToBuffer(cred.id),
    })
  );

  const publicKeyOptions: PublicKeyCredentialRequestOptions = {
    ...requestOptions.publicKey,
    challenge,
    allowCredentials,
  };

  const credential = (await navigator.credentials.get({
    publicKey: publicKeyOptions,
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
export function credentialToJSON(credential: PublicKeyCredential): any {
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
export function assertionToJSON(credential: PublicKeyCredential): any {
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
