import { describe, expect, it } from "vitest";

import { addMultiTag, removeTag, setSeasonTag, splitTags } from "@/lib/tags";

describe("tags helpers", () => {
  it("deduplicates and separates season tags", () => {
    expect(splitTags(["Spring", " cute ", "cute", ""])).toEqual({
      seasonTag: "Spring",
      multiTags: ["cute"],
    });
  });

  it("replaces the season tag when a new season tag is added", () => {
    expect(addMultiTag(["Spring", "cute"], "Winter")).toEqual(["Winter", "cute"]);
  });

  it("adds a regular multi tag without duplicating existing tags", () => {
    expect(addMultiTag(["Spring", "cute"], "cool")).toEqual(["Spring", "cute", "cool"]);
    expect(addMultiTag(["Spring", "cute"], "cute")).toEqual(["Spring", "cute"]);
  });

  it("updates season tag explicitly", () => {
    expect(setSeasonTag(["Spring", "cute"], "Fall")).toEqual(["Fall", "cute"]);
    expect(setSeasonTag(["Spring", "cute"], null)).toEqual(["cute"]);
  });

  it("removes only the requested tag", () => {
    expect(removeTag(["Spring", "cute", "cool"], "cute")).toEqual(["Spring", "cool"]);
  });
});
