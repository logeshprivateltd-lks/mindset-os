/**
 * MindSet OS Native Low-RAM Terminal App
 * Fast interactive command-line interface for kernel diagnostics and system control
 * Copyright (c) 2026 LKS PVT LTD. All rights reserved.
 */

import React from 'react';
import { MindSetTerminal } from '../../components/terminal';

export const TerminalApp: React.FC = () => {
  return (
    <div className="w-full h-full p-2 bg-[#07090D]">
      <MindSetTerminal />
    </div>
  );
};
