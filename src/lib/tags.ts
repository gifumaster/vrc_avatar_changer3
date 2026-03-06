export const SEASON_TAGS = ["Spring", "Summer", "Fall", "Winter"] as const;

export type MultiTagMode = "all" | "any";

export function isSeasonTag(tag: string): boolean {
  return SEASON_TAGS.includes(tag as (typeof SEASON_TAGS)[number]);
}

export function splitTags(tags: string[]) {
  const unique = [...new Set(tags.map((tag) => tag.trim()).filter(Boolean))];
  const seasonTag = unique.find(isSeasonTag) ?? null;
  const multiTags = unique.filter((tag) => !isSeasonTag(tag));

  return { seasonTag, multiTags };
}

export function setSeasonTag(tags: string[], nextSeasonTag: string | null): string[] {
  const { multiTags } = splitTags(tags);
  return nextSeasonTag ? [nextSeasonTag, ...multiTags] : multiTags;
}

export function addMultiTag(tags: string[], newTag: string): string[] {
  const normalized = newTag.trim();
  if (!normalized) {
    return tags;
  }

  if (isSeasonTag(normalized)) {
    return setSeasonTag(tags, normalized);
  }

  const { seasonTag, multiTags } = splitTags(tags);
  if (multiTags.includes(normalized)) {
    return tags;
  }

  return seasonTag ? [seasonTag, ...multiTags, normalized] : [...multiTags, normalized];
}

export function removeTag(tags: string[], target: string): string[] {
  return tags.filter((tag) => tag !== target);
}
