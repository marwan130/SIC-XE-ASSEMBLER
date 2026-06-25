import type { ButtonHTMLAttributes, ReactNode } from 'react';

interface ButtonProps extends ButtonHTMLAttributes<HTMLButtonElement> {
  children: ReactNode;
  variant?: 'primary' | 'secondary' | 'accent';
  size?: 'sm' | 'md' | 'lg';
  magnetic?: boolean;
}

export const Button = ({
  children,
  variant = 'primary',
  size = 'md',
  magnetic = false,
  className = '',
  ...props
}: ButtonProps) => {
  const variantStyles = {
    primary: 'bg-cyber-yellow text-black',
    secondary: 'bg-dark-bg-secondary text-white border-2 border-black',
    accent: 'bg-vibrant-purple text-white',
  };

  const sizeStyles = {
    sm: 'px-3 py-1 text-sm',
    md: 'px-6 py-2 text-base',
    lg: 'px-8 py-3 text-lg',
  };

  return (
    <button
      className={`
        ${variantStyles[variant]}
        ${sizeStyles[size]}
        ${magnetic ? 'magnetic' : ''}
        border-2 border-black
        shadow-3d
        hover:shadow-3d-hover
        active:shadow-3d-active
        active:translate-x-1
        active:translate-y-1
        transition-all
        duration-75
        font-bold
        uppercase
        tracking-wider
        ${className}
      `}
      {...props}
    >
      {children}
    </button>
  );
};
