import { Lock, Palette, ShieldAlert, Trash2 } from 'lucide-react';
import type { User } from '../../lib/auth';
import type { TerminalTheme } from '../../types';

interface ProfilePageProps {
  user: User | null;
  theme: TerminalTheme;
  setTheme: (theme: TerminalTheme) => void;
  customCursor: boolean;
  setCustomCursor: (enable: boolean) => void;
  onWipeData: () => void;
  onDeleteAccount: () => void;
}

export default function ProfilePage({
  user,
  theme,
  setTheme,
  customCursor,
  setCustomCursor,
  onWipeData,
  onDeleteAccount,
}: ProfilePageProps) {

  if (!user) return null;

  const formatDate = (dateString: string) => {
    return new Date(dateString).toLocaleDateString('sv-SE').replace(/-/g, '.');
  };

  return (
    <div className="flex-1 p-6 overflow-y-auto select-none max-w-5xl mx-auto w-full h-full space-y-8">
      {/* Top Header */}
      <div className="border-b-3 border-black pb-4">
        <h2 className="font-press text-[18px] text-cyber-yellow font-bold uppercase tracking-tight">
          OPERATOR_PROFILE // SYSTEM_DIAGNOSTICS
        </h2>
        <p className="font-mono text-[10px] text-gray-500 uppercase mt-1">
          Review operator credentials and system theme profiles
        </p>
      </div>

      {/* Profile Card */}
      <div className="bg-cyber-panel border-3 border-black p-6 shadow-[4px_4px_0px_#000] flex flex-col sm:flex-row gap-6 items-start relative overflow-hidden">
        {/* Avatar Area */}
        <div className="relative flex-shrink-0 mx-auto sm:mx-0">
          <div className="w-32 h-32 border-3 border-black bg-black overflow-hidden shadow-[3px_3px_0px_#000] flex items-center justify-center">
            <span className="text-theme-accent font-press text-[18px] font-bold">
              {user.username[0].toUpperCase()}
            </span>
          </div>
          {/* LVL Badge */}
          <div className="absolute -bottom-1 -right-1 bg-theme-accent text-black border-2 border-black px-2 py-0.5 font-press text-[10px] font-bold rotate-6 shadow-[1.5px_1.5px_0px_#000]">
            LVL_1
          </div>
        </div>

        {/* Details Area */}
        <div className="flex-1 space-y-3 min-w-0 w-full text-center sm:text-left">
          <div>
            <h3 className="font-press text-[12px] text-theme-accent font-bold uppercase tracking-tight truncate">
              OPERATOR: {user.username}
            </h3>
            <p className="text-gray-400 font-mono text-[10px] border-l-2 border-theme-accent pl-3 py-0.5 mt-1 select-text">
              "If it fits in 24 bits, it ships."
            </p>
          </div>

          <div className="grid grid-cols-2 gap-4 pt-2">
            <div className="border border-dashed border-gray-700 p-2.5 bg-black/40">
              <span className="text-[10px] text-gray-500 font-press block mb-1">SYSTEM_UID</span>
              <span className="text-theme-accent font-mono text-[10px] font-bold select-text">{user.id.slice(0, 8).toUpperCase()}</span>
            </div>
            <div className="border border-dashed border-gray-700 p-2.5 bg-black/40">
              <span className="text-[10px] text-gray-500 font-press block mb-1">REGISTRATION</span>
              <span className="text-theme-accent font-mono text-[10px] font-bold select-text">{formatDate(user.created_at)}</span>
            </div>
          </div>
        </div>
      </div>

      {/* Settings Grid */}
      <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
        {/* Security & Credentials */}
        <section className="bg-cyber-panel border-3 border-black p-6 shadow-[4px_4px_0px_#000]">
          <h4 className="font-press text-[12px] text-theme-accent font-bold mb-6 flex items-center gap-2 uppercase">
            <Lock className="w-4 h-4" />
            ENCRYPTION_OVERRIDE
          </h4>

          <div className="space-y-4">
            <div className="space-y-1">
              <label className="text-[10px] font-press text-gray-500 uppercase">OPERATOR_ID_HANDLE</label>
              <input
                type="text"
                value={user.username}
                disabled
                className="w-full bg-[#0a0a0f] border-3 border-black p-3 font-mono text-xs text-gray-500 cursor-not-allowed"
              />
            </div>

            <button
              onClick={() => {
                if (confirm('CRITICAL PROTOCOL: ARE YOU ABSOLUTELY SURE YOU WANT TO DELETE YOUR ACCOUNT? THIS ACTION IS IRREVERSIBLE AND WILL DELETE ALL YOUR DATA.')) {
                  onDeleteAccount();
                }
              }}
              className="w-full bg-[#FF4444] hover:bg-[#FF6666] text-white font-press text-[10px] font-bold py-3 px-4 border-3 border-black shadow-[3px_3px_0px_#000] active:translate-x-[3px] active:translate-y-[3px] active:shadow-none transition-all cursor-pointer uppercase flex items-center justify-center gap-2"
            >
              <Trash2 className="w-4 h-4" />
              DELETE_ACCOUNT
            </button>
          </div>
        </section>

        {/* UI Customizer settings */}
        <section className="bg-cyber-panel border-3 border-black p-6 shadow-[4px_4px_0px_#000] flex flex-col justify-between">
          <div className="space-y-6">
            <h4 className="font-press text-[12px] text-theme-accent font-bold mb-4 flex items-center gap-2 uppercase">
              <Palette className="w-4 h-4" />
              UI_CONFIG_PARAMS
            </h4>

            {/* Accent Theme choice */}
            <div className="space-y-2">
              <label className="text-[10px] font-press text-gray-500 uppercase">TERMINAL_THEME</label>
              <div className="grid grid-cols-3 gap-2">
                {(['neon', 'cyber', 'pink'] as TerminalTheme[]).map((t) => {
                  const isActive = theme === t;
                  const bgColors = {
                    neon: 'bg-[#00FF66] text-black',
                    cyber: 'bg-[#FFF500] text-black',
                    pink: 'bg-[#FF007A] text-black',
                  };

                  return (
                    <button
                      key={t}
                      onClick={() => setTheme(t)}
                      className={`p-2.5 font-mono text-xs border-2 border-black cursor-pointer uppercase ${
                        isActive
                          ? bgColors[t] + ' font-bold shadow-none translate-x-[1px] translate-y-[1px]'
                          : 'bg-[#0c0c12] text-white hover:bg-white/5 shadow-[2px_2px_0px_#000]'
                      }`}
                      style={{ transition: 'none' }}
                    >
                      {t}
                    </button>
                  );
                })}
              </div>
            </div>

            {/* Custom Mouse Toggle */}
            <div className="space-y-2 pt-2">
              <label className="text-[10px] font-press text-gray-500 uppercase">HAND_CURSOR_PROTOCOL</label>
              <div className="flex items-center border-3 border-black w-fit">
                <button
                  onClick={() => setCustomCursor(true)}
                  className={`px-4 py-1.5 font-press text-[10px] cursor-pointer font-bold border-r-3 border-black ${
                    customCursor ? 'bg-theme-accent text-black' : 'bg-[#0a0a0f] text-gray-600 hover:bg-white/5'
                  }`}
                  style={{ transition: 'none' }}
                >
                  ON
                </button>
                <button
                  onClick={() => setCustomCursor(false)}
                  className={`px-4 py-1.5 font-press text-[10px] cursor-pointer font-bold ${
                    !customCursor ? 'bg-theme-accent text-black' : 'bg-[#0a0a0f] text-gray-600 hover:bg-white/5'
                  }`}
                  style={{ transition: 'none' }}
                >
                  OFF
                </button>
              </div>
            </div>
          </div>
        </section>
      </div>

      {/* Wipe Data Area */}
      <section className="border-3 border-hot-pink bg-hot-pink/5 p-6 shadow-[3px_3px_0px_rgba(255,0,122,0.15)] flex flex-col sm:flex-row items-center justify-between gap-4">
        <div className="text-center sm:text-left">
          <h4 className="text-hot-pink font-press text-[12px] font-bold uppercase flex items-center justify-center sm:justify-start gap-2">
            <ShieldAlert className="w-4 h-4" />
            WIPE_TERMINAL_DATA
          </h4>
          <p className="font-mono text-[10px] text-gray-400 mt-1 uppercase">
            Permanently delete all local compiled snapshots and logs. This is irreversible.
          </p>
        </div>
        <button
          onClick={() => {
            if (confirm('CRITICAL PROTOCOL: ARE YOU ABSOLUTELY SURE YOU WANT TO PURGE ALL SNAPSHOTS?')) {
              onWipeData();
              alert('PURGE COMPLETE.');
            }
          }}
          className="bg-hot-pink hover:bg-white text-black font-press text-[10px] font-bold py-3 px-6 border-3 border-black shadow-[3px_3px_0px_#000] active:translate-x-[3px] active:translate-y-[3px] active:shadow-none transition-all cursor-pointer uppercase shrink-0"
        >
          PURGE_ALL
        </button>
      </section>
    </div>
  );
}