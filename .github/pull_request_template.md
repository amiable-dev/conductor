## Description

<!-- Provide a clear and concise description of your changes -->

## Motivation

<!-- Why is this change needed? What problem does it solve? -->
<!-- Link to related issues: Fixes #123, Closes #456 -->

## Type of Change

<!-- Mark the relevant option with an 'x' -->

- [ ] Bug fix (non-breaking change that fixes an issue)
- [ ] New feature (non-breaking change that adds functionality)
- [ ] Breaking change (fix or feature that would cause existing functionality to not work as expected)
- [ ] Documentation update
- [ ] Performance improvement
- [ ] Code refactoring (no functional changes)
- [ ] Build/CI configuration change
- [ ] Other (please describe):

## Changes Made

<!-- List the main changes in this PR -->

-
-
-

## Testing Performed

<!-- Describe how you tested your changes -->

- [ ] Tested with real MIDI device
- [ ] Ran full test suite: `cargo test --all`
- [ ] Ran clippy: `cargo clippy -- -D warnings`
- [ ] Ran formatter: `cargo fmt --check`
- [ ] Tested on multiple MIDI devices (if device-related)
- [ ] Tested LED feedback (if LED-related)
- [ ] Manual testing performed (describe below)

### Manual Testing Details

<!-- Describe manual testing steps and results -->

```
Steps:
1.
2.
3.

Results:
-
```

## Screenshots / Videos

<!-- If applicable, add screenshots or videos to demonstrate the changes -->
<!-- For LED feedback changes, a video is especially helpful -->

## Configuration Examples

<!-- If this adds/changes configuration options, provide examples -->

```toml
# Example config showing new feature

```

## Breaking Changes

<!-- If this is a breaking change, describe the impact and migration path -->

- [ ] Config format changes (provide migration guide)
- [ ] API changes (document in CHANGELOG)
- [ ] Behavior changes (explain new behavior)

### Migration Guide

<!-- How should users update their configs/code? -->

```toml
# Old config:


# New config:

```

## Performance Impact

<!-- Describe any performance implications -->

- [ ] No performance impact
- [ ] Performance improvement (describe below)
- [ ] Performance regression (justify below)

## Checklist

<!-- Mark completed items with an 'x' -->

- [ ] Code follows the project's style guidelines (rustfmt + clippy clean)
- [ ] Self-review completed
- [ ] Comments added for complex logic
- [ ] Documentation updated (if applicable)
  - [ ] Updated rustdoc comments
  - [ ] Updated README (if needed)
  - [ ] Updated configuration documentation
  - [ ] Updated CHANGELOG.md
- [ ] Tests added/updated
  - [ ] Unit tests for new functions
  - [ ] Integration tests (if applicable)
  - [ ] All tests pass locally
- [ ] No new compiler warnings
- [ ] Commit messages follow conventional format
- [ ] Branch is up-to-date with main

## Related Issues

<!-- Link related issues and PRs -->

- Closes #
- Related to #
- Depends on #

## Additional Notes

<!-- Any other information that reviewers should know -->

---

<!-- Thank you for contributing to Conductor! -->
