import { createContext, useContext, useState, useEffect, ReactNode, useCallback } from 'react';
import { useToast } from '@/hooks/use-toast';
import { AuthApi, Configuration, User, LoginRequest, RegisterRequest } from '../../ts-client/api';
import { getToken, clearAuthData, isTokenExpired, decodeUserFromToken, TOKEN_KEY } from './authUtils';
import { useNavigate } from "react-router-dom";

interface AuthContextType {
  user: User | null;
  login: (credentials: LoginRequest) => Promise<void>;
  register: (userData: RegisterRequest) => Promise<void>;
  logout: () => void;
  isLoading: boolean;
}

const AuthContext = createContext<AuthContextType | undefined>(undefined);

const api = new AuthApi(new Configuration({ 
  basePath: import.meta.env.VITE_API_BASE_URL || 'http://localhost:3000' 
}));

export const AuthProvider = ({ children }: { children: ReactNode }) => {
  const [user, setUser] = useState<User | null>(null);
  const [isLoading, setIsLoading] = useState(true);
  const { toast } = useToast();
  const navigate = useNavigate();

  useEffect(() => {
    const token = getToken();
    if (token && !isTokenExpired(token)) {
      setUser(decodeUserFromToken(token));
    } else {
      clearAuthData();
    }
    setIsLoading(false);
  }, []);

  const logout = useCallback(() => {
    clearAuthData();
    setUser(null);
    toast({
      title: "Logged Out",
      description: "You have been successfully logged out."
    });
    navigate("/login");
  }, [toast, navigate]);

  useEffect(() => {
    if (!user) return;
    const checkTokenExpiry = () => {
      const token = getToken();
      if (token && isTokenExpired(token)) {
        logout();
        toast({
          title: "Session Expired",
          description: "Your session has expired. Please log in again.",
          variant: "destructive"
        });
      }
    };
    const interval = setInterval(checkTokenExpiry, 60000);
    checkTokenExpiry();
    return () => clearInterval(interval);
  }, [user, toast, logout]);

  const login = async (credentials: LoginRequest) => {
    setIsLoading(true);
    try {
      const response = await api.login(credentials);
      localStorage.setItem(TOKEN_KEY, response.data.access_token);
      setUser(decodeUserFromToken(response.data.access_token));
      toast({
        title: "Welcome back!",
        description: "You have been successfully logged in."
      });
      console.log("Navigating to /profile");
      navigate("/profile");
    } catch (error) {
      toast({
        title: "Login Failed",
        description: error instanceof Error ? error.message : "An error occurred during login",
        variant: "destructive"
      });
      throw error;
    } finally {
      setIsLoading(false);
      console.log('login finally');
    }
  };

  const register = async (userData: RegisterRequest) => {
    setIsLoading(true);
    try {
      const response = await api.register(userData);
      localStorage.setItem(TOKEN_KEY, response.data.access_token);
      setUser(decodeUserFromToken(response.data.access_token));
      toast({
        title: "Account Created",
        description: "Your account has been created successfully!"
      });
      navigate("/profile");
    } catch (error) {
      toast({
        title: "Registration Failed",
        description: error instanceof Error ? error.message : "An error occurred during registration",
        variant: "destructive"
      });
      throw error;
    } finally {
      setIsLoading(false);
    }
  };

  return (
    <AuthContext.Provider value={{ user, login, register, logout, isLoading }}>
      {children}
    </AuthContext.Provider>
  );
};

// eslint-disable-next-line react-refresh/only-export-components
export const useAuth = () => {
  const context = useContext(AuthContext);
  if (context === undefined) {
    throw new Error('useAuth must be used within an AuthProvider');
  }
  return context;
};
