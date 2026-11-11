import React from 'react';

export interface ButtonProps extends React.ButtonHTMLAttributes<HTMLButtonElement> {
  variant?: 'primary' | 'secondary' | 'accent' | 'danger' | 'ghost';
  size?: 'sm' | 'md' | 'lg';
  icon?: React.ReactNode;
}

export const Button: React.FC<ButtonProps> = ({
  children,
  variant = 'primary',
  size = 'md',
  icon,
  className = '',
  disabled,
  ...props
}) => {
  const sizeClasses = {
    sm: 'px-2.5 py-1 text-xs gap-1.5',
    md: 'px-3.5 py-1.5 text-sm gap-2',
    lg: 'px-4 py-2 text-base gap-2.5',
  };

  const variantClasses = {
    primary: 'bg-[#8B9CFF] text-[#07090D] font-medium hover:bg-[#A3B1FF] active:bg-[#7384E6]',
    secondary: 'bg-[#182235] text-[#F9FAFB] border border-[#202B40] hover:bg-[#202B40]',
    accent: 'bg-[#65E6C4] text-[#07090D] font-medium hover:bg-[#7EF2D3]',
    danger: 'bg-[#FF6B6B] text-[#07090D] font-medium hover:bg-[#FF8585]',
    ghost: 'bg-transparent text-[#9CA3AF] hover:text-[#F9FAFB] hover:bg-[#111827]',
  };

  return (
    <button
      disabled={disabled}
      className={`inline-flex items-center justify-center rounded-md font-sans transition-all duration-150 disabled:opacity-40 disabled:cursor-not-allowed select-none ${sizeClasses[size]} ${variantClasses[variant]} ${className}`}
      {...props}
    >
      {icon && <span className="flex-shrink-0">{icon}</span>}
      {children}
    </button>
  );
};

