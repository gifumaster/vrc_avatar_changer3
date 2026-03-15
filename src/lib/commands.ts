import { invoke } from "@tauri-apps/api/core";

export type LoginInput = {
  username: string;
  password: string;
};

export type TwoFactorInput = {
  code: string;
  mode: "totp" | "emailotp";
};

export type AvatarFilter = {
  query: string;
  tags: string[];
};

export type AvatarSummary = {
  id: string;
  name: string;
  description: string;
  thumbnailUrl: string | null;
  thumbnailPath: string | null;
  thumbnailVersion: number | null;
  tags: string[];
  updatedAt: string | null;
};

export type AvatarCachePayload = {
  avatars: AvatarSummary[];
  lastSyncedAt: string | null;
};

export type StoredSession = {
  username: string;
  auth_token: string;
};

export type LoginResult = {
  status: "signed_out" | "pending_2fa" | "authenticated";
  username: string;
  auth_token: string | null;
  two_factor_mode: "totp" | "emailotp" | null;
};

export type SessionState = {
  status: "signed_out" | "pending_2fa" | "authenticated";
  username: string | null;
  two_factor_required: boolean;
};

export type OscSettings = {
  enabled: boolean;
  host: string;
  port: number;
};

export async function login(input: LoginInput): Promise<LoginResult> {
  return invoke<LoginResult>("login_vrchat", { request: input });
}

export async function submitTwoFactor(
  input: TwoFactorInput & { authToken: string; username: string },
): Promise<SessionState> {
  return invoke<SessionState>("submit_two_factor", {
    request: {
      auth_token: input.authToken,
      username: input.username,
      code: input.code,
      mode: input.mode,
    },
  });
}

export async function loadCachedAvatarList(): Promise<AvatarCachePayload> {
  return invoke<AvatarCachePayload>("load_cached_avatar_list");
}

export async function saveAvatarTags(avatarId: string, tags: string[]): Promise<AvatarCachePayload> {
  return invoke<AvatarCachePayload>("save_avatar_tags", { avatarId, tags });
}

export async function refreshAvatarList(_filter?: AvatarFilter): Promise<AvatarCachePayload> {
  return invoke<AvatarCachePayload>("refresh_avatar_list");
}

export async function refreshLatestAvatarPage(limit: 20 | 50 | 100): Promise<AvatarCachePayload> {
  return invoke<AvatarCachePayload>("refresh_latest_avatar_page", { limit });
}

export async function cacheAvatarThumbnails(avatarIds: string[]): Promise<AvatarCachePayload> {
  return invoke<AvatarCachePayload>("cache_avatar_thumbnails", { avatarIds });
}

export async function refreshAvatarDetail(avatarId: string): Promise<AvatarCachePayload> {
  return invoke<AvatarCachePayload>("refresh_avatar_detail", { avatarId });
}

export async function switchAvatarViaOsc(_avatarId: string): Promise<void> {
  return invoke("switch_avatar_via_osc", { avatarId: _avatarId });
}

export async function loadOscSettings(): Promise<OscSettings> {
  return invoke<OscSettings>("load_osc_settings");
}

export async function saveOscSettings(settings: OscSettings): Promise<OscSettings> {
  return invoke<OscSettings>("save_osc_settings", { settings });
}

export async function loadSavedSession(): Promise<StoredSession | null> {
  return invoke<StoredSession | null>("load_saved_session");
}

export async function saveSession(session: StoredSession): Promise<void> {
  return invoke("save_session", { session });
}

export async function clearSavedSession(): Promise<void> {
  return invoke("clear_saved_session");
}

export async function verifySession(): Promise<SessionState> {
  return invoke<SessionState>("verify_session");
}

export async function openExternalUrl(url: string): Promise<void> {
  return invoke("open_external_url", { url });
}
