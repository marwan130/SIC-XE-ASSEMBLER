import { Terminal, Database, User, LogOut, ChevronLeft, ChevronRight } from 'lucide-react';
import type { User as UserType } from '../../lib/auth';

interface LeftNavbarProps {
  activePage: string;
  setActivePage: (page: string) => void;
  user: UserType | null;
  onLogout: () => void;
  onLogin?: () => void;
  collapsed: boolean;
  setCollapsed: (collapsed: boolean) => void;
}

export default function LeftNavbar({
  activePage,
  setActivePage,
  user,
  onLogout,
  onLogin,
  collapsed,
  setCollapsed,
}: LeftNavbarProps) {

  const navItems = [
    { id: 'assembler', label: 'Assembler', icon: Terminal, color: '#00FF66' }, // Neon Green
    { id: 'history', label: 'History', icon: Database, color: '#FFF500' },    // Cyber Yellow
    { id: 'profile', label: 'Profile', icon: User, color: '#00E0FF' },      // Electric Blue
  ];

  // Quick 8-bit synthetic mechanical audio feedback loop
  const playSelectSound = (isHighPitch: boolean = false) => {
    try {
      const ctx = new (window.AudioContext || (window as any).webkitAudioContext)();
      const osc = ctx.createOscillator();
      const gain = ctx.createGain();

      osc.type = 'square'; // Authentic retro NES console sound waves
      osc.frequency.setValueAtTime(isHighPitch ? 1200 : 880, ctx.currentTime);

      gain.gain.setValueAtTime(0.02, ctx.currentTime);
      gain.gain.exponentialRampToValueAtTime(0.00001, ctx.currentTime + 0.05); // Quick decay

      osc.connect(gain);
      gain.connect(ctx.destination);
      osc.start();
      osc.stop(ctx.currentTime + 0.05);
    } catch (e) {
    }
  };

  return (
    <aside 
      className={`bg-[#0A0A0A] border-b-4 md:border-b-0 md:border-r-4 border-black flex flex-row flex-wrap md:flex-nowrap md:flex-col justify-between select-none z-30 shrink-0 transition-all duration-75 ease-out relative w-full md:w-auto h-auto md:h-screen ${
        collapsed ? 'md:w-[64px]' : 'md:w-[200px]'
      }`}
    >
      {/* Top Header Module / Game Main Marquee */}
      <div className="p-2 md:p-3 border-b-0 md:border-b-4 border-r-4 md:border-r-4 border-black bg-[#121212] flex flex-row md:flex-col items-center justify-center min-h-[56px] md:min-h-[72px] relative overflow-hidden shrink-0">
        {/* Synthwave horizontal grid light rail */}
        <div className="absolute inset-x-0 md:inset-x-0 top-0 md:top-0 h-[2px] bg-gradient-to-r from-transparent via-[#FF007A] to-transparent"></div>
        
        {/* Mobile Logo */}
        <div className="block md:hidden text-center min-w-[70px]">
          <h1 className="font-press text-[12px] text-white font-bold tracking-tighter uppercase relative">
            SIC/<span className="text-theme-accent animate-pulse">XE</span>
          </h1>
        </div>

        {/* Desktop Logo */}
        <div className="hidden md:block">
          {collapsed ? (
            <div className="w-8 h-8 bg-black border-2 border-theme-accent flex items-center justify-center animate-pulse shadow-[0_0_8px_var(--theme-accent)]">
              <img src="/assembler.png" alt="SIC/XE" style={{ width: 32, height: 32, objectFit: 'contain', imageRendering: 'pixelated' }} />
            </div>
          ) : (
            <div className="text-center min-w-[140px]">
              <h1 className="font-press text-[16px] md:text-[18px] text-white font-bold tracking-tighter uppercase relative">
                SIC/<span className="text-theme-accent animate-pulse">XE</span>
              </h1>
              <div className="flex items-center justify-center gap-1 mt-1">
                <span className="inline-block w-1.5 h-1.5 rounded-full bg-theme-accent animate-ping"></span>
                <span className="font-mono text-[9px] md:text-[10px] text-theme-accent tracking-widest font-bold">
                  CPU_LINK_ON
                </span>
              </div>
            </div>
          )}
        </div>
      </div>

      {/* Navigation Stack - horizontal on mobile, vertical on desktop */}
      <nav className="flex-1 py-1 md:py-6 flex flex-row flex-wrap md:flex-col gap-1 md:gap-3 px-2 md:px-0 justify-center md:justify-start">
        {navItems.map((item) => {
          const Icon = item.icon;
          const isActive = activePage === item.id;

          return (
            <button
              key={item.id}
              onClick={() => {
                playSelectSound(isActive);
                setActivePage(item.id);
              }}
              className={`w-auto md:w-[calc(100%-16px)] mx-0 md:mx-2 py-1.5 md:py-3 px-2 md:px-3 flex items-center gap-1.5 md:gap-3 transition-all duration-75 uppercase font-press text-[9px] md:text-[10px] text-left border-2 ${
                isActive
                  ? 'bg-black text-white border-theme-accent shadow-[2px_2px_0px_var(--theme-accent)] md:shadow-[3px_3px_0px_var(--theme-accent)]'
                  : 'text-gray-400 bg-[#121212] border-black hover:text-white hover:border-neutral-700 hover:translate-x-0.5'
              }`}
            >
              <Icon
                className="w-4 h-4 flex-shrink-0"
                style={{ color: isActive ? 'var(--theme-accent)' : item.color }}
              />
              <span className={`truncate flex-1 flex justify-between items-center block ${collapsed ? 'md:hidden' : 'md:block'}`}>
                {item.label}
                {isActive && <span className="text-theme-accent text-[10px] animate-bounce hidden md:inline">◀</span>}
              </span>
            </button>
          );
        })}
      </nav>

      {/* User Controls and Panel Navigation */}
      <div className="border-t-0 md:border-t-4 border-l-4 md:border-l-0 border-black p-2 flex flex-row md:flex-col gap-2 md:gap-3 bg-[#121212] items-center shrink-0">
        
        {/* User Identity Display */}
        {user ? (
          <div 
            onClick={() => { playSelectSound(true); setActivePage('profile'); }}
            className="flex items-center gap-2 p-1.5 overflow-hidden cursor-pointer bg-black/60 border-2 border-transparent hover:border-[#00E0FF] transition-all"
          >
            {/* Level Profile Icon Container */}
            <div className="w-8 h-8 bg-black border-2 border-[#00FF66] text-[#00FF66] flex-shrink-0 flex items-center justify-center font-press text-[12px] font-bold shadow-[inset_0_0_4px_#00FF66]">
              {user.username[0].toUpperCase()}
            </div>
            <div className={`flex flex-col min-w-0 block ${collapsed ? 'md:hidden' : 'md:block'}`}>
              <span className="font-press text-[9px] md:text-[10px] text-white truncate font-bold">
                {user.username}
              </span>
              <span className="font-mono text-[10px] md:text-[12px] text-[#FFF500] uppercase tracking-wider leading-none">
                OPERATOR
              </span>
            </div>
          </div>
        ) : (
          onLogin && (
            <button
              onClick={() => { 
                playSelectSound(true); 
                onLogin(); 
              }}
              className="w-auto md:w-full py-2 px-2 flex items-center gap-2 text-gray-400 bg-black hover:text-[#00FF66] border-2 border-neutral-900 hover:border-[#00FF66] transition-all duration-75 uppercase font-press text-[9px] md:text-[10px] text-left"
            >
              <User className="w-4 h-4 text-[#00FF66] flex-shrink-0" />
              <span className={`block ${collapsed ? 'md:hidden' : 'md:block'}`}>CONNECT</span>
            </button>
          )
        )}

        {/* Action Controls / Bottom Buttons */}
        <div className="flex gap-1.5 relative min-h-[32px] items-center">
          {user && (
            <button
              onClick={() => { playSelectSound(false); onLogout(); }}
              className={`py-2 px-2 flex items-center justify-center bg-black hover:bg-[#FF007A] border-2 border-neutral-900 hover:border-black text-gray-400 hover:text-black transition-all duration-75 font-press text-[9px] w-auto md:w-auto ${
                collapsed ? 'md:hidden' : 'md:flex-1'
              }`}
              title="DISCONNECT HARDWARE NETWORK"
            >
              <LogOut className="w-4 h-4 flex-shrink-0" />
              <span className={`ml-2 block ${collapsed ? 'md:hidden' : 'md:block'}`}>EJECT</span>
            </button>
          )}

          {/* Structural Ribbon Arrow Switch */}
          <button
            onClick={() => { playSelectSound(false); setCollapsed(!collapsed); }}
            className={`h-8 min-w-[32px] bg-[#1a1a2e] hover:bg-[#00E0FF] text-white hover:text-black border-2 border-black flex items-center justify-center transition-all duration-75 shrink-0 hidden md:flex ${
              collapsed ? 'md:w-8' : 'w-10'
            }`}
          >
            {collapsed ? <ChevronRight className="w-4 h-4 animate-pulse" /> : <ChevronLeft className="w-4 h-4" />}
          </button>
        </div>

      </div>
    </aside>
  );
}
