export type AvatarSummary = {
  id: string;
  name: string;
  description: string;
  thumbnailUrl: string;
  tags: string[];
  updatedAt: string;
};

export type SessionState = {
  status: "signed_out" | "pending_2fa" | "authenticated";
  username: string | null;
  twoFactorRequired: boolean;
};

export type CacheState = {
  itemCount: number;
  lastSyncedAt: string | null;
};

export type OscState = {
  enabled: boolean;
  host: string;
  port: number;
};

export type UiSettings = {
  tagsEnabled: boolean;
  switchButtonsEnabled: boolean;
};
