# repo-watch

A small local utility that scans your machine for Git repositories and automatically commits + pushes any changes to `origin`.

This is designed to be run on a schedule (cron/launchd) as a lightweight “backup my work” tool.

## What it does

For each discovered Git repo:

- computes a diff summary (`files changed`, `insertions`, `deletions`)
- skips autopush unless the change threshold is met (default: 20 total line changes)
- if the repo has an `origin` remote and the threshold is met, runs:
  - `git add -A`
  - `git commit -m "autopush: <files> files, +<ins>/-<del> (YYYY-MM-DD HH:MM)"`
  - `git push origin`

Repos with no changes or changes below the threshold are skipped.

## Install

```bash
git clone <this-repo>
cd repo-watch
cargo build --release
```

Binary will be at
```bash
./target/release/repo-watch
```

Run once
```bash
./target/release/repo-watch
```

Run via the bundled script
```bash
./scripts/autopush.sh
```

It will:
- build the binary if needed
- run repo-watch

## Scheduling (cron)

Example: run every 3 hours and log output:
```cron
0 */3 * * * /Users/<home>/repo-watch/scripts/autopush.sh >> /tmp/repo-watch.log 2>&1
```

Check your current cron jobs:
```cron
crontab -l
```

## Notes/limitations
- This tool cannot push repos without an origin remote.
- Authentication is handled by Git (SSH keys / credential helper). This tool does not manage auth.
- If a push fails due to being behind remote, it will report the failure (current behavior: do not pull/rebase automatically).
- Default threshold is 20 total line changes (`insertions + deletions`) to avoid commit spam.

## License

MIT