<template>
  <section class="card stack">
    <div>
      <p class="section-kicker">Settings</p>
    </div>

    <label>
      <span class="label">Avatar Switch Method</span>
      <select v-model="localMethod" class="text-input">
        <option value="osc">OSC</option>
        <option value="api">VRChat API</option>
      </select>
    </label>
    <p class="muted settings-hint">
      {{
        localMethod === "api"
          ? "別PC/VR機器で動作中ならこちら。反映はやや遅く、クライアント側で反映されない場合があります。"
          : "このPCで VRChat を起動しているときOSC機能で切り替えます。(推奨)"
      }}
    </p>
    <div class="osc-settings-group" :class="{ 'osc-settings-disabled': localMethod !== 'osc' }">
      <label>
        <span class="label">OSC Host</span>
        <input v-model.trim="localHost" class="text-input" type="text" placeholder="127.0.0.1" :disabled="localMethod !== 'osc'" />
      </label>
      <label>
        <span class="label">OSC Port</span>
        <input v-model.number="localPort" class="text-input" type="number" min="1" max="65535" :disabled="localMethod !== 'osc'" />
      </label>
    </div>
    <label class="row">
      <span>タグ機能を使う</span>
      <input v-model="localTagsEnabled" type="checkbox" />
    </label>
    <label class="row">
      <span>Switchボタンを表示</span>
      <input v-model="localSwitchButtonsEnabled" type="checkbox" />
    </label>
    <label>
      <span class="label">最新取得件数</span>
      <select v-model.number="localLatestFetchCount" class="text-input">
        <option :value="20">20</option>
        <option :value="50">50</option>
        <option :value="100">100</option>
      </select>
    </label>

    <div class="card-actions">
      <button class="primary-button" type="button" :disabled="busy" @click="handleSave">
        {{ busy ? "Saving..." : "Save Settings" }}
      </button>
    </div>
    <p v-if="props.message" class="muted">{{ props.message }}</p>
  </section>
</template>

<script setup lang="ts">
import { ref, watch } from "vue";

import type { AvatarSwitchState, UiSettings } from "@/types";

const props = defineProps<{
  switchSettings: AvatarSwitchState;
  ui: UiSettings;
  message?: string;
}>();

const emit = defineEmits<{
  save: [payload: { switchSettings: AvatarSwitchState; ui: UiSettings }];
}>();

const localMethod = ref(props.switchSettings.method);
const localHost = ref(props.switchSettings.osc.host);
const localPort = ref(props.switchSettings.osc.port);
const localTagsEnabled = ref(props.ui.tagsEnabled);
const localSwitchButtonsEnabled = ref(props.ui.switchButtonsEnabled);
const localLatestFetchCount = ref<20 | 50 | 100>(props.ui.latestFetchCount);
const busy = ref(false);

watch(
  () => props.switchSettings,
  (switchSettings) => {
    localMethod.value = switchSettings.method;
    localHost.value = switchSettings.osc.host;
    localPort.value = switchSettings.osc.port;
  },
  { deep: true },
);

watch(
  () => props.ui,
  (ui) => {
    localTagsEnabled.value = ui.tagsEnabled;
    localSwitchButtonsEnabled.value = ui.switchButtonsEnabled;
    localLatestFetchCount.value = ui.latestFetchCount;
  },
  { deep: true },
);

function handleSave() {
  busy.value = true;
  const normalizedHost = localHost.value.trim() === "" ? "127.0.0.1" : localHost.value.trim();
  const normalizedPort = Number(localPort.value) || 9000;

  emit("save", {
    switchSettings: {
      method: localMethod.value,
      osc: {
        enabled: true,
        host: normalizedHost,
        port: normalizedPort,
      },
    },
    ui: {
      tagsEnabled: localTagsEnabled.value,
      switchButtonsEnabled: localSwitchButtonsEnabled.value,
      latestFetchCount: localLatestFetchCount.value,
    },
  });

  localHost.value = normalizedHost;
  localPort.value = normalizedPort;
  busy.value = false;
}
</script>

<style scoped>
.settings-hint {
  margin: -10px 0 2px;
  line-height: 1.45;
}

.osc-settings-group {
  display: grid;
  gap: 12px;
}

.osc-settings-disabled {
  opacity: 0.48;
}
</style>
