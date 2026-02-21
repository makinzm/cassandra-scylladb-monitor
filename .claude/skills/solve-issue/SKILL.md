---
name: solve-issue
description: Solve a GitHub issue end-to-end — read the issue, plan the work, implement it on a branch, commit, and open a pull request.
disable-model-invocation: true
argument-hint: <issue-number>
allowed-tools: Bash(gh issue view *), Bash(gh pr create *), Bash(git switch *), Bash(git add *), Bash(git commit *), Bash(git push *)
---

Solve GitHub issue $ARGUMENTS end-to-end following these steps in order:

## Step 1 — Read the issue

```bash
gh issue view $ARGUMENTS
```

Read the full issue: title, reason, acceptance criteria, and any `## Blocked by` section.

## Step 2 — Check blockers

If the issue body contains a `## Blocked by` section, for each listed issue number run:

```bash
gh issue view <blocking-issue-number> --json state,title
```

- If **any** blocking issue is still **open**, stop immediately and tell the user:
  > "Issue #$ARGUMENTS is blocked by #N (<title>), which is still open. Merge that first before solving this one."
- Only continue to Step 3 when **all** blocking issues are closed.

## Step 3 — Plan the solving order

Before touching any file:
- List which files need to be created or modified
- Determine the order (e.g. config before code, schema before logic)
- State the plan clearly so the user can understand the approach

## Step 4 — Create a branch

```bash
git switch -c issue-$ARGUMENTS/<short-description>
```

Use a short kebab-case description derived from the issue title.

## Step 5 — Implement

Work through the plan in the determined order:
- Read existing files before editing them
- Make only the changes required by the acceptance criteria
- Do not refactor unrelated code

## Step 6 — Commit

Stage specific files and commit with a conventional commit message:

```bash
git add <specific files>
git commit -m "<type>: <what and why>

Closes #$ARGUMENTS"
```

Commit types: `feat`, `fix`, `docs`, `chore`, `refactor`.
Group related changes in one commit; use multiple commits if changes are logically separate.

## Step 7 — Push and open a pull request

```bash
git push -u origin HEAD
gh pr create \
  --title "<title matching the issue>" \
  --body "$(cat <<'EOF'
## Summary
- <bullet describing what was done>

## Changes
- <file or component>: <why it was changed>

## How to verify
- [ ] <step matching acceptance criterion 1>
- [ ] <step matching acceptance criterion 2>

Closes #$ARGUMENTS
EOF
)"
```

Print the PR URL at the end.
