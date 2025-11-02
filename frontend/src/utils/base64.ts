/**
 * Convert base64url string to ArrayBuffer
 * @param base64URLString - The base64url encoded string
 * @returns ArrayBuffer containing the decoded bytes
 */
export function base64URLStringToBuffer(base64URLString: string): ArrayBuffer {
  // Convert base64url to base64
  const base64 = base64URLString.replace(/-/g, "+").replace(/_/g, "/");

  // Decode base64
  const binary = atob(base64);
  const bytes = new Uint8Array(binary.length);

  for (let i = 0; i < binary.length; i++) {
    bytes[i] = binary.charCodeAt(i);
  }

  return bytes.buffer;
}

/**
 * Convert ArrayBuffer to base64url string
 * @param buffer - The ArrayBuffer to encode
 * @returns base64url encoded string
 */
export function bufferToBase64URLString(buffer: ArrayBuffer): string {
  const bytes = new Uint8Array(buffer);
  let binary = "";

  for (let i = 0; i < bytes.byteLength; i++) {
    // TypeScript safety: bytes[i] is guaranteed to exist for i < byteLength
    binary += String.fromCharCode(bytes[i]!);
  }

  // Convert to base64
  const base64 = btoa(binary);

  // Convert base64 to base64url
  return base64.replace(/\+/g, "-").replace(/\//g, "_").replace(/=/g, "");
}
