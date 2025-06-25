import { User } from '../../../ts-client/api';

export const TOKEN_KEY = 'auth_token';

export const getToken = (): string | null => {
  return localStorage.getItem(TOKEN_KEY);
};

export const clearAuthData = () => {
  localStorage.removeItem(TOKEN_KEY);
};

export const isTokenExpired = (token: string): boolean => {
  if (!token) return true;
  try {
    const payload = JSON.parse(atob(token.split('.')[1]));
    const currentTime = Date.now() / 1000;
    return payload.exp < currentTime;
  } catch {
    return true;
  }
};

export function decodeUserFromToken(token: string) {
  try {
    const payload = JSON.parse(atob(token.split('.')[1]));
    return {
      id: "", // id is not included in the JWT, so we return an empty string
      email: payload.email || "",
      first_name: payload.first_name || "",
      last_name: payload.last_name || "",
      role: payload.role || "",
      password: "", // password is not included in the JWT, so we return an empty string
      // id and password are NOT in the JWT, so don't include them
    };
  } catch {
    return null;
  }
}
