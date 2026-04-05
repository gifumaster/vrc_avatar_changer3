import { describe, expect, it } from "vitest";

import { extractBase64Payload } from "@/lib/avatarImage";

describe("extractBase64Payload", () => {
  it("returns the payload after the first comma", () => {
    expect(extractBase64Payload("data:image/png;base64,abc123")).toBe("abc123");
  });

  it("throws when the data url does not contain a payload", () => {
    expect(() => extractBase64Payload("data:image/png;base64")).toThrow("Processed image data is invalid.");
  });
});
