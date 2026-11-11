/**
 * MindSet OS Dashboard App Wrapper for Window Manager
 * Copyright (c) 2026 LKS PVT LTD. All rights reserved.
 */

import React from 'react';
import { Dashboard } from '../../components/Dashboard';
import { useKernelStore } from '../../stores/kernelStore';

export const DashboardApp: React.FC = () => {
  const { lockSystem } = useKernelStore();
  return <Dashboard onLock={lockSystem} />;
};
