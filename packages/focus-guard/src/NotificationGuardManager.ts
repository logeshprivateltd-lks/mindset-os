/**
 * MindSet OS Focus Guard — Notification Guard Manager
 * Enforces selective notification suppression while strictly guaranteeing
 * uninterrupted emergency communication channels.
 * Copyright (c) 2026 LKS PVT LTD. All rights reserved.
 */

import { NotificationGuardRule } from './types';

export class NotificationGuardManager {
  private rules: Map<string, NotificationGuardRule> = new Map();
  private isSuppressed: boolean = false;
  private allowEmergencyInSuppression: boolean = true;

  constructor(initialRules?: NotificationGuardRule[]) {
    if (initialRules) {
      for (const rule of initialRules) {
        this.rules.set(rule.appId, rule);
      }
    }
  }

  /**
   * Adds or updates a notification filter rule.
   */
  public addRule(rule: NotificationGuardRule): void {
    this.rules.set(rule.appId, rule);
  }

  /**
   * Removes a rule by app ID.
   */
  public removeRule(appId: string): void {
    this.rules.delete(appId);
  }

  /**
   * Lists all current rules.
   */
  public getRules(): NotificationGuardRule[] {
    return Array.from(this.rules.values());
  }

  /**
   * Activates system-wide notification suppression.
   */
  public suppressAll(exceptEmergency: boolean = true): void {
    this.isSuppressed = true;
    this.allowEmergencyInSuppression = exceptEmergency;
  }

  /**
   * Restores normal notification delivery.
   */
  public restoreAll(): void {
    this.isSuppressed = false;
  }

  /**
   * Returns current suppression status.
   */
  public isSuppressionActive(): boolean {
    return this.isSuppressed;
  }

  /**
   * Core routing decision: determines whether a notification should surface.
   * Emergency communication is NEVER blocked when flagged.
   */
  public shouldAllow(appId: string, isEmergencyContact: boolean = false): boolean {
    // Universal safeguard: Emergency contacts always pass through
    if (isEmergencyContact && this.allowEmergencyInSuppression) {
      return true;
    }

    if (this.isSuppressed) {
      // In suppression mode, only explicitly allowed apps or emergency bypass
      const rule = this.rules.get(appId);
      if (rule?.alwaysAllowEmergency && isEmergencyContact) {
        return true;
      }
      return rule ? rule.allow : false;
    }

    // Normal mode: allow unless an explicit block rule exists
    const rule = this.rules.get(appId);
    return rule ? rule.allow : true;
  }
}

