import React from 'react';

export interface AuroraCardProps {
  children: React.ReactNode;
  title?: string;
  badge?: React.ReactNode;
  className?: string;
  elevated?: boolean;
}

export const AuroraCard: React.FC<AuroraCardProps> = ({
  children,
  title,
  badge,
  className = '',
  elevated = false,
}) => {
  return (
    <div
      className={`rounded-lg border p-4 transition-all duration-200 ${
        elevated
          ? 'bg-[#182235] border-[#202B40] shadow-[0_4px_20px_rgba(0,0,0,0.4)]'
          : 'bg-[#111827] border-[#1F2937] hover:border-[#374151]'
      } ${className}`}
    >
      {(title || badge) && (
        <div className="flex items-center justify-between pb-3 mb-3 border-b border-[#1F2937]">
          {title && <h3 className="text-sm font-semibold text-[#F9FAFB] tracking-wide">{title}</h3>}
          {badge && <div>{badge}</div>}
        </div>
      )}
      {children}
    </div>
  );
};

