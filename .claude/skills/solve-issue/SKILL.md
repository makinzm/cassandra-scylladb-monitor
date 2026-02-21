---
name: solve-issue
description: Solve a GitHub issue end-to-end — read the issue, plan the work, implement it on a branch, commit, and open a pull request.
disable-model-invocation: true
argument-hint: <issue-number>
allowed-tools: Bash(gh issue view *), Bash(gh issue list *), Bash(gh pr create *), Bash(git switch *), Bash(git add *), Bash(git commit *), Bash(git push *)
---

Solve GitHub issue $ARGUMENTS end-to-end following these steps in order:

## Step 0 — Resolve the issue number (only when no argument is given)

If `$ARGUMENTS` is empty, you must determine which issue to solve before proceeding.

1. Run:
   ```bash
   gh issue list --state open --json number,title,body
   ```
2. Parse each issue's `## Blocked by` section (if present) to build a full dependency graph.
3. Find every issue whose blockers are either closed or non-existent — these are the **unblocked** issues.
4. **Recommend the unblocked issue with the lowest number** as the natural next step, and explain:
   - Why it is unblocked (no open blockers)
   - Which other issues depend on it (i.e. what solving it will unlock)
   - The full dependency chain so the user can see the big picture
5. Ask the user which issue they want to solve. Use the recommended issue as the default choice.
6. Once the user picks an issue number, set that as the target and continue to Step 1.

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

## Step 6 — Verify acceptance criteria

Before committing, confirm every acceptance criterion is met:

- **Prefer TDD**: if the issue involves code with testable behaviour, write and run tests first (`cargo test`, `pytest`, etc.). Do not proceed to commit if tests fail.
- **If TDD is not applicable** (e.g. infrastructure, config, dashboards): confirm operation manually — start the relevant services and check each criterion by hand (e.g. `curl`, browser, CLI).
- Document the verification method in the PR body under "How to verify" so reviewers can reproduce it.

Do not commit until all acceptance criteria are verified.

## Step 7 — Commit

Stage specific files and commit with a conventional commit message:

```bash
git add <specific files>
git commit -m "<type>: <what and why in one sentence> — Closes #$ARGUMENTS"
```

Use a multi-line body only when the reason cannot fit in one sentence.
Commit types: `feat`, `fix`, `docs`, `chore`, `refactor`.
Group related changes in one commit; use multiple commits if changes are logically separate.

## Step 8 — Push and open a pull request

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
