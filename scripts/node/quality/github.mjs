import fs from 'node:fs';

function readGithubEvent(env) {
  if (env.GITHUB_EVENT_NAME !== 'pull_request' || !env.GITHUB_EVENT_PATH) return null;
  return JSON.parse(fs.readFileSync(env.GITHUB_EVENT_PATH, 'utf8'));
}

export function readGithubEventBody(env) {
  const payload = readGithubEvent(env);
  if (!payload) return null;
  return payload.pull_request?.body ?? '';
}

export function readGithubHeadSha(env) {
  const payload = readGithubEvent(env);
  if (!payload) return null;
  return payload.pull_request?.head?.sha ?? null;
}

export function readGithubPullRequest(env) {
  const payload = readGithubEvent(env);
  return payload?.pull_request ?? null;
}
