import { useState, useEffect, useCallback } from 'react';
import { authService, type User } from '../lib/auth';

export function useAuth() {
  const [user, setUser] = useState<User | null>(null);
  const [isLoggedIn, setIsLoggedIn] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [isInitialized, setIsInitialized] = useState(false);

  // Check authentication status on mount
  useEffect(() => {
    const checkAuth = () => {
      try {
        const authenticated = authService.isAuthenticated();
        setIsLoggedIn(authenticated);
        if (authenticated) {
          setUser(authService.getCurrentUser());
        }
      } catch (err) {
        console.error('Auth check failed:', err);
        setIsLoggedIn(false);
      } finally {
        setIsInitialized(true);
      }
    };
    checkAuth();
  }, []);

  const login = useCallback(async (email: string, password: string) => {
    setError(null);
    try {
      const result = await authService.login({ email, password });
      setUser(result.user);
      setIsLoggedIn(true);
      return { success: true };
    } catch (err: any) {
      const errorMsg = err.response?.data?.error || 'AUTHENTICATION_FAILED';
      setError(errorMsg);
      return { success: false, error: errorMsg };
    }
  }, []);

  const register = useCallback(async (email: string, password: string) => {
    setError(null);
    try {
      const result = await authService.register({ email, password });
      setUser(result.user);
      setIsLoggedIn(true);
      return { success: true };
    } catch (err: any) {
      const errorMsg = err.response?.data?.error || 'REGISTRATION_FAILED';
      setError(errorMsg);
      return { success: false, error: errorMsg };
    }
  }, []);

  const logout = useCallback(() => {
    authService.logout();
    setUser(null);
    setIsLoggedIn(false);
    setError(null);
  }, []);

  const clearError = useCallback(() => {
    setError(null);
  }, []);

  return {
    user,
    isLoggedIn,
    error,
    login,
    register,
    logout,
    clearError,
    isInitialized,
  };
}