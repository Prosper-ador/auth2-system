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

export const decodeUserFromToken = (token: string): User | null => {
  try {
    const payload = JSON.parse(atob(token.split('.')[1]));
    return {
      id: payload.sub || '',
      email: payload.email || '',
      first_name: payload.first_name || '',
      last_name: payload.last_name || '',
      role: payload.role || '',
      password: '', // not present in token
    };
  } catch {
    return null;
  }
};
