---
name: create-issues
description: Create GitHub issues as scrum increments with reason and acceptance criteria. Use when planning features, bugs, or tasks as GitHub issues.
disable-model-invocation: true
argument-hint: <description of issues to create>
allowed-tools: Bash(gh issue create *), Bash(gh issue list *)
---

Create one or more GitHub issues in scrum increment style using `gh issue create`.

For each issue, structure the body exactly as:

```
## Reason
Why this issue exists — the problem it solves or the value it delivers.

## Blocked by
- #N — <one-line reason why that issue must be merged first>

(Omit this section entirely if the issue has no blockers.)

## Acceptance Criteria
- [ ] Criterion 1 (concrete and verifiable)
- [ ] Criterion 2
- [ ] Criterion 3
```

Guidelines:
- Title: action-oriented, imperative verb (e.g. "Add Prometheus scrape config for ScyllaDB")
- Split large features into small, independently deliverable issues
- Each criterion must be specific enough to verify — no vague statements like "works correctly"
- Add labels with `--label` when clearly applicable (e.g. `enhancement`, `bug`, `documentation`)
- If the arguments mention a dependency (e.g. "blocked by #2", "depends on #1"), always include the `## Blocked by` section

After creating each issue, print the issue URL.

Issues to create: $ARGUMENTS
