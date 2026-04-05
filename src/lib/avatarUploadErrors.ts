export function formatAvatarUploadError(error: unknown): string {
  const rawMessage =
    typeof error === "string" ? error : error instanceof Error ? error.message : "Failed to upload avatar image.";
  const message = rawMessage.replace(/\s+/g, " ").trim();

  if (message.includes("unauthorized")) {
    return "VRChat セッションの認証に失敗しました。再ログインしてからもう一度お試しください。";
  }

  if (message.includes("did not return a usable file URL")) {
    return "画像アップロードは成功しましたが、VRChat から画像 URL を取得できませんでした。";
  }

  if (message.includes("did not return imageUrl")) {
    return "画像は送信できましたが、VRChat 側でアバター画像の更新結果を確認できませんでした。";
  }

  if (message.includes("VRChat avatar image upload failed")) {
    return `VRChat への画像アップロードに失敗しました。${message}`;
  }

  if (message.includes("VRChat avatar update failed")) {
    return `VRChat 側のアバター更新に失敗しました。${message}`;
  }

  if (message.includes("Failed to decode image")) {
    return "画像を読み込めませんでした。別の画像でお試しください。";
  }

  if (message.includes("Failed to read image file")) {
    return "画像ファイルを読み取れませんでした。";
  }

  return `サムネイル画像の更新に失敗しました。${message}`;
}
