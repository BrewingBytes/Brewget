import { jwtDecode } from "jwt-decode";

interface JWTPayload {
  sub: string;
  iat: number;
  exp: number;
}

export function getUserIdFromToken(token: string): string | null {
  try {
    const decoded = jwtDecode<JWTPayload>(token);
    return decoded.sub;
  } catch (error) {
    console.error("Failed to decode token:", error);
    return null;
  }
}
