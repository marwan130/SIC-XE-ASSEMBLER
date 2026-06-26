import { useState } from 'react';
import type { FormEvent } from 'react';
import { UserPlus, ArrowLeft } from 'lucide-react';

interface RegisterPageProps {
  onRegister: (email: string, password: string) => Promise<{ success: boolean; error?: string }>;
  onSwitchToLogin: () => void;
  errorMsg: string | null;
  clearError: () => void;
  onClose?: () => void;
}

export default function RegisterPage({
  onRegister,
  onSwitchToLogin,
  errorMsg,
  clearError,
  onClose,
}: RegisterPageProps) {
  const [email, setEmail] = useState('');
  const [password, setPassword] = useState('');
  const [confirmPassword, setConfirmPassword] = useState('');
  const [formError, setFormError] = useState<string | null>(null);
  const [submitting, setSubmitting] = useState(false);

  const handleSubmit = async (e: FormEvent) => {
    e.preventDefault();
    clearError();
    setFormError(null);

    if (!email.trim() || !password.trim() || !confirmPassword.trim()) {
      setFormError('ALL ENCRYPT CHANNELS REQUIRED');
      return;
    }

    if (password !== confirmPassword) {
      setFormError('PASSPHRASE MISMATCH ERROR DETECTED');
      return;
    }

    setSubmitting(true);
    const result = await onRegister(email, password);
    setSubmitting(false);

    if (!result.success && result.error) {
      setFormError(result.error);
    }
  };

  const handleOAuthRedirect = (provider: 'google' | 'github') => {
    clearError();
    setFormError(null);
    window.location.href = `${import.meta.env.VITE_API_URL || 'http://127.0.0.1:8080'}/auth/${provider}`;
  };

  return (
    <div className="min-h-screen w-full flex items-center justify-center p-4 relative select-none">
      <div className="w-full max-w-md bg-cyber-panel p-8 shadow-[6px_6px_0px_#000] relative border-4 border-black">
        {/* Close button */}
        {onClose && (
          <button
            onClick={onClose}
            className="absolute top-2 right-2 text-gray-500 hover:text-white font-mono text-[12px] border border-gray-700 px-2 py-1 hover:bg-gray-800 transition-colors"
          >
            ✕
          </button>
        )}

        {/* Back Link */}
        <button
          onClick={onSwitchToLogin}
          className="absolute top-4 left-4 flex items-center gap-1 font-mono text-[9px] text-gray-500 hover:text-white transition-colors cursor-pointer uppercase font-bold"
        >
          <ArrowLeft className="w-3.5 h-3.5" />
          <span>BACK</span>
        </button>

        {/* Cute CSS Pixel Robot Header */}
        <div className="flex flex-col items-center gap-2 mb-6 mt-2">
          <div className="pixel-astronaut mb-2 animate-bounce">
            <div className="astro-visor" />
            <div className="astro-light" />
            <div className="astro-pack" />
          </div>
          <h1 className="font-press text-[18px] text-neon-green text-center uppercase tracking-tighter drop-shadow-[0_0_4px_#00FF66] font-bold">
            CREATE ACCOUNT
          </h1>
        </div>

        {/* Input Form */}
        <form onSubmit={handleSubmit} className="flex flex-col gap-4">
          {/* Email */}
          <div className="flex flex-col gap-1">
            <label className="font-press text-[11px] sm:text-[12px] text-neon-green uppercase tracking-wider pl-1">
              Email
            </label>
            <input
              type="email"
              value={email}
              onChange={(e) => setEmail(e.target.value)}
              placeholder="USER@CORE.OS"
              className="w-full bg-[#0a0a0f] border-3 border-black p-2.5 font-mono text-[13px] text-white focus:outline-none focus:ring-0 focus:border-neon-green focus:shadow-[0_0_8px_#00FF66] placeholder:text-gray-700"
            />
          </div>

          {/* Passwords layout grid alignment matrix */}
          <div className="grid grid-cols-2 gap-3 sm:gap-4 items-end">
            {/* Password */}
            <div className="flex flex-col gap-1">
              <label className="font-press text-[12px] sm:text-[12px] text-neon-green uppercase tracking-wider pl-1 truncate">
                Password
              </label>
              <input
                type="password"
                value={password}
                onChange={(e) => setPassword(e.target.value)}
                placeholder="********"
                className="w-full bg-[#0a0a0f] border-3 border-black p-2.5 font-mono text-[13px] text-white focus:outline-none focus:ring-0 focus:border-neon-green focus:shadow-[0_0_8px_#00FF66] placeholder:text-gray-700"
              />
            </div>

            {/* Confirm Password (perfectly balanced size configuration matching layout logic) */}
            <div className="flex flex-col gap-1">
              <label className="font-press text-[12px] sm:text-[12px] text-neon-green uppercase tracking-wider pl-1 truncate">
                Re-Entry
              </label>
              <input
                type="password"
                value={confirmPassword}
                onChange={(e) => setConfirmPassword(e.target.value)}
                placeholder="********"
                className="w-full bg-[#0a0a0f] border-3 border-black p-2.5 font-mono text-[13px] text-white focus:outline-none focus:ring-0 focus:border-neon-green focus:shadow-[0_0_8px_#00FF66] placeholder:text-gray-700"
              />
            </div>
          </div>

          {/* Form Error or Banner */}
          {(formError || errorMsg) && (
            <div className="bg-[#1A0505] border-2 border-[#FF4444] p-3 text-center mt-1">
              <span className="font-mono text-[11px] text-[#FF9999] uppercase leading-tight block">
                {formError || errorMsg}
              </span>
            </div>
          )}

          {/* Submit Button */}
          <button
            type="submit"
            disabled={submitting}
            className="bg-neon-green text-black font-press text-[12px] font-bold py-3.5 border-3 border-black shadow-[4px_4px_0px_#000] active:translate-x-[4px] active:translate-y-[4px] active:shadow-none transition-all cursor-pointer uppercase flex items-center justify-center gap-2 mt-2"
          >
            <UserPlus className="w-4 h-4" />
            <span>{submitting ? 'SYNCHRONIZING...' : 'JOIN SYSTEM'}</span>
          </button>
        </form>

        {/* OR Divider */}
        <div className="flex items-center gap-3 my-5">
          <div className="flex-1 h-0.5 bg-black" />
          <span className="font-press text-[10px] text-gray-500 uppercase tracking-widest">— OR —</span>
          <div className="flex-1 h-0.5 bg-black" />
        </div>

        {/* OAuth Row */}
        <div className="w-full flex flex-col gap-3">
          <div className="grid grid-cols-2 gap-4">
            <button
              onClick={() => handleOAuthRedirect('github')}
              disabled={submitting}
              className="bg-[#24292F] hover:bg-[#343B45] text-white font-press text-[12px] sm:text-[12px] font-bold p-3 border-3 border-black shadow-[4px_4px_0px_#000] active:translate-x-[4px] active:translate-y-[4px] active:shadow-none transition-all cursor-pointer uppercase flex items-center justify-center gap-1"
            >
              GITHUB
            </button>
            <button
              onClick={() => handleOAuthRedirect('google')}
              disabled={submitting}
              className="bg-[#EA4335] hover:bg-[#FF5C4D] text-white font-press text-[12px] sm:text-[12px] font-bold p-3 border-3 border-black shadow-[4px_4px_0px_#000] active:translate-x-[4px] active:translate-y-[4px] active:shadow-none transition-all cursor-pointer uppercase flex items-center justify-center gap-1"
            >
              GOOGLE
            </button>
          </div>
        </div>

        {/* Link back to Login */}
        <div className="mt-6 text-center flex flex-col items-center gap-1">
          <span className="font-mono text-[10px] text-gray-500 uppercase">ALREADY_SYNCHRONIZED?</span>
          <button
            onClick={onSwitchToLogin}
            className="font-press text-[12px] text-cyber-yellow hover:text-white underline underline-offset-4 decoration-2 transition-colors uppercase cursor-pointer font-bold"
          >
            LOGIN
          </button>
        </div>
      </div>

      {/* Embedded Astronaut Asset Head Style Rules */}
      <style>{`
        .pixel-astronaut {
          width: 32px;
          height: 30px;
          background-color: #E2E8F0;
          border: 3px solid #000;
          box-shadow: 
            inset -3px -3px 0 #CBD5E1,
            2px 2px 0 #000;
          position: relative;
        }

        .astro-visor {
          position: absolute;
          width: 20px;
          height: 12px;
          background-color: #0F172A;
          border: 2px solid #000;
          top: 5px;
          left: 6px;
          box-shadow: inset -2px -2px 0 #1E293B, inset 2px 2px 0 #38BDF8;
        }

        .astro-light {
          position: absolute;
          width: 4px;
          height: 4px;
          background-color: #FF007A;
          border: 1px solid #000;
          top: -6px;
          left: 14px;
        }

        .astro-pack {
          position: absolute;
          width: 6px;
          height: 18px;
          background-color: #94A3B8;
          border: 2.5px solid #000;
          top: 6px;
          left: -7px;
        }
      `}</style>
    </div>
  );
}