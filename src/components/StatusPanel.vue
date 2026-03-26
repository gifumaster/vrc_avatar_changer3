<template>
  <aside class="panel status-panel">
    <h2>System status</h2>

    <div class="status-block">
      <div class="status-row">
        <span class="label">Session</span>
        <span class="value">{{ session.status }}</span>
      </div>
      <p class="muted">
        {{
          session.twoFactorRequired
            ? "Two-factor input is pending."
            : session.username
              ? `Signed in as ${session.username}.`
              : "No active session loaded."
        }}
      </p>
    </div>

    <div class="status-block">
      <div class="status-row">
        <span class="label">Cache</span>
        <span class="value">{{ cache.itemCount }} items</span>
      </div>
      <p class="muted">
        Last synced:
        {{ cache.lastSyncedAt ?? "not synced yet" }}
      </p>
    </div>

    <div class="status-block">
      <div class="status-row">
        <span class="label">Switch</span>
        <span class="value">{{ switchSettings.method.toUpperCase() }}</span>
      </div>
      <p class="muted">
        {{ switchSettings.method === "osc" ? `${switchSettings.osc.host}:${switchSettings.osc.port}` : "Uses saved VRChat session" }}
      </p>
    </div>
  </aside>
</template>

<script setup lang="ts">
import type { AvatarSwitchState, CacheState, SessionState } from "@/types";

defineProps<{
  session: SessionState;
  cache: CacheState;
  switchSettings: AvatarSwitchState;
}>();
</script>
