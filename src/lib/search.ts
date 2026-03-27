const normalize = (value: string) =>
  value
    .normalize("NFD")
    .replace(/[\u0300-\u036f]/g, "")
    .toLowerCase()
    .trim();

const tokenScore = (query: string, candidate: string) => {
  if (!query) {
    return 1;
  }

  const normalizedQuery = normalize(query);
  const normalizedCandidate = normalize(candidate);

  if (!normalizedCandidate) {
    return 0;
  }

  if (normalizedCandidate === normalizedQuery) {
    return 1000;
  }

  if (normalizedCandidate.startsWith(normalizedQuery)) {
    return 700 - Math.max(0, normalizedCandidate.length - normalizedQuery.length);
  }

  if (normalizedCandidate.includes(normalizedQuery)) {
    return 450 - normalizedCandidate.indexOf(normalizedQuery);
  }

  let queryIndex = 0;
  let streak = 0;
  let score = 0;

  for (let index = 0; index < normalizedCandidate.length; index += 1) {
    if (normalizedCandidate[index] === normalizedQuery[queryIndex]) {
      queryIndex += 1;
      streak += 1;
      score += 25 * streak;
      if (queryIndex === normalizedQuery.length) {
        return score;
      }
    } else {
      streak = 0;
    }
  }

  return 0;
};

export const rankItems = <T>(
  items: T[],
  query: string,
  extractor: (item: T) => string[]
) => {
  if (!query.trim()) {
    return items.map((item) => ({ item, score: 1 }));
  }

  return items
    .map((item) => {
      const score = extractor(item).reduce((best, part) => {
        return Math.max(best, tokenScore(query, part));
      }, 0);

      return { item, score };
    })
    .filter((entry) => entry.score > 0)
    .sort((left, right) => right.score - left.score);
};
