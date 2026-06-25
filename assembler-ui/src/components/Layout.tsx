import type { ReactNode } from 'react';
import { Link } from 'react-router-dom';
import { Button } from './Button';
import { authService } from '../lib/auth';

interface LayoutProps {
  children: ReactNode;
}

export const Layout = ({ children }: LayoutProps) => {
  const isAuthenticated = authService.isAuthenticated();

  return (
    <div className="min-h-screen bg-dark-bg text-white">
      <nav className="sticky top-0 z-40 bg-dark-bg-secondary border-b-3 border-black px-6 py-4">
        <div className="max-w-7xl mx-auto flex items-center justify-between">
          <Link to="/" className="text-2xl font-bold font-mono text-retro-green hover:animate-flash">
            SIC/XE ASSEMBLER
          </Link>
          <div className="flex gap-4">
            {isAuthenticated ? (
              <>
                <span className="text-sm font-mono text-electric-blue">
                  {authService.getCurrentUser()?.email}
                </span>
                <Button
                  variant="secondary"
                  size="sm"
                  onClick={() => authService.logout()}
                >
                  LOGOUT
                </Button>
              </>
            ) : (
              <>
                <Link to="/login">
                  <Button variant="secondary" size="sm">
                    LOGIN
                  </Button>
                </Link>
                <Link to="/register">
                  <Button variant="primary" size="sm" magnetic>
                    REGISTER
                  </Button>
                </Link>
              </>
            )}
          </div>
        </div>
      </nav>
      <main className="max-w-7xl mx-auto px-6 py-8">{children}</main>
    </div>
  );
};