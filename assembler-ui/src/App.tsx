import { useState, useEffect } from 'react';
import { useAuth } from './hooks/useAuth';
import { useAssembler } from './hooks/useAssembler';
import type { TerminalTheme } from './types';

import StarField from './components/background/StarField';
import CRTOverlay from './components/background/CRTOverlay';
import PixelCursor from './components/cursor/PixelCursor';

import LeftNavbar from './components/layout/LeftNavbar';
import PixelDivider from './components/layout/PixelDivider';
import CodeEditor from './components/assembler/CodeEditor';
import OutputPanel from './components/assembler/OutputPanel';
import HistoryGrid from './components/history/HistoryGrid';
import ProfilePage from './components/profile/ProfilePage';

import LoginPage from './components/auth/LoginPage';
import RegisterPage from './components/auth/RegisterPage';

export default function App() {
  const auth = useAuth();
  const asm = useAssembler(auth.isLoggedIn);

  // Navigation states
  const [activePage, setActivePage] = useState<string>('assembler');
  const [authScreen, setAuthScreen] = useState<'login' | 'register' | null>(null);
  const [navbarCollapsed, setNavbarCollapsed] = useState(false);

  // visual customizers
  const [themeAccent, setThemeAccent] = useState<TerminalTheme>(() => {
    return (localStorage.getItem('sicxe_theme_accent') as TerminalTheme) || 'neon';
  });
  const [customCursor, setCustomCursor] = useState(() => {
    const saved = localStorage.getItem('sicxe_custom_cursor');
    return saved !== null ? saved === 'true' : true;
  });

  // Brief pixel transition wipe state
  const [isWiping, setIsWiping] = useState(false);

  useEffect(() => {
    localStorage.setItem('sicxe_theme_accent', themeAccent);
  }, [themeAccent]);

  useEffect(() => {
    localStorage.setItem('sicxe_custom_cursor', String(customCursor));
    if (customCursor) {
      document.body.classList.add('custom-cursor-active');
    } else {
      document.body.classList.remove('custom-cursor-active');
    }
  }, [customCursor]);

  // Initialize cursor state on mount
  useEffect(() => {
    const savedCursor = localStorage.getItem('sicxe_custom_cursor');
    if (savedCursor === 'true') {
      document.body.classList.add('custom-cursor-active');
    }
  }, []);

  // Page switcher with pixel wipe animation delay
  const changePageWithWipe = (targetPage: string) => {
    if (targetPage === activePage) return;
    setIsWiping(true);
    setTimeout(() => {
      setActivePage(targetPage);
    }, 250); // half way through wipe
    setTimeout(() => {
      setIsWiping(false);
    }, 550); // wipe complete
  };

  // Helper: Wipe all history
  const handleWipeAllSnapshots = () => {
    localStorage.removeItem('sicxe_db_sessions');
    asm.setOutputs(null);
    asm.refreshHistory();
  };

  // Helper: Delete account
  const handleDeleteAccount = async () => {
    const result = await auth.deleteAccount();
    if (result.success) {
      setAuthScreen('login');
      setActivePage('assembler');
    }
  };

  // Handler: Selecting historical snapshot loads details and switches page
  const handleSelectSession = (session: any) => {
    asm.loadSession(session);
    changePageWithWipe('assembler');
  };

  // Build the content for active panel
  const renderActiveContent = () => {
    switch (activePage) {
      case 'assembler':
        return (
          <form 
            onSubmit={(e) => e.preventDefault()} 
            className="flex-1 flex flex-col md:flex-row h-full overflow-hidden"
          >
            {/* Editor panel */}
            <div className="flex-1 min-h-[50vh] md:min-h-0">
              <CodeEditor
                code={asm.code}
                setCode={asm.setCode}
                title={asm.sessionTitle}
                setTitle={asm.setSessionTitle}
                onAssemble={() => asm.assemble()}
                isAssembling={asm.loading}
              />
            </div>

            {/* Pixel Divider - hidden on mobile */}
            <div className="hidden md:block">
              <PixelDivider />
            </div>

            {/* Output Panel */}
            <div className="flex-1 min-h-[50vh] md:min-h-0 border-t-4 md:border-t-0 md:border-l-4 border-black">
              <OutputPanel
                outputs={asm.outputs}
                isLoading={asm.loading}
                error={asm.error}
                onClearError={asm.clearError}
                sessionTitle={asm.sessionTitle}
              />
            </div>
          </form>
        );

      case 'history':
        return (
          <HistoryGrid
            history={asm.history}
            onSelectSession={handleSelectSession}
            onDeleteSession={asm.deleteSession}
          />
        );

      case 'profile':
        return (
          <ProfilePage
            user={auth.user}
            theme={themeAccent}
            setTheme={setThemeAccent}
            customCursor={customCursor}
            setCustomCursor={setCustomCursor}
            onWipeData={handleWipeAllSnapshots}
            onDeleteAccount={handleDeleteAccount}
          />
        );

      default:
        return (
          <div className="flex-1 flex items-center justify-center font-press text-[12px] text-hot-pink font-bold">
            404_PAGE_NOT_FOUND
          </div>
        );
    }
  };

  // Decide theme accent class
  const getThemeClass = () => {
    switch (themeAccent) {
      case 'cyber':
        return 'theme-cyber';
      case 'pink':
        return 'theme-pink';
      case 'neon':
      default:
        return 'theme-neon';
    }
  };

  return (
    <div className={`min-h-screen text-white font-sans relative ${getThemeClass()}`}>
      {/* 1. Custom Pixel Stars & CRT overlays */}
      <StarField />
      <CRTOverlay />

      {/* 2. Custom Glove pointer */}
      {customCursor && <PixelCursor />}

      {/* 3. Pixel wipe transition block overlay */}
      {isWiping && (
        <div className="fixed inset-0 bg-black z-[999] flex flex-col pointer-events-none select-none">
          {Array.from({ length: 6 }).map((_, idx) => (
            <div
              key={idx}
              className="flex-1 bg-black border-y border-white/5 transition-transform duration-300 ease-in-out"
              style={{
                animation: 'pixel-wipe-horizontal 0.5s steps(8) forwards',
                animationDelay: `${idx * 45}ms`,
              }}
            />
          ))}
        </div>
      )}

      {/* 4. Auth modal overlay*/}
      {authScreen && !auth.isLoggedIn && (
        <div className="fixed inset-0 bg-black/80 z-[100] flex items-center justify-center p-4">
          <div className="relative w-full max-w-md">
            {authScreen === 'login' ? (
              <LoginPage
                onLogin={auth.login}
                onSwitchToRegister={() => setAuthScreen('register')}
                errorMsg={auth.error}
                clearError={auth.clearError}
                onClose={() => setAuthScreen(null)}
              />
            ) : (
              <RegisterPage
                onRegister={auth.register}
                onSwitchToLogin={() => setAuthScreen('login')}
                errorMsg={auth.error}
                clearError={auth.clearError}
                onClose={() => setAuthScreen(null)}
              />
            )}
          </div>
        </div>
      )}

      {/* 5. Main workstation interface */}
      <div className="flex flex-col md:flex-row h-auto md:h-screen overflow-auto md:overflow-hidden relative z-30">
        {/* Collapsible left navbar */}
        <LeftNavbar
          activePage={activePage}
          setActivePage={(page) => {
            // Require auth for history and profile
            if ((page === 'history' || page === 'profile') && !auth.isLoggedIn) {
              setAuthScreen('login');
              return;
            }
            changePageWithWipe(page);
          }}
          user={auth.user}
          onLogout={auth.logout}
          onLogin={() => setAuthScreen('login')}
          collapsed={navbarCollapsed}
          setCollapsed={setNavbarCollapsed}
        />

        {/* Active Workstation panel */}
        <main className="flex-1 flex flex-col min-h-screen md:h-full bg-black/20">
          {renderActiveContent()}
        </main>
      </div>

      <style>{`
        /* Pixel wipe horizontal block animation keys */
        @keyframes pixel-wipe-horizontal {
          0% {
            transform: scaleX(0);
            transform-origin: left;
          }
          50% {
            transform: scaleX(1);
            transform-origin: left;
          }
          51% {
            transform: scaleX(1);
            transform-origin: right;
          }
          100% {
            transform: scaleX(0);
            transform-origin: right;
          }
        }

        /* Responsive Theme variables for dynamic accent swaps */
        .theme-neon {
          --theme-accent: #00FF66;
          --theme-glow: 0 0 10px rgba(0, 255, 102, 0.4);
        }
        .theme-cyber {
          --theme-accent: #FFF500;
          --theme-glow: 0 0 10px rgba(255, 245, 0, 0.4);
        }
        .theme-pink {
          --theme-accent: #FF007A;
          --theme-glow: 0 0 10px rgba(255, 0, 122, 0.4);
        }
      `}</style>
    </div>
  );
}