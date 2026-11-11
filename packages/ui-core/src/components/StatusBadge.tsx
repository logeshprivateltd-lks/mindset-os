import React from 'react';

export interface StatusBadgeProps {
  status: 'healthy' | 'warning' | 'critical' | 'info' | 'neutral';
  label: string;
  metric?: string;
}

export const StatusBadge: React.FC<StatusBadgeProps> = ({ status, label, metric }) => {
  const colorMap = {
    healthy: 'bg-[#65E6C4]/10 text-[#65E6C4] border-[#65E6C4]/30',
    warning: 'bg-[#FFD078]/10 text-[#FFD078] border-[#FFD078]/30',
    critical: 'bg-[#FF6B6B]/10 text-[#FF6B6B] border-[#FF6B6B]/30',
    info: 'bg-[#8B9CFF]/10 text-[#8B9CFF] border-[#8B9CFF]/30',
    neutral: 'bg-[#182235] text-[#9CA3AF] border-[#202B40]',
  };

  const dotMap = {
    healthy: 'bg-[#65E6C4]',
    warning: 'bg-[#FFD078]',
    critical: 'bg-[#FF6B6B]',
    info: 'bg-[#8B9CFF]',
    neutral: 'bg-[#9CA3AF]',
  };

  return (
    <span className={`inline-flex items-center space-x-1.5 px-2 py-0.5 rounded-full text-xs font-medium border ${colorMap[status]}`}>
      <span className={`w-1.5 h-1.5 rounded-full ${dotMap[status]}`} />
      <span>{label}</span>
      {metric && <span className="font-mono text-[10px] opacity-75">({metric})</span>}
    </span>
  );
};

