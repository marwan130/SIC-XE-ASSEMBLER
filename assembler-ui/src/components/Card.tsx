import type { ReactNode } from 'react';

interface CardProps {
  children: ReactNode;
  className?: string;
  header?: ReactNode;
  headerColor?: string;
}

export const Card = ({
  children,
  className = '',
  header,
  headerColor = 'bg-cyber-yellow',
}: CardProps) => {
  return (
    <div
      className={`
        bg-dark-bg-secondary
        border-3 border-black
        shadow-3d
        ${className}
      `}
      style={{ borderWidth: '3px' }}
    >
      {header && (
        <div
          className={`
            ${headerColor}
            border-b-3 border-black
            p-4
            font-bold
            uppercase
            tracking-wider
          `}
          style={{ borderWidth: '0 0 3px 0' }}
        >
          {header}
        </div>
      )}
      <div className="p-4">{children}</div>
    </div>
  );
};
