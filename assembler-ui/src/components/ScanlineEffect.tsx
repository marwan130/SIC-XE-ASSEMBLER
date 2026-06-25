import type { ReactNode } from 'react';

interface ScanlineEffectProps {
  children: ReactNode;
  className?: string;
}

export const ScanlineEffect = ({ children, className = '' }: ScanlineEffectProps) => {
  return (
    <div className={`relative overflow-hidden ${className}`}>
      {children}
      <div className="absolute inset-0 pointer-events-none">
        <div className="w-full h-full animate-scanline bg-gradient-to-b from-transparent via-white/5 to-transparent" />
      </div>
    </div>
  );
};
