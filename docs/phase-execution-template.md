# Phase Execution Template

This template provides a structured prompt for executing any MIDIMon phase with proper dependency tracking and workflow management.

---

## Template Usage

Replace `[PHASE_NUMBER]`, `[PHASE_NAME]`, `[EPIC_ID]`, and other bracketed placeholders with actual values for the phase you're executing.

---

# Execute MIDIMon Phase [PHASE_NUMBER]: [PHASE_NAME]

## Context
- **Phase**: [PHASE_NUMBER] of 5
- **Epic**: [EPIC_ID] ([PHASE_NAME])
- **Status**: [Planned/In Progress]
- **Duration**: [X] weeks
- **Total Issues**: [N] issues
- **Target Version**: [vX.X.X]

## Prerequisites Check
Before starting, verify:
- [ ] Previous phase ([EPIC_ID]) marked complete
- [ ] All blocking dependencies resolved (see Dependency Documents below)
- [ ] Documentation site infrastructure ready (AMI-258 complete)
- [ ] Team has capacity for [X] weeks of work

## Dependency Documents
- Read @docs/implementation-roadmap.md ([PHASE_NAME] section, lines [START-END])
- Read @docs/linear-dependencies.md ([PHASE_NAME] dependencies)
- Read @docs/traceability-matrix.md (features requiring attention)
- Read [EPIC_ID] in Linear for detailed objectives

## Execution Instructions

### Step 1: Phase Analysis
1. **List all child issues** of [EPIC_ID] from Linear
2. **Extract dependency relationships** from docs/linear-dependencies.md
3. **Create dependency-ordered work queue** (topological sort)
4. **Identify parallel work opportunities** (issues with no shared dependencies)
5. **Highlight critical path** (longest dependency chain to phase completion)

**Output Format**:
```
Phase [PHASE_NUMBER] Work Queue:

Priority 0 (Start Immediately):
- [ISSUE_ID]: [Title] - No dependencies, blocks [X] issues
- [ISSUE_ID]: [Title] - No dependencies, independent

Priority 1 (After P0 Complete):
- [ISSUE_ID]: [Title] - Depends on [BLOCKER_ID], blocks [X] issues

Priority 2 (After P1 Complete):
- [ISSUE_ID]: [Title] - Depends on [BLOCKER_ID]

Parallel Groups:
Group A: [ISSUE_ID], [ISSUE_ID], [ISSUE_ID] (can be done simultaneously)
Group B: [ISSUE_ID], [ISSUE_ID] (can be done simultaneously)

Critical Path: [ISSUE_ID] → [ISSUE_ID] → [ISSUE_ID] ([X] days total)
```

### Step 2: Create Execution Plan
Present a structured plan showing:

#### Ready to Start (No Blockers)
- List issues that can begin immediately
- Note which ones can be done in parallel
- Estimate: [X] days

#### Blocked Issues
- List issues and what they're waiting for
- Estimate when blockers will clear
- Alternative work if blocked

#### Parallel Work Groups
- Group 1: [Issues that can be done together]
- Group 2: [Issues that can be done together]
- Rationale: [Why these can be parallel]

#### Critical Path Analysis
- Longest dependency chain: [X] days
- Bottleneck issues: [List issues that block the most]
- Risk factors: [Dependencies with uncertainty]

#### Estimated Timeline
- Optimistic: [X] days (if parallel work maximized)
- Realistic: [Y] days (accounting for dependencies)
- Pessimistic: [Z] days (if blockers encountered)
- **Target Completion**: [Date]

### Step 3: Execute Work (After User Approval)

For each issue in dependency order:

#### Pre-Execution Check
- [ ] All dependencies resolved (check Linear status)
- [ ] Required context gathered (specs, designs, references)
- [ ] Documentation site structure ready for updates

#### Execution
1. **Update Linear**: Set issue status to "In Progress"
2. **Do the work**:
   - Create specifications (if documentation issue)
   - Write code (if implementation issue)
   - Write tests (if testing issue)
   - Create designs (if planning issue)
3. **Update docs-site/**: Per Documentation Site Update Policy
   - Update relevant pages in docs-site/src/
   - Add examples to configuration/ or reference/
   - Update troubleshooting if applicable
4. **Verify**: Run `cd docs-site && mdbook build` to verify no errors
5. **Validate Definition of Done** (REQUIRED before marking "Done"):
   - [ ] All acceptance criteria from Linear issue are met
   - [ ] All success criteria checkboxes are completed
   - [ ] All required deliverables are created and committed
   - [ ] Tests pass locally (if code changes)
   - [ ] Test coverage ≥80% with 100% tests passing (if code changes)
   - [ ] All GitHub Actions pass (build, lint, test, security, docs)
   - [ ] Reviewed GitHub Actions logs for warnings or issues
   - [ ] Documentation site updated per policy
   - [ ] Code/content reviewed (self-review minimum, PR review ideal)
   - [ ] No known critical bugs or incomplete implementations
   - [ ] Manually verified the implementation works as specified
6. **Update Linear**: Set issue status to "Done" (only after validation)
7. **Report Progress**: Log completion and next steps

#### Post-Execution
- Unblock dependent issues
- Update work queue
- Report progress to user

### Step 4: Phase Completion

When all [N] issues complete:

#### Verification Checklist
- [ ] All [N] child issues of [EPIC_ID] marked "Done"
- [ ] All success criteria met (see [EPIC_ID] in Linear)
- [ ] Test coverage targets achieved
- [ ] Documentation site fully updated with phase changes
- [ ] No critical bugs or blockers remain
- [ ] Performance metrics within targets

#### Documentation Updates
- [ ] Update docs/traceability-matrix.md with phase completion
- [ ] Update docs/implementation-roadmap.md progress tracking
- [ ] Update [EPIC_ID] description with actual completion date
- [ ] Create phase completion summary

#### Phase Closeout
1. **Update Linear**: Set [EPIC_ID] status to "Done"
2. **Create Git Tag**: `git tag -a [VERSION_TAG] -m "[PHASE_NAME] Complete"`
3. **Phase Review Meeting**: Schedule with stakeholders
4. **Lessons Learned**: Document what went well/poorly
5. **Next Phase Kickoff**: Prepare Phase [N+1] execution

## Documentation Site Policy

**CRITICAL**: All issues MUST update docs-site/ per policy in docs/implementation-roadmap.md lines 159-219

### Standard Updates Required
- **New Triggers**: Update configuration/triggers.md and reference/trigger-types.md
- **New Actions**: Update configuration/actions.md and reference/action-types.md
- **New Features**: Update getting-started/, configuration/, or devices/ as appropriate
- **Architecture Changes**: Update development/architecture.md

### Definition of Done Template
```markdown
### Documentation Site Updates
- [ ] Updated relevant pages in docs-site/ with feature documentation
- [ ] Added configuration examples to docs-site/src/configuration/
- [ ] Updated reference documentation if new trigger/action types added
- [ ] Added troubleshooting section if applicable
- [ ] Verified mdbook build succeeds locally (`cd docs-site && mdbook build`)
- [ ] Reviewed generated HTML for formatting and clarity
```

## Progress Reporting

After each issue completion, report:

```
Phase [PHASE_NUMBER] Progress Report
=====================================
Date: [YYYY-MM-DD]
Elapsed: [X] days / [Y] total

Completed: [X/N] issues
- [ISSUE_ID]: [Title] ✅
- [ISSUE_ID]: [Title] ✅

In Progress: [List]
- [ISSUE_ID]: [Title] - [% complete]

Blocked: [List with blockers]
- [ISSUE_ID]: [Title] - Blocked by [BLOCKER_ID]

Next Up: [Next 3 issues in queue]
1. [ISSUE_ID]: [Title] - Ready to start
2. [ISSUE_ID]: [Title] - Depends on [ISSUE_ID]
3. [ISSUE_ID]: [Title] - Depends on [ISSUE_ID]

Estimated Completion: [Date] ([on track/at risk/delayed])

Risks/Blockers: [Any concerns]
```

## Questions to Confirm

Before beginning execution, confirm:

1. **Approach**: Should I proceed with Step 1 (Analyze Phase)?
2. **Approval Gate**: Do you want to review the execution plan before I start work?
3. **Execution Mode**:
   - Sequential (one issue at a time, safer)
   - Parallel (multiple issues simultaneously, faster)
   - Hybrid (parallel where possible, sequential for critical path)
4. **Reporting Frequency**: How often should I report progress?
   - After each issue
   - Daily summary
   - Weekly summary
5. **Autonomy Level**:
   - Full autonomy (execute all issues automatically)
   - Issue-by-issue approval (confirm before each issue)
   - Group approval (confirm before each parallel group)

## Emergency Procedures

### If Blocked
1. Document the blocker clearly
2. Identify alternative work (non-blocked issues)
3. Escalate if blocker is external (needs user action)
4. Consider workarounds or temporary solutions

### If Scope Changes
1. Pause execution
2. Document scope change request
3. Update work queue and timeline
4. Get user approval before continuing

### If Quality Issues Found
1. Stop and report the issue
2. Assess impact on dependent work
3. Create fix-it issue if needed
4. Adjust timeline if significant

---

## Phase-Specific Customization

When using this template, customize these sections based on the phase:

### Phase 0: Current State Preservation
- Focus on documentation and tagging
- Prerequisites: None (first phase)
- Main work: Git tagging, open source setup, documentation

### Phase 1: Documentation & Test Coverage
- Focus on specifications and testing
- Prerequisites: Phase 0 complete, docs-site infrastructure ready
- Main work: Complete feature specs, achieve 85% test coverage

### Phase 2: Core Engine Extraction
- Focus on refactoring and API design
- Prerequisites: Phase 1 complete, all specs finalized
- Main work: Create workspace, extract midimon-core

### Phase 3: Daemon & Config Hot-Reload
- Focus on service architecture
- Prerequisites: Phase 2 complete, core API stable
- Main work: Daemon, IPC, config watching, menu bar

### Phase 4: Tauri UI & Visual Configuration
- Focus on GUI development
- Prerequisites: Phase 3 complete, daemon working
- Main work: Tauri app, MIDI Learn, per-app profiles

### Phase 5: Advanced Features
- Focus on community features
- Prerequisites: Phase 4 complete, v2.0 stable
- Main work: Virtual MIDI, plugins, marketplace

---

## Template Version
- **Version**: 1.0
- **Last Updated**: 2025-11-11
- **Maintained By**: MIDIMon Project Team
- **Location**: docs/phase-execution-template.md

## Related Documents
- docs/implementation-roadmap.md - Overall project roadmap
- docs/linear-dependencies.md - Dependency graph
- docs/traceability-matrix.md - Feature tracking
- docs/PRD-main.md - Product requirements

---

**End of Template**
