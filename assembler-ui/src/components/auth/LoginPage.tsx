import { useState } from 'react';
import type { FormEvent } from 'react';
import { LogIn } from 'lucide-react';

interface LoginPageProps {
  onLogin: (email: string, password: string) => Promise<{ success: boolean; error?: string }>;
  onSwitchToRegister: () => void;
  errorMsg: string | null;
  clearError: () => void;
  onClose?: () => void;
}

function isValidEmail(val: string) {
  return /^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(val.trim());
}

export default function LoginPage({
  onLogin,
  onSwitchToRegister,
  errorMsg,
  clearError,
  onClose,
}: LoginPageProps) {
  const [email, setEmail] = useState('');
  const [password, setPassword] = useState('');
  const [showPassword, setShowPassword] = useState(false);
  const [emailError, setEmailError] = useState<string | null>(null);
  const [formError, setFormError] = useState<string | null>(null);
  const [submitting, setSubmitting] = useState(false);

  // Validate email on blur only 
  const handleEmailBlur = () => {
    if (email && !isValidEmail(email)) {
      setEmailError('INVALID EMAIL FORMAT');
    } else {
      setEmailError(null);
    }
  };

  const handleEmailChange = (val: string) => {
    setEmail(val);
    // Clear the inline error as soon as they start fixing it
    if (emailError && isValidEmail(val)) setEmailError(null);
  };

  const handleSubmit = async (e: FormEvent) => {
    e.preventDefault();
    clearError();
    setFormError(null);

    // Client-side validation
    if (!email.trim()) {
      setEmailError('EMAIL IS REQUIRED');
      return;
    }
    if (!isValidEmail(email)) {
      setEmailError('INVALID EMAIL FORMAT');
      return;
    }
    if (!password.trim()) {
      setFormError('PASSPHRASE IS REQUIRED');
      return;
    }

    setSubmitting(true);
    try {
      const result = await onLogin(email, password);
      if (!result.success) {
        // Map backend errors 
        const raw = (result.error || '').toLowerCase();
        if (raw.includes('400') || raw.includes('invalid') || raw.includes('credentials') || raw.includes('password') || raw.includes('incorrect')) {
          setFormError('INCORRECT EMAIL OR PASSWORD');
        } else if (raw.includes('not found') || raw.includes('no user') || raw.includes('404')) {
          setFormError('NO ACCOUNT FOUND WITH THAT EMAIL');
        } else if (raw.includes('429') || raw.includes('rate')) {
          setFormError('TOO MANY ATTEMPTS — TRY AGAIN LATER');
        } else if (raw.includes('network') || raw.includes('fetch') || raw.includes('failed')) {
          setFormError('CONNECTION FAILED — CHECK YOUR NETWORK');
        } else if (result.error) {
          setFormError(result.error.toUpperCase());
        } else {
          setFormError('LOGIN FAILED — PLEASE TRY AGAIN');
        }
      }
    } catch {
      setFormError('CONNECTION FAILED — CHECK YOUR NETWORK');
    } finally {
      setSubmitting(false);
    }
  };

  const handleOAuthRedirect = (provider: 'google' | 'github') => {
    clearError();
    setFormError(null);
    window.location.href = `${import.meta.env.VITE_API_URL || 'http://127.0.0.1:8080'}/auth/${provider}`;
  };

  const displayError = formError || errorMsg;

  return (
    <div className="min-h-screen w-full flex items-start justify-center p-4 pt-8 pb-20 relative select-none overflow-y-auto">
      <div className="w-full max-w-md bg-[#121212] p-8 shadow-[6px_6px_0px_#000] relative">
        {onClose && (
          <button
            onClick={onClose}
            className="absolute top-2 right-2 text-gray-500 hover:text-white font-mono text-[12px] border border-gray-700 px-2 py-1 hover:bg-gray-800 transition-colors"
          >
            ✕
          </button>
        )}

        {/* Header */}
        <div className="flex flex-col items-center gap-2 mb-6">
          <div className="pixel-robot mb-2 animate-bounce">
            <div className="robot-eyes">
              <div className="robot-eye" />
              <div className="robot-eye" />
            </div>
            <div className="robot-antenna" />
            <div className="robot-mouth" />
          </div>
          <h1 className="font-press text-[18px] text-[#00FF66] text-center uppercase tracking-tighter drop-shadow-[0_0_4px_#00FF66] font-bold">
            SIGN IN
          </h1>
        </div>

        <form onSubmit={handleSubmit} className="flex flex-col gap-5" noValidate>
          {/* Email */}
          <div className="flex flex-col gap-1.5">
            <label className="font-press text-[12px] text-neon-green uppercase tracking-wider pl-1">
              EMAIL
            </label>
            <input
              type="email"
              value={email}
              onChange={(e) => handleEmailChange(e.target.value)}
              onBlur={handleEmailBlur}
              placeholder="user@core.os"
              className={`w-full bg-[#0a0a0f] border-3 p-3 pl-4 font-mono text-[12px] text-white focus:outline-none focus:ring-0 placeholder:text-gray-700 transition-colors ${
                emailError
                  ? 'border-[#FF4444] shadow-[0_0_8px_#FF4444]'
                  : 'border-black focus:border-neon-green focus:shadow-[0_0_8px_#00FF66]'
              }`}
            />
            {emailError && (
              <span className="font-mono text-[10px] text-[#FF9999] uppercase pl-1">{emailError}</span>
            )}
          </div>

          {/* Password */}
          <div className="flex flex-col gap-1.5">
            <label className="font-press text-[12px] text-neon-green uppercase tracking-wider pl-1">
              PASSWORD
            </label>
            <div className="relative">
              <input
                type={showPassword ? 'text' : 'password'}
                value={password}
                onChange={(e) => { setPassword(e.target.value); if (formError) setFormError(null); }}
                placeholder="••••••••"
                className={`w-full bg-[#0a0a0f] border-3 p-3 pl-4 pr-12 font-mono text-[12px] text-white focus:outline-none focus:ring-0 placeholder:text-gray-700 transition-colors ${
                  displayError && !emailError
                    ? 'border-[#FF4444] shadow-[0_0_8px_#FF4444]'
                    : 'border-black focus:border-neon-green focus:shadow-[0_0_8px_#00FF66]'
                }`}
              />
              <button
                type="button"
                onClick={() => setShowPassword(!showPassword)}
                className="absolute right-3 top-1/2 -translate-y-1/2 text-gray-500 hover:text-white transition-colors"
              >
                {showPassword ? '👁️' : '👁️‍🗨️'}
              </button>
            </div>
          </div>

          {/* General error banner */}
          {displayError && (
            <div className="bg-[#1A0505] border-2 border-[#FF4444] p-3 text-center">
              <span className="font-mono text-[10px] text-[#FF9999] uppercase leading-tight block">
                {displayError}
              </span>
            </div>
          )}

          {/* Submit */}
          <button
            type="submit"
            disabled={submitting}
            className="bg-neon-green text-black font-press text-[12px] font-bold py-4 border-3 border-black shadow-[4px_4px_0px_#000] active:translate-x-[4px] active:translate-y-[4px] active:shadow-none transition-all cursor-pointer uppercase flex items-center justify-center gap-2 disabled:opacity-60"
          >
            <LogIn className="w-4 h-4" />
            <span>{submitting ? 'CONNECTING...' : 'LOGIN'}</span>
          </button>
        </form>

        {/* Divider */}
        <div className="flex items-center gap-3 my-6">
          <div className="flex-1 h-0.5 bg-black" />
          <span className="font-press text-[10px] text-gray-500 uppercase tracking-widest">— OR —</span>
          <div className="flex-1 h-0.5 bg-black" />
        </div>

        {/* OAuth */}
        <div className="grid grid-cols-2 gap-4">
          <button
            onClick={() => handleOAuthRedirect('google')}
            disabled={submitting}
            className="bg-[#EA4335] hover:bg-[#FF5C4D] text-white font-press text-[12px] font-bold p-3 border-3 border-black shadow-[4px_4px_0px_#000] active:translate-x-[4px] active:translate-y-[4px] active:shadow-none transition-all cursor-pointer uppercase flex items-center justify-center gap-1"
          >
            GOOGLE
          </button>
          <button
            onClick={() => handleOAuthRedirect('github')}
            disabled={submitting}
            className="bg-[#24292F] hover:bg-[#343B45] text-white font-press text-[12px] font-bold p-3 border-3 border-black shadow-[4px_4px_0px_#000] active:translate-x-[4px] active:translate-y-[4px] active:shadow-none transition-all cursor-pointer uppercase flex items-center justify-center gap-1"
          >
            GITHUB
          </button>
        </div>

        {/* Register link */}
        <div className="mt-8 text-center flex flex-col items-center gap-1">
          <span className="font-mono text-[10px] text-gray-500 uppercase">NEW_OPERATOR?</span>
          <button
            onClick={onSwitchToRegister}
            className="font-press text-[12px] text-cyber-yellow hover:text-white underline underline-offset-4 decoration-2 transition-colors uppercase cursor-pointer font-bold"
          >
            CREATE_ACCOUNT
          </button>
        </div>
      </div>

      <style>{`
        .pixel-robot {
          width: 32px;
          height: 28px;
          background-color: #00FF66;
          border: 3px solid #000;
          box-shadow: inset -3px -3px 0 #00AA44, inset 3px 3px 0 #D0FFE0, 2px 2px 0 #000;
          position: relative;
        }
        .robot-eyes {
          position: absolute;
          top: 6px; left: 4px; right: 4px;
          height: 6px;
          display: flex;
          justify-content: space-between;
        }
        .robot-eye {
          width: 6px; height: 6px;
          background-color: #00E0FF;
          border: 1.5px solid #000;
        }
        .robot-antenna {
          position: absolute;
          width: 4px; height: 6px;
          background-color: #000;
          top: -9px; left: 11px;
        }
        .robot-antenna::before {
          content: '';
          position: absolute;
          width: 8px; height: 4px;
          background-color: #FF007A;
          border: 2px solid #000;
          top: -5px; left: -4px;
        }
        .robot-mouth {
          position: absolute;
          width: 12px; height: 3px;
          background-color: #000;
          bottom: 5px; left: 7px;
        }
      `}</style>
    </div>
  );
}