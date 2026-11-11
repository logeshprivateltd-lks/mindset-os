/**
 * MindSet OS Dashboard App Wrapper for Window Manager
 * Renders the 5-Zone Personalised Command Center Dashboard
 * Copyright (c) 2026 LKS PVT LTD. All rights reserved.
 */

import React from 'react';
import { CommandCenterPage } from '../../pages/command-center';
import { useKernelStore } from '../../stores/useKernelStore';

export const DashboardApp: React.FC = () => {
  const { lockSystem } = useKernelStore();
  return <CommandCenterPage onLock={lockSystem} />;
};
