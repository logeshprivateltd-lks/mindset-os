/**
 * LockScreen wrapper forwarding to SmartLockOverlay
 * Copyright (c) 2026 LKS PVT LTD. All rights reserved.
 */

import React from 'react';
import { SmartLockOverlay } from '../pages/auth/SmartLockOverlay';

export const LockScreen: React.FC<{ onUnlock: () => void }> = ({ onUnlock }) => {
  return <SmartLockOverlay onUnlock={onUnlock} />;
};

export default LockScreen;
