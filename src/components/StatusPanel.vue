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
        <span class="label">OSC</span>
        <span class="value">{{ osc.enabled ? "enabled" : "disabled" }}</span>
      </div>
      <p class="muted">{{ osc.host }}:{{ osc.port }}</p>
    </div>
  </aside>
</template>

<script setup lang="ts">
import type { CacheState, OscState, SessionState } from "@/types";

defineProps<{
  session: SessionState;
  cache: CacheState;
  osc: OscState;
}>();
</script>
