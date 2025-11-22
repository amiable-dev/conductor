# Linear Dependencies Configuration

This document defines all dependencies between issues in the MIDIMon Linear workspace to enforce phase gates and proper sequencing.

## Phase Gate Dependencies

These dependencies enforce that phases complete in order:

### Phase Progression
```
Phase 0 (AMI-104)
    ↓ blocks
Phase 1 (AMI-105)
    ↓ blocks
Phase 2 (AMI-106)
    ↓ blocks
Phase 3 (AMI-107)
    ↓ blocks
Phase 4 (AMI-108)
    ↓ blocks
Phase 5 (AMI-109)
```

**Instructions**: In Linear, set the following "Blocked by" relationships:
- AMI-105 is blocked by AMI-104
- AMI-106 is blocked by AMI-105
- AMI-107 is blocked by AMI-106
- AMI-108 is blocked by AMI-107
- AMI-109 is blocked by AMI-108

## Phase 1 Internal Dependencies

### Planning Issues Block Phase 2
These planning issues must complete before Phase 2 can start:

```
AMI-123 (midimon-core API design) → blocks AMI-106
AMI-124 (Workspace structure design) → blocks AMI-106
AMI-125 (Backward compatibility strategy) → blocks AMI-106
AMI-126 (Phase 2 migration checklist) → blocks AMI-106
```

**Instructions**: Set AMI-106 (Phase 2) as blocked by AMI-123, AMI-124, AMI-125, AMI-126

### Testing Infrastructure Dependencies
The device simulator unblocks other testing work:

```
AMI-121 (Device simulator) → blocks AMI-117, AMI-118, AMI-119, AMI-120
```

**Instructions**: Set AMI-117, AMI-118, AMI-119, AMI-120 as blocked by AMI-121

### Test Coverage Tracking
Coverage tracking enables monitoring other test issues:

```
AMI-122 (Coverage tracking) → monitors AMI-117, AMI-118, AMI-119, AMI-120
```

**Instructions**: Set AMI-122 as a related issue to all test issues (not strictly blocking, but should be set up early)

## Feature Epic Dependencies

### Specification-Complete Requirement for Phase 2
All features with missing specs must be completed before Phase 2 refactoring:

```
Epic 1 Features (AMI-80, AMI-81, AMI-82, AMI-83) → block AMI-106
Epic 2 Features (AMI-85, AMI-86, AMI-87, AMI-88, AMI-89) → block AMI-106
Epic 3 Features (AMI-91, AMI-92, AMI-93, AMI-94, AMI-95) → block AMI-106
```

**Instructions**: Set AMI-106 (Phase 2) as blocked by all 14 reopened feature issues (AMI-80 through AMI-95)

**Rationale**: Cannot refactor code without complete specifications. Phase 1 ensures all features are fully documented before architectural changes.

## Feature Epic to Epic Dependencies

### Epic Relationships for v2.0 Critical Path

```
Epic 8: Visual Configuration & MIDI Learn (AMI-100)
    ↓ blocks
Epic 9: Configuration Management (AMI-101)
    ↓ blocks
Epic 10: Per-App Context Awareness (AMI-102)
```

**Instructions**: In Linear, set:
- AMI-101 is blocked by AMI-100
- AMI-102 is blocked by AMI-101

**Rationale**:
- MIDI Learn (Epic 8) provides the UI foundation for configuration management
- Configuration Management (Epic 9) enables saving/loading profiles needed for per-app context
- Per-App Context (Epic 10) requires profile management to switch configs based on active app

## Phase 4 Feature Dependencies

These features must complete before Phase 4 (v2.0) can be considered done:

```
Epic 8 (AMI-100) → blocks AMI-108 (Phase 4)
TF1, TF3, TF4 issues → block AMI-108
```

**Instructions**: Once Epic 8 features are created as issues, set AMI-108 as blocked by Epic 8 and its critical features.

## Recurring Task Dependencies

The recurring documentation tasks depend on work being completed:

```
AMI-127 (Weekly traceability updates) → monitors specification issues
AMI-128 (Weekly roadmap updates) → monitors Phase 1 epic
```

**Instructions**: These are monitoring tasks, not strict blockers. Link as related issues for visibility.

## Summary of Dependencies to Set in Linear

### Critical Path (Must Set First)
1. **Phase Gates**:
   - AMI-105 blocked by AMI-104
   - AMI-106 blocked by AMI-105
   - AMI-107 blocked by AMI-106
   - AMI-108 blocked by AMI-107
   - AMI-109 blocked by AMI-108

2. **Phase 1 → Phase 2 Gate**:
   - AMI-106 blocked by AMI-123, AMI-124, AMI-125, AMI-126
   - AMI-106 blocked by all 14 feature issues (AMI-80 through AMI-95)

3. **Feature Epic Critical Path**:
   - AMI-101 blocked by AMI-100
   - AMI-102 blocked by AMI-101

### Supporting Dependencies
4. **Testing Infrastructure**:
   - AMI-117, AMI-118, AMI-119, AMI-120 blocked by AMI-121

5. **Related Issues** (for visibility, not strict blocking):
   - Link AMI-122 to all test issues
   - Link AMI-127 to specification issues
   - Link AMI-128 to AMI-105

## How to Set Dependencies in Linear

1. Open an issue in Linear
2. Click "Add relation" in the right sidebar
3. Select "Blocked by" (or "Blocks" from the opposite direction)
4. Search for and select the blocking issue
5. Save

## Verification Checklist

After setting all dependencies, verify:
- [ ] Phase 1 cannot start until Phase 0 tagged
- [ ] Phase 2 cannot start until Phase 1 complete (all 19 issues + 14 features)
- [ ] Phase 3 cannot start until Phase 2 complete
- [ ] Phase 4 cannot start until Phase 3 complete
- [ ] Phase 5 cannot start until Phase 4 complete
- [ ] Epic 9 cannot start until Epic 8 complete
- [ ] Epic 10 cannot start until Epic 9 complete
- [ ] Test issues blocked until device simulator ready

## Benefits of This Dependency Structure

1. **Phase Gates**: Prevents starting refactoring before documentation complete
2. **Clear Critical Path**: Highlights which issues block major milestones
3. **Risk Visibility**: Shows when Phase 2 is at risk if specs incomplete
4. **Parallel Work**: Non-blocking issues can proceed concurrently
5. **Realistic Timelines**: Linear roadmap view shows true completion dates based on dependencies

---

**Last Updated**: 2025-11-11
**Total Dependencies**: 28 blocking relationships + 5 related links
