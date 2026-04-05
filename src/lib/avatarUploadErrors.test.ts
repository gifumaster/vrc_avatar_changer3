import { describe, expect, it } from "vitest";

import { formatAvatarUploadError } from "@/lib/avatarUploadErrors";

describe("formatAvatarUploadError", () => {
  it("formats unauthorized errors for users", () => {
    expect(formatAvatarUploadError("unauthorized")).toContain("再ログイン");
  });

  it("formats upload stage failures", () => {
    expect(formatAvatarUploadError("VRChat avatar image upload failed with status 400")).toContain(
      "VRChat への画像アップロードに失敗しました。",
    );
  });

  it("preserves unknown details while adding user-facing context", () => {
    expect(formatAvatarUploadError("something odd happened")).toBe(
      "サムネイル画像の更新に失敗しました。something odd happened",
    );
  });
});
