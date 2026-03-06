<template>
  <section class="card stack">
    <div>
      <p class="section-kicker">Authentication</p>
      <h3>VRChat Login</h3>
    </div>

    <template v-if="session.status !== 'authenticated'">
      <template v-if="session.status !== 'pending_2fa'">
        <label>
          <span class="label">Username</span>
          <input
            v-model.trim="username"
            class="text-input"
            type="text"
            placeholder="VRChat username"
          />
        </label>
        <label>
          <span class="label">Password</span>
          <input
            v-model.trim="password"
            class="text-input"
            type="password"
            placeholder="VRChat password"
          />
        </label>
      </template>

      <label>
        <span class="label">2FA Code</span>
        <input
          v-model.trim="twoFactorCode"
          class="text-input"
          type="text"
          :placeholder="
            session.status === 'pending_2fa'
              ? 'Enter your 2FA code'
              : 'Required only when VRChat requests 2FA'
          "
          :disabled="session.status !== 'pending_2fa'"
        />
      </label>

      <div class="card-actions">
        <button
          class="primary-button"
          type="button"
          :disabled="isSubmitDisabled"
          @click="handlePrimaryAction"
        >
          {{ primaryActionLabel }}
        </button>
        <span class="muted">
          {{
            session.status === "pending_2fa"
              ? "VRChat requested a two-factor confirmation."
              : "Successful login stores the session in Windows Credential Manager."
          }}
        </span>
      </div>
    </template>

    <template v-else>
      <div class="row">
        <span>Saved account</span>
        <strong>{{ session.username }}</strong>
      </div>
      <div class="card-actions">
        <button class="ghost-button" type="button" :disabled="busy" @click="handleClear">
          {{ busy ? "Signing out..." : "Sign out" }}
        </button>
        <span class="muted">The saved token will be verified again on the next app launch.</span>
      </div>
    </template>

    <p v-if="errorMessage" class="error-text">{{ errorMessage }}</p>
  </section>
</template>

<script setup lang="ts">
import { computed, ref } from "vue";

import { clearSavedSession, login, submitTwoFactor } from "@/lib/commands";
import type { SessionState } from "@/types";

const props = defineProps<{
  session: SessionState;
}>();

const emit = defineEmits<{
  signedIn: [];
  pendingTwoFactor: [];
  cleared: [];
}>();

const username = ref("");
const password = ref("");
const pendingAuthToken = ref("");
const pendingMode = ref<"totp" | "emailotp">("totp");
const twoFactorCode = ref("");
const busy = ref(false);
const errorMessage = ref("");

const primaryActionLabel = computed(() => {
  if (busy.value) {
    return props.session.status === "pending_2fa" ? "Verifying..." : "Signing in...";
  }

  return props.session.status === "pending_2fa" ? "Verify 2FA" : "Sign in";
});

const isSubmitDisabled = computed(() => {
  if (busy.value) {
    return true;
  }

  if (props.session.status === "pending_2fa") {
    return pendingAuthToken.value === "" || twoFactorCode.value === "";
  }

  return username.value === "" || password.value === "";
});

async function handlePrimaryAction() {
  if (props.session.status === "pending_2fa") {
    await handleVerifyTwoFactor();
    return;
  }

  await handleLogin();
}

async function handleLogin() {
  busy.value = true;
  errorMessage.value = "";

  try {
    const result = await login({
      username: username.value,
      password: password.value,
    });

    username.value = result.username;
    password.value = "";
    pendingAuthToken.value = result.auth_token ?? "";
    pendingMode.value = result.two_factor_mode ?? "totp";

    if (result.status === "pending_2fa") {
      twoFactorCode.value = "";
      emit("pendingTwoFactor");
      return;
    }

    emit("signedIn");
  } catch (error) {
    errorMessage.value = error instanceof Error ? error.message : "Failed to sign in.";
  } finally {
    busy.value = false;
  }
}

async function handleVerifyTwoFactor() {
  busy.value = true;
  errorMessage.value = "";

  try {
    await submitTwoFactor({
      authToken: pendingAuthToken.value,
      username: username.value,
      code: twoFactorCode.value,
      mode: pendingMode.value,
    });

    pendingAuthToken.value = "";
    twoFactorCode.value = "";
    emit("signedIn");
  } catch (error) {
    errorMessage.value =
      error instanceof Error ? error.message : "Failed to verify the two-factor code.";
  } finally {
    busy.value = false;
  }
}

async function handleClear() {
  busy.value = true;
  errorMessage.value = "";

  try {
    await clearSavedSession();
    username.value = "";
    password.value = "";
    pendingAuthToken.value = "";
    twoFactorCode.value = "";
    emit("cleared");
  } catch (error) {
    errorMessage.value = error instanceof Error ? error.message : "Failed to sign out.";
  } finally {
    busy.value = false;
  }
}
</script>
