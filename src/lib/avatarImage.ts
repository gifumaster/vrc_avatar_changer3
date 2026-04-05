function readFileAsDataUrl(file: File): Promise<string> {
  return new Promise((resolve, reject) => {
    const reader = new FileReader();
    reader.onerror = () => reject(reader.error ?? new Error("Failed to read image file."));
    reader.onload = () => {
      if (typeof reader.result !== "string") {
        reject(new Error("Failed to load image data."));
        return;
      }
      resolve(reader.result);
    };
    reader.readAsDataURL(file);
  });
}

function loadImage(dataUrl: string): Promise<HTMLImageElement> {
  return new Promise((resolve, reject) => {
    const image = new Image();
    image.onerror = () => reject(new Error("Failed to decode image."));
    image.onload = () => resolve(image);
    image.src = dataUrl;
  });
}

function canvasToPngDataUrl(canvas: HTMLCanvasElement): string {
  return canvas.toDataURL("image/png");
}

export async function processAvatarImageFile(file: File): Promise<string> {
  const sourceUrl = await readFileAsDataUrl(file);
  const image = await loadImage(sourceUrl);
  const srcWidth = image.width;
  const srcHeight = image.height;
  const outputSize = srcWidth <= 1920 && srcHeight <= 1024 ? 1024 : 2048;
  const cropSize = Math.min(srcWidth, srcHeight);

  // Keep the same left-biased crop behavior as simple_resizer.
  const centerX = srcHeight / 2;
  const centerY = srcHeight / 2;
  const cropX = centerX - cropSize / 2;
  const cropY = centerY - cropSize / 2;

  const canvas = document.createElement("canvas");
  canvas.width = outputSize;
  canvas.height = outputSize;

  const context = canvas.getContext("2d");
  if (!context) {
    throw new Error("Canvas context is not available.");
  }

  context.drawImage(image, cropX, cropY, cropSize, cropSize, 0, 0, canvas.width, canvas.height);

  return canvasToPngDataUrl(canvas);
}

export function extractBase64Payload(dataUrl: string): string {
  const parts = dataUrl.split(",", 2);
  if (parts.length !== 2 || !parts[1]) {
    throw new Error("Processed image data is invalid.");
  }

  return parts[1];
}
