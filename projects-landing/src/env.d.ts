/// <reference types="astro/client" />

interface RepoData {
  name: string;
  description: string | null;
  url: string;
  topics: string[];
  isPrivate: boolean;
  isArchived: boolean;
  stargazerCount: number;
  pushedAt: string;
}
