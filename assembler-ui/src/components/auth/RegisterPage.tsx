import { useState } from 'react';
import type { FormEvent } from 'react';
import { UserPlus, ArrowLeft } from 'lucide-react';

interface RegisterPageProps {
  onRegister: (email: string, password: string, name: string) => Promise<{ success: boolean; error?: string }>;
  onSwitchToLogin: () => void;
  errorMsg: string | null;
  clearError: () => void;
  onClose?: () => void;
}

function isValidEmail(val: string) {
  return /^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(val.trim());
}

export default function RegisterPage({
  onRegister,
  onSwitchToLogin,
  errorMsg,
  clearError,
  onClose,
}: RegisterPageProps) {
  const [name, setName] = useState('');
  const [email, setEmail] = useState('');
  const [password, setPassword] = useState('');
  const [confirmPassword, setConfirmPassword] = useState('');
  const [showPassword, setShowPassword] = useState(false);
  const [showConfirmPassword, setShowConfirmPassword] = useState(false);
  const [submitting, setSubmitting] = useState(false);

  // field errors
  const [nameError, setNameError] = useState<string | null>(null);
  const [emailError, setEmailError] = useState<string | null>(null);
  const [passwordError, setPasswordError] = useState<string | null>(null);
  const [confirmError, setConfirmError] = useState<string | null>(null);
  const [formError, setFormError] = useState<string | null>(null);

  // field validators 
  const validateName = (val: string) => {
    if (!val.trim()) return 'DISPLAY NAME IS REQUIRED';
    if (val.trim().length < 2) return 'NAME MUST BE AT LEAST 2 CHARACTERS';
    return null;
  };

  const validateEmail = (val: string) => {
    if (!val.trim()) return 'EMAIL IS REQUIRED';
    if (!isValidEmail(val)) return 'INVALID EMAIL FORMAT';
    return null;
  };

  const validatePassword = (val: string) => {
    if (!val) return 'PASSWORD IS REQUIRED';
    if (val.length < 8 || val.length > 12) return 'PASSWORD MUST BE 8–12 CHARACTERS';
    if (!/[a-zA-Z]/.test(val)) return 'PASSWORD MUST CONTAIN LETTERS';
    if (!/[0-9]/.test(val)) return 'PASSWORD MUST CONTAIN NUMBERS';
    return null;
  };

  const validateConfirm = (val: string, pw: string) => {
    if (!val) return 'PLEASE CONFIRM YOUR PASSWORD';
    if (val !== pw) return 'PASSWORDS DO NOT MATCH';
    return null;
  };

  // on change handlers
  const handleNameChange = (val: string) => {
    setName(val);
    if (nameError && !validateName(val)) setNameError(null);
  };
  const handleEmailChange = (val: string) => {
    setEmail(val);
    if (emailError && !validateEmail(val)) setEmailError(null);
  };
  const handlePasswordChange = (val: string) => {
    setPassword(val);
    if (passwordError && !validatePassword(val)) setPasswordError(null);
    if (confirmError && confirmPassword && val === confirmPassword) setConfirmError(null);
  };
  const handleConfirmChange = (val: string) => {
    setConfirmPassword(val);
    if (confirmError && val === password) setConfirmError(null);
  };

  const handleNameBlur   = () => setNameError(validateName(name));
  const handleEmailBlur  = () => setEmailError(validateEmail(email));
  const handlePasswordBlur = () => setPasswordError(validatePassword(password));
  const handleConfirmBlur  = () => setConfirmError(validateConfirm(confirmPassword, password));

  const handleSubmit = async (e: FormEvent) => {
    e.preventDefault();
    clearError();
    setFormError(null);

    // Run all validators at once so every field shows its error
    const ne = validateName(name);
    const ee = validateEmail(email);
    const pe = validatePassword(password);
    const ce = validateConfirm(confirmPassword, password);

    setNameError(ne);
    setEmailError(ee);
    setPasswordError(pe);
    setConfirmError(ce);

    if (ne || ee || pe || ce) return;

    setSubmitting(true);
    try {
      const result = await onRegister(email, password, name);
      if (!result.success && result.error) {
        const raw = result.error.toLowerCase();
        if (raw.includes('email') && (raw.includes('taken') || raw.includes('exists') || raw.includes('already'))) {
          setEmailError('EMAIL ALREADY IN USE');
        } else {
          setFormError(result.error);
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

  const fieldClass = (hasError: boolean) =>
    `w-full bg-[#0a0a0f] border-3 p-2.5 font-mono text-[12px] text-white focus:outline-none focus:ring-0 placeholder:text-gray-700 transition-colors ${
      hasError
        ? 'border-[#FF4444] shadow-[0_0_8px_#FF4444]'
        : 'border-black focus:border-neon-green focus:shadow-[0_0_8px_#00FF66]'
    }`;

  const fieldErr = (msg: string | null) =>
    msg ? <span className="font-mono text-[10px] text-[#FF9999] uppercase pl-1 mt-0.5">{msg}</span> : null;

  return (
    <div className="min-h-screen w-full flex items-start justify-center p-4 pt-8 pb-20 relative select-none overflow-y-auto">
      <div className="w-full max-w-md bg-cyber-panel p-8 shadow-[6px_6px_0px_#000] relative border-4 border-black">
        {onClose && (
          <button onClick={onClose} className="absolute top-2 right-2 text-gray-500 hover:text-white font-mono text-[12px] border border-gray-700 px-2 py-1 hover:bg-gray-800 transition-colors">
            ✕
          </button>
        )}

        <button onClick={onSwitchToLogin} className="absolute top-4 left-4 flex items-center gap-1 font-mono text-[10px] text-gray-500 hover:text-white transition-colors cursor-pointer uppercase font-bold">
          <ArrowLeft className="w-3.5 h-3.5" />
          <span>BACK</span>
        </button>

        {/* Header */}
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

        <form onSubmit={handleSubmit} className="flex flex-col gap-4" noValidate>
          {/* Name */}
          <div className="flex flex-col gap-1">
            <label className="font-press text-[12px] text-neon-green uppercase tracking-wider pl-1">Display Name</label>
            <input
              type="text"
              value={name}
              onChange={(e) => handleNameChange(e.target.value)}
              onBlur={handleNameBlur}
              placeholder="USER_NAME"
              className={fieldClass(!!nameError)}
            />
            {fieldErr(nameError)}
          </div>

          {/* Email */}
          <div className="flex flex-col gap-1">
            <label className="font-press text-[12px] text-neon-green uppercase tracking-wider pl-1">Email</label>
            <input
              type="email"
              value={email}
              onChange={(e) => handleEmailChange(e.target.value)}
              onBlur={handleEmailBlur}
              placeholder="USER@CORE.OS"
              className={fieldClass(!!emailError)}
            />
            {fieldErr(emailError)}
          </div>

          {/* Password + Confirm */}
          <div className="grid grid-cols-2 gap-3 sm:gap-4 items-start">
            <div className="flex flex-col gap-1">
              <label className="font-press text-[12px] text-neon-green uppercase tracking-wider pl-1 truncate">Password</label>
              <div className="relative">
                <input
                  type={showPassword ? 'text' : 'password'}
                  value={password}
                  onChange={(e) => handlePasswordChange(e.target.value)}
                  onBlur={handlePasswordBlur}
                  placeholder="********"
                  className={fieldClass(!!passwordError)}
                />
                <button
                  type="button"
                  onClick={() => setShowPassword(!showPassword)}
                  className="absolute right-3 top-1/2 -translate-y-1/2 text-gray-500 hover:text-white transition-colors"
                >
                  {showPassword ? '👁️' : '👁️‍🗨️'}
                </button>
              </div>
              {fieldErr(passwordError)}
            </div>
            <div className="flex flex-col gap-1">
              <label className="font-press text-[12px] text-neon-green uppercase tracking-wider pl-1 truncate">Re-Entry</label>
              <div className="relative">
                <input
                  type={showConfirmPassword ? 'text' : 'password'}
                  value={confirmPassword}
                  onChange={(e) => handleConfirmChange(e.target.value)}
                  onBlur={handleConfirmBlur}
                  placeholder="********"
                  className={fieldClass(!!confirmError)}
                />
                <button
                  type="button"
                  onClick={() => setShowConfirmPassword(!showConfirmPassword)}
                  className="absolute right-3 top-1/2 -translate-y-1/2 text-gray-500 hover:text-white transition-colors"
                >
                  {showConfirmPassword ? '👁️' : '👁️‍🗨️'}
                </button>
              </div>
              {fieldErr(confirmError)}
            </div>
          </div>

          {/* General form / backend error */}
          {(formError || errorMsg) && (
            <div className="bg-[#1A0505] border-2 border-[#FF4444] p-3 text-center mt-1">
              <span className="font-mono text-[10px] text-[#FF9999] uppercase leading-tight block">
                {formError || errorMsg}
              </span>
            </div>
          )}

          <button
            type="submit"
            disabled={submitting}
            className="bg-neon-green text-black font-press text-[12px] font-bold py-3.5 border-3 border-black shadow-[4px_4px_0px_#000] active:translate-x-[4px] active:translate-y-[4px] active:shadow-none transition-all cursor-pointer uppercase flex items-center justify-center gap-2 mt-2 disabled:opacity-60"
          >
            <UserPlus className="w-4 h-4" />
            <span>{submitting ? 'SYNCHRONIZING...' : 'JOIN SYSTEM'}</span>
          </button>
        </form>

        {/* Divider */}
        <div className="flex items-center gap-3 my-5">
          <div className="flex-1 h-0.5 bg-black" />
          <span className="font-press text-[10px] text-gray-500 uppercase tracking-widest">— OR —</span>
          <div className="flex-1 h-0.5 bg-black" />
        </div>

        {/* OAuth */}
        <div className="grid grid-cols-2 gap-4">
          <button onClick={() => handleOAuthRedirect('github')} disabled={submitting}
            className="bg-[#24292F] hover:bg-[#343B45] text-white font-press text-[12px] font-bold p-3 border-3 border-black shadow-[4px_4px_0px_#000] active:translate-x-[4px] active:translate-y-[4px] active:shadow-none transition-all cursor-pointer uppercase flex items-center justify-center gap-1">
            GITHUB
          </button>
          <button onClick={() => handleOAuthRedirect('google')} disabled={submitting}
            className="bg-[#EA4335] hover:bg-[#FF5C4D] text-white font-press text-[12px] font-bold p-3 border-3 border-black shadow-[4px_4px_0px_#000] active:translate-x-[4px] active:translate-y-[4px] active:shadow-none transition-all cursor-pointer uppercase flex items-center justify-center gap-1">
            GOOGLE
          </button>
        </div>

        <div className="mt-6 text-center flex flex-col items-center gap-1">
          <span className="font-mono text-[10px] text-gray-500 uppercase">ALREADY_SYNCHRONIZED?</span>
          <button onClick={onSwitchToLogin} className="font-press text-[12px] text-cyber-yellow hover:text-white underline underline-offset-4 decoration-2 transition-colors uppercase cursor-pointer font-bold">
            LOGIN
          </button>
        </div>
      </div>

      <style>{`
        .pixel-astronaut {
          width: 32px; height: 30px;
          background-color: #E2E8F0;
          border: 3px solid #000;
          box-shadow: inset -3px -3px 0 #CBD5E1, 2px 2px 0 #000;
          position: relative;
        }
        .astro-visor {
          position: absolute;
          width: 20px; height: 12px;
          background-color: #0F172A;
          border: 2px solid #000;
          top: 5px; left: 6px;
          box-shadow: inset -2px -2px 0 #1E293B, inset 2px 2px 0 #38BDF8;
        }
        .astro-light {
          position: absolute;
          width: 4px; height: 4px;
          background-color: #FF007A;
          border: 1px solid #000;
          top: -6px; left: 14px;
        }
        .astro-pack {
          position: absolute;
          width: 6px; height: 18px;
          background-color: #94A3B8;
          border: 2.5px solid #000;
          top: 6px; left: -7px;
        }
      `}</style>
    </div>
  );
}