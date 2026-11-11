/**
 * MindSet OS UAC Elevation Hook
 * Provides programmatic authorization modal for high-privilege operations
 * Copyright (c) 2026 LKS PVT LTD. All rights reserved.
 */

import { useState, useCallback } from 'react';
import { authClient } from '../../lib/authClient';
import type { UacModalProps } from './UacModal';

export function useUac() {
  const [isOpen, setIsOpen] = useState(false);
  const [reason, setReason] = useState('');
  const [targetResource, setTargetResource] = useState('');
  const [resolver, setResolver] = useState<((token: string | null) => void) | null>(null);

  const requestElevation = useCallback(
    async (operationReason: string, resource = 'MindSet OS Security Vault'): Promise<boolean> => {
      // First check if already elevated in kernel memory
      const alreadyElevated = await authClient.isUacElevated();
      if (alreadyElevated) {
        return true;
      }

      setReason(operationReason);
      setTargetResource(resource);
      setIsOpen(true);

      return new Promise<boolean>((resolve) => {
        setResolver(() => (token: string | null) => {
          setIsOpen(false);
          resolve(token !== null);
        });
      });
    },
    []
  );

  const handleSuccess = useCallback(
    (token: string) => {
      if (resolver) {
        resolver(token);
        setResolver(null);
      }
      setIsOpen(false);
    },
    [resolver]
  );

  const handleCancel = useCallback(() => {
    if (resolver) {
      resolver(null);
      setResolver(null);
    }
    setIsOpen(false);
  }, [resolver]);

  const modalProps: UacModalProps = {
    isOpen,
    reason,
    targetResource,
    onSuccess: handleSuccess,
    onCancel: handleCancel,
  };

  return {
    requestElevation,
    modalProps,
    isOpen,
  };
}
