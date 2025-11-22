# Phase Execution Guide

Quick reference for executing MIDIMon phases with proper dependency tracking.

## Files Created

1. **phase-execution-template.md** - Master template for any phase
2. **phase-0-execution.md** - Phase 0: Current State Preservation (Open Source Setup)
3. **phase-1-execution.md** - Phase 1: Documentation & Test Coverage
4. **phase-2-execution.md** - (To be created) Phase 2: Core Engine Extraction
5. **phase-3-execution.md** - (To be created) Phase 3: Daemon & Config Hot-Reload
6. **phase-4-execution.md** - (To be created) Phase 4: Tauri UI & Visual Configuration
7. **phase-5-execution.md** - (To be created) Phase 5: Advanced Features

## How to Use

### Quick Start (Copy & Paste)

To execute a phase, copy the relevant prompt from the phase file and paste it into Claude Code:

```bash
# For Phase 0 (current)
cat docs/phase-0-execution.md

# For Phase 1 (next)
cat docs/phase-1-execution.md
```

Then copy the entire content and use it as your prompt to Claude Code.

### Standard Workflow

1. **Verify Prerequisites**
   - Check previous phase is complete
   - Verify all blocking dependencies resolved
   - Review the "Prerequisites Check" section

2. **Customize if Needed**
   - Update issue numbers if they've changed
   - Adjust timeline estimates based on team capacity
   - Modify parallel groups based on available resources

3. **Execute Step 1: Analysis**
   ```
   Execute Step 1 from @docs/phase-[N]-execution.md:
   - List all child issues from Linear
   - Create dependency-ordered work queue
   - Present execution plan
   ```

4. **Review & Approve Plan**
   - Review the presented work queue
   - Confirm parallel work groups make sense
   - Approve before proceeding to execution

5. **Execute Step 3: Work**
   ```
   Proceed with Step 3: Execute all issues in dependency order
   - Update Linear statuses
   - Update docs-site/ for each issue
   - Report progress regularly
   ```

6. **Validate Completion (Definition of Done)**
   Before marking any issue as "Done" in Linear, MUST validate:
   - **Acceptance Criteria**: All acceptance criteria from Linear issue are met
   - **Success Criteria**: All success criteria checkboxes are completed
   - **Deliverables**: All required deliverables are created and committed
   - **Testing**: Tests pass locally (if code changes)
   - **Test Coverage**: Minimum 80% code coverage, 100% tests passing (if code changes)
   - **GitHub Actions**: All CI/CD checks pass (build, lint, test, security, docs)
   - **Documentation**: docs-site/ updated per Documentation Site Update Policy
   - **Review**: Code/content reviewed (self-review minimum, PR review ideal)
   - **Quality**: No known critical bugs or incomplete implementations
   - **Validation**: Manually verify the implementation works as specified

7. **Complete Step 4: Closeout**
   - Verify all completion criteria met
   - Update documentation
   - Create git tag
   - Schedule next phase kickoff

## Phase-Specific Prompts

### Phase 0: Open Source Setup (Current)

**Status**: In Progress
**Duration**: 1.5 weeks
**Issues**: 11

**Quick Prompt**:
```
Execute Phase 0 for MIDIMon using @docs/phase-0-execution.md:

1. Analyze all 11 open source setup issues (children of AMI-104)
2. Create dependency-ordered work queue following the documented dependency graph
3. Present execution plan with parallel work groups
4. Upon approval, execute issues in order updating Linear and docs-site/
5. Complete when all issues done and repository is public

Key priorities:
- AMI-247 (Repo Setup) must be first
- AMI-258 (Docs Infrastructure) blocks AMI-251 (Docs Content)
- AMI-252 (Governance) enables parallel work on AMI-253 and AMI-255

Confirm approach before starting.
```

### Phase 1: Documentation & Test Coverage (Next)

**Status**: Planned
**Duration**: 2 weeks
**Issues**: 19

**Quick Prompt**:
```
Execute Phase 1 for MIDIMon using @docs/phase-1-execution.md:

Prerequisites:
- Phase 0 (AMI-104) complete
- AMI-258 (Docs Infrastructure) ready
- Public repository operational

Key Strategy:
- Week 1: Complete all 7 feature specs + device simulator (AMI-121) in parallel
- Week 2: Write tests (blocked by simulator) + migration planning (blocks Phase 2)
- Target: 85%+ test coverage

Critical Dependencies:
- AMI-121 blocks AMI-117, 118, 119, 120 (test issues)
- AMI-123, 124, 125, 126 block Phase 2 start

Confirm approach before starting.
```

### Phase 2: Core Engine Extraction (Future)

**Status**: Planned
**Duration**: 3-4 weeks
**Issues**: 17

**Prerequisites**:
- Phase 1 complete (all specs done, 85%+ test coverage)
- AMI-123, 124, 125, 126 approved
- Migration architecture reviewed

**To Create**: Run this to generate the prompt:
```
Create docs/phase-2-execution.md based on:
- docs/phase-execution-template.md
- docs/implementation-roadmap.md Phase 2 section (lines 460-600)
- AMI-106 (Phase 2 Epic) in Linear
```

## Dependency Patterns

### Sequential Chain
When issues must be done in order:
```
A → B → C → D
```
Execute: A, wait for completion, then B, then C, then D

### Parallel Groups
When issues can be done simultaneously:
```
     ┌─ B
A ──┼─ C
     └─ D
```
Execute: A first, then B, C, D simultaneously

### Diamond Pattern
When issues converge:
```
     ┌─ B ─┐
A ──┤       ├─ D
     └─ C ─┘
```
Execute: A, then B and C in parallel, then D after both complete

### Critical Path
Focus on the longest dependency chain:
```
A → B → C → E  (5 days) ← Critical path
A → D → E      (3 days)
```
Prioritize B and C to avoid delaying E

## Tips for Success

### 1. Start with Analysis
Always run Step 1 (Analysis) first to understand dependencies before executing any work.

### 2. Use Parallel Work
Maximize efficiency by identifying issues that can be done simultaneously.

### 3. Monitor Blockers
Check blocker status frequently. If a blocker is delayed, pivot to alternative work.

### 4. Update Documentation
Never skip docs-site/ updates. They're required by the Documentation Site Update Policy.

### 5. Verify mdbook Builds
Run `cd docs-site && mdbook build` after every docs-site/ change to catch errors early.

### 6. Report Progress
Regular progress reports help identify issues early and keep stakeholders informed.

### 7. Don't Skip Prerequisites
Verify all prerequisites before starting a phase to avoid mid-phase blockers.

## Common Pitfalls

### ❌ Skipping Dependency Analysis
**Problem**: Starting work without understanding dependencies leads to blocked work and wasted effort.
**Solution**: Always complete Step 1 (Analysis) first.

### ❌ Working on Blocked Issues
**Problem**: Starting an issue before its dependencies are met leads to incomplete work.
**Solution**: Check Linear status of dependencies before starting any issue.

### ❌ Forgetting Documentation Updates
**Problem**: Completing code work but not updating docs-site/ violates the Documentation Site Update Policy.
**Solution**: Add "Update docs-site/" to every issue's checklist.

### ❌ Sequential Execution Only
**Problem**: Doing all issues sequentially wastes time when parallel work is possible.
**Solution**: Identify parallel work groups in Step 2 (Execution Plan).

### ❌ No Progress Reporting
**Problem**: Silent execution leaves stakeholders uninformed and makes debugging issues difficult.
**Solution**: Report progress after each issue completion or daily.

### ❌ Ignoring Blockers
**Problem**: Waiting for blocked issues instead of pivoting to alternative work wastes time.
**Solution**: Monitor blockers actively and switch to non-blocked work when needed.

## Integration with Linear

### Issue Status Flow
```
Backlog → Todo → In Progress → Done
```

### Using Issue Dependencies
Linear supports "Blocked by" relationships. Use them to:
1. Prevent starting work on blocked issues
2. Visualize dependency graphs
3. Track critical path automatically

### Labels to Use
- `phase-[N]`: Indicates which phase the issue belongs to
- `Feature`: User-facing functionality
- `Documentation`: Documentation updates
- `Testing`: Test coverage work
- `Planning`: Architecture and design work

## Reporting Templates

### Daily Progress Report
```
Phase [N] Daily Progress - [Date]
===================================
Completed Today:
- [ISSUE_ID]: [Title] ✅

In Progress:
- [ISSUE_ID]: [Title] - [X]% complete

Blocked:
- [ISSUE_ID]: [Title] - Blocked by [BLOCKER_ID] (ETA: [date])

Next Up:
- [ISSUE_ID]: [Title] - Starting tomorrow

Phase Progress: [X]/[N] issues complete ([X]%)
Estimated Completion: [Date] ([on track/at risk/delayed])
```

### Weekly Summary Report
```
Phase [N] Weekly Summary - Week of [Date]
==========================================
This Week:
- Completed: [X] issues
- Started: [Y] issues
- Blocked: [Z] issues

Key Accomplishments:
- [Major milestone]
- [Major milestone]

Risks/Issues:
- [Risk description and mitigation]

Next Week Plan:
- [What we'll focus on]
- [Expected completions]

Phase Progress: [X]/[N] issues complete ([X]%)
Estimated Completion: [Date] ([status])
```

## Questions & Support

### How do I handle scope changes?
1. Pause execution
2. Document the scope change
3. Update work queue and timeline
4. Get stakeholder approval
5. Resume execution

### What if I find a critical bug?
1. Create a new issue for the bug
2. Assess impact on current work
3. Decide: fix immediately or defer
4. Update timeline if significant
5. Document in progress report

### How do I handle dependencies on external teams?
1. Identify the external dependency clearly
2. Escalate to stakeholders
3. Find alternative work while waiting
4. Track dependency status
5. Adjust timeline if needed

### What if the estimate was wrong?
1. Update the estimate in the issue
2. Recalculate phase timeline
3. Inform stakeholders of delay
4. Identify opportunities to accelerate other work
5. Document lessons learned for future phases

---

## Quick Reference Card

### Phase Execution Checklist
- [ ] Read phase-[N]-execution.md
- [ ] Verify prerequisites complete
- [ ] Run Step 1: Analysis
- [ ] Review and approve execution plan
- [ ] Execute issues in dependency order
- [ ] Update Linear statuses
- [ ] Update docs-site/ per policy
- [ ] Report progress regularly
- [ ] Verify completion criteria met
- [ ] Complete phase closeout

### Key Commands
```bash
# Read phase execution guide
cat docs/phase-[N]-execution.md

# Verify docs-site builds
cd docs-site && mdbook build

# Check test coverage
cargo tarpaulin --workspace

# Create phase completion tag
git tag -a [VERSION] -m "Phase [N] Complete"
```

### Key Documents
- phase-execution-template.md - Master template
- phase-[N]-execution.md - Phase-specific guide
- implementation-roadmap.md - Overall roadmap
- linear-dependencies.md - Dependency graph
- traceability-matrix.md - Feature tracking

---

**Last Updated**: 2025-01-16
**Maintained By**: MIDIMon Project Team

---

## Phase Completion Status

| Phase | Version | Status | Completion Date | Description |
|-------|---------|--------|-----------------|-------------|
| Phase 0 | v0.1.0 | ✅ Complete | - | Baseline monolithic implementation |
| Phase 1 | - | ✅ Complete | - | Security fixes & test infrastructure (informal) |
| Phase 2 | v0.2.0 | ✅ Complete | 2025-11-12 | ActionExecutor extraction |
| **Phase 2.5** | **v2.0.1** | ✅ **Complete** | **2025-01-16** | **Security hardening (6 fixes, 36+ tests)** |
| Phase 3 | v1.0.0 | ✅ Complete | ~2025-11 | Daemon infrastructure (19 issues) |
| Phase 4 | v2.0.0 | ✅ Complete | 2025-11-14 | Tauri GUI & Visual Configuration |
| Phase 5 | - | ❌ Not Started | - | Advanced Features & Polish (planning) |

**Current Phase**: Phase 5 Planning
**Next Version**: v2.5.0 (estimated)

### Phase 2.5: Security Hardening (INSERTED)

Phase 2.5 was inserted after Phase 4 completion based on multi-agent code review findings. This demonstrates adaptive planning and quality-first delivery.

**Scope**:
- 5 security fixes (1 HIGH, 4 MEDIUM severity)
  1. Remove shell interpreter from execute_shell() - command injection prevention
  2. Add IPC request size limit (1MB) - DoS prevention
  3. Fix TOCTOU race in config save() - atomic write pattern
  4. Set secure file permissions (0600/0700) - information disclosure prevention
  5. User-specific socket paths - XDG compliance, socket hijacking prevention
- 1 architectural purity fix (CRITICAL)
  6. Remove enigo from core library - true UI-independence

**Results**:
- Security Score: 7.5 → 9.0 (+1.5)
- Architecture Score: 8.5 → 9.5 (+1.0)
- Combined Score: 9.25/10
- Test Coverage: 36+ new security tests, 485+ total tests passing
- Documentation: 52,000 words across 10 files

**Rationale for Insertion**: Multi-agent code review identified security vulnerabilities that needed addressing before public release. Rather than ship v2.0.0 with known issues, Phase 2.5 was inserted to maintain quality standards.

See `PHASE2.5_COMPLETE.md` for full implementation details.
