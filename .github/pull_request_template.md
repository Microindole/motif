## Summary
- [ ] This PR keeps the change scoped and avoids unrelated edits.
- [ ] I updated docs in `agent/` if product direction, architecture, or workflow changed.
- [ ] I updated `cases/`, `demo/`, or tests if behavior changed.

Write 2-4 lines explaining what changed and why.

## Hard checks
- [ ] `./scripts/check-quality.ps1`
- [ ] CI should pass `quality`, `coverage`, and `CodeQL`

## Structure review
- [ ] No file became an obvious god file or mixed multiple responsibilities.
- [ ] No directory became overly flat or filled with pattern-cloned files.
- [ ] No generated artifacts or cache directories were added to Git.

## AI-specific review
- [ ] No copy-paste implementation was added where an abstraction should exist.
- [ ] Names are semantic and not placeholders like `helper`, `util`, `temp`, `final`, or `new`.
- [ ] Comments explain constraints or tradeoffs, not obvious code steps.
- [ ] No black-magic or unsafe shortcut was introduced.

## Large change override
- [ ] This PR intentionally exceeds the change-size gate and requires explicit human review.

Explain why the change cannot be split safely, and what reviewers should focus on.
