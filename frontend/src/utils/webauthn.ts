import { startRegistration, startAuthentication } from '@simplewebauthn/browser';
import type { 
    PublicKeyCredentialCreationOptionsJSON, 
    PublicKeyCredentialRequestOptionsJSON,
    RegistrationResponseJSON,
    AuthenticationResponseJSON
} from '@simplewebauthn/browser';

/**
 * Check if the browser supports WebAuthn
 */
export function isWebAuthnSupported(): boolean {
    return (
        window?.PublicKeyCredential !== undefined &&
        typeof window.PublicKeyCredential === 'function'
    );
}

/**
 * Start passkey registration
 */
export async function registerPasskey(
    challenge: PublicKeyCredentialCreationOptionsJSON
): Promise<RegistrationResponseJSON> {
    if (!isWebAuthnSupported()) {
        throw new Error('WebAuthn is not supported in this browser');
    }

    return await startRegistration({ optionsJSON: challenge });
}

/**
 * Start passkey authentication
 */
export async function authenticatePasskey(
    challenge: PublicKeyCredentialRequestOptionsJSON
): Promise<AuthenticationResponseJSON> {
    if (!isWebAuthnSupported()) {
        throw new Error('WebAuthn is not supported in this browser');
    }

    return await startAuthentication({ optionsJSON: challenge });
}
