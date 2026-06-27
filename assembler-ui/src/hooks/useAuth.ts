import { useState, useEffect, useCallback } from 'react';
import { authService, type User } from '../lib/auth';

function parseAuthError(err: any): string {
  // Axios-style response
  const backendMsg: string | undefined =
    err?.response?.data?.error ||
    err?.response?.data?.message ||
    err?.response?.data;

  // Fetch-style 
  const fetchMsg: string | undefined =
    err?.body?.error ||
    err?.body?.message ||
    err?.data?.error ||
    err?.data?.message;

  const raw = (backendMsg || fetchMsg || err?.message || '').toString().toLowerCase();

  if (!raw || raw === 'error') return 'LOGIN FAILED — PLEASE TRY AGAIN';

  if (raw.includes('not found') || raw.includes('no user') || raw.includes('no account'))
    return 'NO ACCOUNT FOUND WITH THAT EMAIL';

  if (
    raw.includes('invalid credentials') ||
    raw.includes('wrong password') ||
    raw.includes('incorrect password') ||
    raw.includes('invalid password') ||
    raw.includes('password') ||
    raw.includes('credentials') ||
    raw.includes('401') ||
    raw.includes('400')
  )
    return 'INCORRECT EMAIL OR PASSWORD';

  if (raw.includes('429') || raw.includes('rate limit') || raw.includes('too many'))
    return 'TOO MANY ATTEMPTS — TRY AGAIN LATER';

  if (raw.includes('network') || raw.includes('fetch') || raw.includes('failed to fetch') || raw.includes('econnrefused'))
    return 'CONNECTION FAILED — CHECK YOUR NETWORK';

  if (raw.includes('email') && (raw.includes('taken') || raw.includes('exists') || raw.includes('already')))
    return 'EMAIL ALREADY IN USE';

  // return the raw backend message uppercased as a last resort 
  return (backendMsg || fetchMsg || err?.message || 'LOGIN FAILED — PLEASE TRY AGAIN').toUpperCase();
}

function parseRegisterError(err: any): string {
  const msg = parseAuthError(err);
  // re-map login-specific fallbacks to register context
  if (msg === 'LOGIN FAILED — PLEASE TRY AGAIN') return 'REGISTRATION FAILED — PLEASE TRY AGAIN';
  if (msg === 'INCORRECT EMAIL OR PASSWORD') return 'INVALID DETAILS — CHECK YOUR INPUT';
  return msg;
}

export function useAuth() {
  const [user, setUser] = useState<User | null>(null);
  const [isLoggedIn, setIsLoggedIn] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [isInitialized, setIsInitialized] = useState(false);

  useEffect(() => {
    try {
      const authenticated = authService.isAuthenticated();
      setIsLoggedIn(authenticated);
      if (authenticated) setUser(authService.getCurrentUser());
    } catch (err) {
      console.error('Auth check failed:', err);
      setIsLoggedIn(false);
    } finally {
      setIsInitialized(true);
    }
  }, []);

  const login = useCallback(async (email: string, password: string) => {
    setError(null);
    try {
      const result = await authService.login({ email, password });
      setUser(result.user);
      setIsLoggedIn(true);
      return { success: true };
    } catch (err: any) {
      const errorMsg = parseAuthError(err);
      setError(errorMsg);
      return { success: false, error: errorMsg };
    }
  }, []);

  const register = useCallback(async (email: string, password: string, name: string) => {
    setError(null);
    try {
      const result = await authService.register({ email, password, name });
      setUser(result.user);
      setIsLoggedIn(true);
      return { success: true };
    } catch (err: any) {
      const errorMsg = parseRegisterError(err);
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

  const clearError = useCallback(() => setError(null), []);

  return { user, isLoggedIn, error, login, register, logout, clearError, isInitialized };
}