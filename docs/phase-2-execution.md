# Execute MIDIMon Phase 2: Core Engine Extraction (v1.0.0)

## Context
- **Phase**: 2 of 5
- **Epic**: AMI-106 (Core Engine Extraction)
- **Status**: Planned - Starts after Phase 1 complete
- **Duration**: 3-4 weeks
- **Total Issues**: 17 issues
- **Target Version**: v1.0.0

## Prerequisites Check
Before starting, verify:
- [ ] Phase 1 (AMI-105) marked complete in Linear
- [ ] All blocking dependencies resolved:
  - [ ] All feature specs complete (AMI-108 through AMI-116)
  - [ ] Test coverage ≥85% achieved
  - [ ] AMI-123 (API Design) approved
  - [ ] AMI-124 (Workspace Structure) approved
  - [ ] AMI-125 (Config Compatibility) approved
  - [ ] AMI-126 (Migration Checklist) reviewed
- [ ] Documentation site infrastructure ready (AMI-258 complete)
- [ ] v0.1.0-monolithic tag exists (safe rollback point)
- [ ] All Phase 1 tests passing (302 tests)
- [ ] Team has capacity for 3-4 weeks of work

## Dependency Documents

**CRITICAL - Read These First**:
- Read @docs/phase2-migration-checklist.md (1,892 lines - complete step-by-step guide)
- Read @docs/phase2-migration-guide.md (427 lines - technical migration steps)
- Read @docs/PHASE2_CHECKLIST_SUMMARY.md (266 lines - quick reference)
- Read @docs/api-design.md (AMI-123 - public API specification)
- Read @docs/workspace-structure.md (AMI-124 - directory layout)
- Read @docs/config-compatibility.md (AMI-125 - backward compatibility)
- Read @docs/implementation-roadmap.md (Phase 2 section, lines 460-600)
- Read @docs/linear-dependencies.md (Phase 2 dependencies)
- Read @docs/traceability-matrix.md (features requiring attention)
- Read AMI-106 in Linear for detailed objectives

## Phase 2 Architecture Overview

### Before (Phase 1 - Monolithic Binary)
```
midimon/
├── Cargo.toml
├── src/
│   ├── main.rs
│   ├── config.rs
│   ├── event_processor.rs
│   ├── mappings.rs
│   ├── actions.rs
│   ├── feedback.rs
│   ├── mikro_leds.rs
│   ├── midi_feedback.rs
│   └── device_profile.rs
└── config.toml
```

### After (Phase 2 - Workspace Structure)
```
midimon/
├── Cargo.toml                      # Workspace root
├── midimon-core/                   # Pure Rust engine crate (UI-free)
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs                  # Public API exports
│       ├── engine.rs               # MidiMonEngine (public)
│       ├── config.rs               # Config types (public)
│       ├── events.rs               # Event types (public)
│       ├── actions.rs              # Action types/executor (public)
│       ├── mapping.rs              # Mapping engine (public)
│       ├── feedback.rs             # Feedback traits (public)
│       ├── device.rs               # Device profiles (public)
│       ├── error.rs                # Error types (public)
│       ├── event_processor.rs     # State machine (PRIVATE)
│       ├── timing.rs               # Timing detection (PRIVATE)
│       ├── velocity.rs             # Velocity processing (PRIVATE)
│       ├── chord.rs                # Chord detection (PRIVATE)
│       ├── mikro_leds.rs           # HID LED impl (PRIVATE)
│       └── midi_feedback.rs        # MIDI LED impl (PRIVATE)
├── midimon-daemon/                 # CLI binary (current functionality)
│   ├── Cargo.toml
│   └── src/
│       ├── main.rs                 # CLI entry point
│       └── bin/                    # Diagnostic tools
│           ├── midi_diagnostic.rs
│           ├── led_diagnostic.rs
│           └── ...
└── config/
    └── default.toml                # Default config template
```

## Execution Instructions

### Step 1: Phase Analysis

1. **List all child issues** of AMI-106 from Linear
2. **Extract dependency relationships** from docs/phase2-migration-checklist.md and docs/linear-dependencies.md
3. **Create dependency-ordered work queue** (topological sort based on 10-step checklist)
4. **Identify parallel work opportunities** (issues with no shared dependencies)
5. **Highlight critical path** (longest dependency chain to phase completion)

**Output Format**:
```
Phase 2 Work Queue (Based on phase2-migration-checklist.md):

Priority 0 - Preparation Phase (MUST BE FIRST):
- Step 1.1-1.6: Pre-Migration Tasks (4-6 hours)
  - Backup repository
  - Verify current tests pass
  - Create v0.1.0-monolithic tag
  - Review dependency documentation
  - Setup development environment
  - Create feature branch (phase-2/workspace-migration)

Priority 1 - Workspace Creation (After Preparation):
- Step 2.1-2.5: Workspace Creation (1-2 hours)
  - Create workspace root Cargo.toml
  - Create directory structure
  - Create midimon-core/Cargo.toml
  - Create midimon-daemon/Cargo.toml
  - Verify workspace build

Priority 2 - Core Library Extraction (After Workspace Created):
- Step 3.1-3.6: Core Library Extraction (4-6 hours)
  - Copy core modules to midimon-core
  - Create new core modules (error.rs, events.rs, engine.rs)
  - Create lib.rs public API
  - Remove UI dependencies (colored, chrono)
  - Fix import paths
  - Verify core library builds

Priority 3 - CLI Daemon Creation (After Core Extraction):
- Step 4.1-4.4: CLI Daemon Creation (2-3 hours)
  - Create CLI main.rs using midimon-core
  - Move diagnostic tools
  - Update diagnostic tool imports
  - Verify daemon builds

Priority 4 - Module Migration (After CLI Created):
- Step 5.1-5.6: Module Migration (3-4 hours)
  - Migrate event_processor.rs
  - Migrate config.rs to public API
  - Migrate actions.rs to public API
  - Migrate feedback.rs to public API
  - Migrate device.rs to public API
  - Migrate mapping.rs to public API

Priority 5 - API Implementation (After Modules Migrated):
- Step 6.1-6.4: API Implementation (2-3 hours)
  - Implement MidiMonEngine struct
  - Implement Config methods
  - Implement error type conversions
  - Add doc comments to public API

Priority 6 - Testing (After API Implemented):
- Step 7.1-7.6: Testing (3-4 hours)
  - Unit tests: Core library
  - Unit tests: Daemon binary
  - Integration tests: Config loading
  - Integration tests: Engine creation
  - Backward compatibility tests
  - Feature parity test (all 26 features)

Priority 7 - Validation (After Testing):
- Step 8.1-8.6: Validation & Verification (2-3 hours)
  - Build verification
  - Dependency check
  - Clippy & formatting
  - Documentation generation
  - Full test suite
  - Config compatibility test

Priority 8 - Documentation (After Validation):
- Step 9.1-9.5: Documentation Updates (2-3 hours)
  - Update README.md
  - Update CLAUDE.md
  - Update architecture docs
  - Create migration guide
  - Update API documentation

Priority 9 - Completion (Final Step):
- Step 10.1-10.6: Git & Completion (1-2 hours)
  - Commit all changes
  - Create v0.2.0-workspace tag
  - Create release notes
  - Push to remote
  - Create pull request
  - Merge to main

Parallel Work Groups:
- NONE - Phase 2 is sequential due to tight dependencies
- However, within each step (e.g., module migration), individual modules can be done simultaneously

Critical Path: Steps 1 → 2 → 3 → 4 → 5 → 6 → 7 → 8 → 9 → 10 (24-36 hours total)
```

### Step 2: Create Execution Plan

Present a structured plan showing:

#### Phase 2 Timeline (3-4 weeks)

**Week 1: Foundation & Workspace Setup**
- Days 1-2: Preparation Phase (Steps 1.1-1.6)
- Days 3-4: Workspace Creation (Steps 2.1-2.5)
- Day 5: Core Library Extraction begins (Steps 3.1-3.3)
- Estimate: 5 days

**Week 2: Core Extraction & CLI**
- Days 1-2: Complete Core Library Extraction (Steps 3.4-3.6)
- Days 3-4: CLI Daemon Creation (Steps 4.1-4.4)
- Day 5: Module Migration begins (Step 5.1-5.2)
- Estimate: 5 days

**Week 3: API Implementation & Testing**
- Days 1-2: Complete Module Migration (Steps 5.3-5.6)
- Days 3-4: API Implementation (Steps 6.1-6.4)
- Day 5: Testing begins (Steps 7.1-7.3)
- Estimate: 5 days

**Week 4: Validation & Completion**
- Days 1-2: Complete Testing (Steps 7.4-7.6)
- Days 3-4: Validation & Documentation (Steps 8.1-9.5)
- Day 5: Git & Completion (Step 10.1-10.6)
- Estimate: 5 days

#### Ready to Start (No Blockers)
- Step 1.1: Backup repository (30 min)
- Step 1.2: Verify current tests pass (30 min)
- Step 1.3: Create v0.1.0-monolithic tag (15 min)
- All other steps are sequential and blocked by previous steps

#### Blocked Issues
- Steps 2-10 are all blocked by completion of previous steps
- NO parallel work possible due to tight coupling
- Must complete each step before proceeding to next

#### Phase 1 Recommendations Integration
Based on Phase 1 lessons learned, this execution plan incorporates:

**Technical**:
- ✅ Start with Core Extraction: Steps 3-5 focus on extracting pure logic
- ✅ Maintain Backward Compatibility: Step 8.6 explicitly tests config compatibility
- ✅ Test Incrementally: Every step includes validation commands
- ✅ Use Feature Flags: Not needed for Phase 2 (clean extraction)

**Process**:
- ✅ Smaller Pull Requests: Each major step (1-10) can be a separate PR
- ✅ Continuous Integration: Validation after every step
- ✅ Documentation Updates: Step 9 dedicated to documentation
- ✅ Test Coverage Maintenance: Step 7.1 requires ≥85% coverage

#### Critical Path Analysis
- Longest dependency chain: 24-36 hours (all 10 steps sequential)
- Bottleneck issues:
  - Step 3 (Core Library Extraction) - 4-6 hours, blocks all downstream work
  - Step 7 (Testing) - 3-4 hours, critical for validation
  - Step 8 (Validation) - 2-3 hours, must pass before merge
- Risk factors:
  - Import path errors during module migration (Step 5)
  - Backward compatibility breaks (Step 7.5)
  - Test coverage regression (Step 7.1)

#### Estimated Timeline
- Optimistic: 24 hours (if all steps go smoothly, full-time work)
- Realistic: 30 hours (accounting for debugging and fixes)
- Pessimistic: 36 hours (if significant issues encountered)
- **Target Completion**: 2025-12-30 (3-4 weeks part-time)

### Step 3: Execute Work (After User Approval)

For each step in the 10-step checklist (docs/phase2-migration-checklist.md):

#### Pre-Execution Check
- [ ] All dependencies resolved (check previous step complete)
- [ ] Required context gathered (checklist section read)
- [ ] Documentation site structure ready for updates
- [ ] Git working directory clean

#### Execution (Follow phase2-migration-checklist.md exactly)

**For Preparation Phase (Step 1)**:
1. **Update Linear**: Set AMI-106 status to "In Progress"
2. **Execute checklist items 1.1-1.6**:
   ```bash
   # 1.1 - Backup Repository (30 min)
   cp -r /Users/christopherjoseph/projects/amiable/midimon ~/midimon-backup-v0.1.0

   # 1.2 - Verify Current Tests Pass (30 min)
   cargo test --all
   cargo test --release

   # 1.3 - Create v0.1.0 Preservation Tag (15 min)
   git add -A
   git commit -m "Preserve v0.1.0 state before Phase 2 migration"
   git tag -a v0.1.0-monolithic -m "MIDIMon v0.1.0 - Monolithic Implementation"
   git push origin v0.1.0-monolithic

   # 1.4 - Review Dependency Documentation (1 hour)
   # Read all Phase 2 planning documents

   # 1.5 - Setup Development Environment (30 min)
   rustup update
   cargo install cargo-watch cargo-tree

   # 1.6 - Create Feature Branch (10 min)
   git checkout -b phase-2/workspace-migration
   git push -u origin phase-2/workspace-migration
   ```
3. **Verify**: Check all items in Step 1 checklist
4. **Report Progress**: "Step 1 (Preparation) complete - 4-6 hours"

**For Workspace Creation (Step 2)**:
1. **Execute checklist items 2.1-2.5**:
   ```bash
   # 2.1 - Create Workspace Root Manifest (30 min)
   mv Cargo.toml Cargo.toml.v0.1.0-backup
   # Create new workspace Cargo.toml (see checklist lines 161-196)

   # 2.2 - Create Directory Structure (30 min)
   mkdir -p midimon-core/src midimon-core/tests
   mkdir -p midimon-daemon/src/bin midimon-daemon/tests
   mkdir -p config/examples config/device_templates

   # 2.3 - Create midimon-core/Cargo.toml (15 min)
   # See checklist lines 237-272

   # 2.4 - Create midimon-daemon/Cargo.toml (15 min)
   # See checklist lines 279-338

   # 2.5 - Verify Workspace Build (20 min)
   cargo metadata --format-version 1 | jq '.workspace_members'
   cargo tree --workspace
   ```
2. **Verify**: Workspace metadata shows both crates
3. **Report Progress**: "Step 2 (Workspace Creation) complete - 1-2 hours"

**For Core Library Extraction (Step 3)**:
1. **Execute checklist items 3.1-3.6** (see lines 373-672 in phase2-migration-checklist.md)
2. **Critical**: Remove colored/chrono from core (Step 3.4)
3. **Verify**: `cd midimon-core && cargo build`
4. **Report Progress**: "Step 3 (Core Extraction) complete - 4-6 hours"

**For CLI Daemon Creation (Step 4)**:
1. **Execute checklist items 4.1-4.4** (see lines 676-832)
2. **Verify**: All 7 binaries compile (midimon + 6 diagnostic tools)
3. **Report Progress**: "Step 4 (CLI Daemon) complete - 2-3 hours"

**For Module Migration (Step 5)**:
1. **Execute checklist items 5.1-5.6** (see lines 836-961)
2. **Critical**: Ensure all public API properly exported
3. **Verify**: `cargo doc --no-deps --open`
4. **Report Progress**: "Step 5 (Module Migration) complete - 3-4 hours"

**For API Implementation (Step 6)**:
1. **Execute checklist items 6.1-6.4** (see lines 965-1040)
2. **Critical**: MidiMonEngine must have all methods from api-design.md
3. **Verify**: All methods documented and compile
4. **Report Progress**: "Step 6 (API Implementation) complete - 2-3 hours"

**For Testing (Step 7)**:
1. **Execute checklist items 7.1-7.6** (see lines 1044-1167)
2. **Critical Success Criteria**:
   - All 302+ tests pass
   - Test coverage ≥85%
   - All 26 v0.1.0 features work identically
   - Config backward compatibility verified
3. **Verify**:
   ```bash
   cargo test --workspace
   cargo llvm-cov --workspace --html
   cargo run -p midimon-daemon --release -- 2
   ```
4. **Report Progress**: "Step 7 (Testing) complete - 3-4 hours"

**For Validation (Step 8)**:
1. **Execute checklist items 8.1-8.6** (see lines 1171-1311)
2. **Critical Quality Checks**:
   - Clean workspace build
   - No circular dependencies
   - cargo clippy passes with no warnings
   - cargo fmt --check passes
   - Documentation builds without errors
3. **Verify**:
   ```bash
   cargo clean && cargo build --workspace --release
   cargo clippy --all-targets --all-features -- -D warnings
   cargo fmt --all -- --check
   cargo doc --workspace --no-deps
   ```
4. **Report Progress**: "Step 8 (Validation) complete - 2-3 hours"

**For Documentation (Step 9)**:
1. **Execute checklist items 9.1-9.5** (see lines 1314-1398)
2. **Update docs-site/** per Documentation Site Update Policy:
   - Update docs-site/src/development/architecture.md with workspace structure
   - Add docs-site/src/api/ directory with core API documentation
   - Update docs-site/src/development/contributing.md with workspace commands
   - Create docs-site/src/development/migration-v1.md
3. **Verify**:
   ```bash
   cd docs-site && mdbook build
   mdbook serve --open
   ```
4. **Report Progress**: "Step 9 (Documentation) complete - 2-3 hours"

**For Completion (Step 10)**:
1. **Execute checklist items 10.1-10.6** (see lines 1402-1571)
2. **Create comprehensive commit**:
   ```bash
   git add -A
   git commit -m "Phase 2: Extract midimon-core workspace structure (AMI-126)"
   # See lines 1413-1461 for full commit message template
   ```
3. **Create completion tag**:
   ```bash
   git tag -a v0.2.0-workspace -m "MIDIMon Phase 2: Workspace Structure Migration"
   git push origin v0.2.0-workspace
   ```
4. **Create Pull Request** with Phase 2 completion summary
5. **Report Progress**: "Step 10 (Completion) complete - Phase 2 DONE"

#### Validation Definition of Done (REQUIRED for Phase 2 Completion)

Before marking AMI-106 as "Done" in Linear, MUST validate:

**Phase-Level Criteria**:
- [ ] All 17 child issues of AMI-106 marked "Done" in Linear
- [ ] All 10 steps in phase2-migration-checklist.md complete
- [ ] Workspace structure exists with midimon-core and midimon-daemon
- [ ] All public API from api-design.md exported in midimon-core/src/lib.rs
- [ ] No colored/chrono dependencies in midimon-core

**Testing Criteria**:
- [ ] All tests pass (unit, integration, compatibility): cargo test --workspace
- [ ] Test coverage ≥85%: cargo llvm-cov --workspace
- [ ] All 26 features working identically to v0.1.0
- [ ] All diagnostic tools compile and run
- [ ] Config backward compatibility verified (v0.1.0 config.toml loads without errors)

**Quality Criteria**:
- [ ] Binary size within 5% of v0.1.0 (3-5MB)
- [ ] cargo fmt --check passes (no formatting issues)
- [ ] cargo clippy passes with no warnings
- [ ] cargo doc --workspace --no-deps generates without errors
- [ ] No circular dependencies: cargo tree --workspace shows clean graph

**Documentation Criteria**:
- [ ] docs-site/ updated per Documentation Site Update Policy
- [ ] docs-site/src/development/architecture.md shows workspace structure
- [ ] docs-site/src/api/ contains midimon-core API documentation
- [ ] docs-site/src/development/migration-v1.md created
- [ ] mdbook build succeeds: cd docs-site && mdbook build
- [ ] README.md and CLAUDE.md updated with workspace commands

**Git Criteria**:
- [ ] v0.1.0-monolithic tag exists (preservation/rollback point)
- [ ] v0.2.0-workspace tag created (completion marker)
- [ ] All changes committed to phase-2/workspace-migration branch
- [ ] Pull request created and approved
- [ ] Merged to main branch

**Validation Process**:
1. Read phase2-migration-checklist.md completely
2. Check off each item in Steps 1-10
3. Verify all success criteria in AMI-106
4. Run full validation suite locally
5. Push and verify all GitHub Actions pass
6. Review GitHub Actions logs for warnings
7. Test the CLI manually with existing config.toml
8. Only then mark AMI-106 as "Done" in Linear

#### Post-Execution
- Unblock Phase 3 issues (AMI-107)
- Update implementation-roadmap.md with actual completion date
- Schedule Phase 3 kickoff meeting
- Document lessons learned in docs/phase-2-retrospective.md

### Step 4: Phase Completion

When all 10 steps complete (Phase 2 checklist Steps 1-10):

#### Verification Checklist
- [ ] All 17 child issues of AMI-106 marked "Done" in Linear
- [ ] All success criteria met (see AMI-106 in Linear)
- [ ] Test coverage ≥85% (cargo llvm-cov --workspace)
- [ ] Documentation site fully updated with workspace structure
- [ ] No critical bugs or blockers remain
- [ ] Performance metrics within targets (<1ms latency, <15MB memory)
- [ ] Binary size ≤5MB (release build)

#### Documentation Updates
- [ ] Update docs/traceability-matrix.md with Phase 2 completion
- [ ] Update docs/implementation-roadmap.md progress tracking
- [ ] Update AMI-106 description with actual completion date
- [ ] Create docs/phase-2-retrospective.md with lessons learned
- [ ] Update docs-site/src/development/architecture.md with final structure
- [ ] Create docs/phase-2-release-notes.md

#### Phase Closeout
1. **Update Linear**: Set AMI-106 status to "Done"
2. **Create Git Tag**: `git tag -a v1.0.0 -m "Phase 2: Core Engine Extraction Complete"`
3. **Phase Review Meeting**: Schedule with stakeholders
4. **Lessons Learned**: Document in docs/phase-2-retrospective.md:
   - What went well
   - What could be improved
   - Recommendations for Phase 3
5. **Next Phase Kickoff**: Prepare Phase 3 execution (AMI-107)
6. **Celebrate**: 🎉 Major milestone achieved!

## Documentation Site Policy

**CRITICAL**: All Phase 2 work MUST update docs-site/ per policy in docs/implementation-roadmap.md lines 159-219

### Standard Updates Required

**Architecture Changes** (Steps 2-5):
- Update docs-site/src/development/architecture.md with workspace structure
- Add diagrams showing midimon-core vs midimon-daemon
- Document public API surface

**New Public APIs** (Step 6):
- Create docs-site/src/api/ directory
- Add docs-site/src/api/engine.md (MidiMonEngine)
- Add docs-site/src/api/config.md (Config types)
- Add docs-site/src/api/events.md (MidiEvent, ProcessedEvent)
- Add docs-site/src/api/actions.md (Action types)
- Add docs-site/src/api/feedback.md (FeedbackController trait)

**Migration Guide** (Step 9):
- Create docs-site/src/development/migration-v1.md
- Document how to use midimon-core in other projects
- Provide examples of embedding the engine
- Explain backward compatibility guarantees

**Development Workflow** (Step 9):
- Update docs-site/src/development/contributing.md with workspace commands
- Add examples: cargo build -p midimon-core, cargo run -p midimon-daemon
- Document how to test individual crates

### Definition of Done Template
```markdown
### Documentation Site Updates (Phase 2)
- [ ] Updated docs-site/src/development/architecture.md with workspace structure
- [ ] Created docs-site/src/api/ directory with core API documentation
- [ ] Created docs-site/src/development/migration-v1.md with migration guide
- [ ] Updated docs-site/src/development/contributing.md with workspace workflow
- [ ] Added code examples to docs-site/src/examples/embedding-core.md
- [ ] Verified mdbook build succeeds locally (`cd docs-site && mdbook build`)
- [ ] Reviewed generated HTML at docs-site/book/index.html
- [ ] Verified all internal links work (no broken links)
```

## Progress Reporting

After each major step completion, report:

```
Phase 2 Progress Report
=======================
Date: [YYYY-MM-DD]
Elapsed: [X] hours / 36 total

Completed Steps:
- Step 1: Preparation Phase ✅ (4-6 hours)
- Step 2: Workspace Creation ✅ (1-2 hours)
- Step 3: Core Library Extraction ✅ (4-6 hours)

Current Step:
- Step 4: CLI Daemon Creation - 60% complete (1 hour elapsed)

Blocked: None

Next Up:
- Step 4.3: Update diagnostic tool imports
- Step 4.4: Verify daemon builds
- Step 5: Module Migration (after Step 4 complete)

Estimated Completion: 2025-12-30 (on track)

Risks/Blockers: None identified

Test Coverage Status:
- Current: 85.2% (target: ≥85%)
- Tests Passing: 302/302 (100%)

Documentation Status:
- Architecture docs: In progress
- API docs: Not started
- Migration guide: Not started
```

## Rollback Procedures

### If Compilation Fails (During Steps 2-6)
```bash
# Option 1: Revert uncommitted changes
git checkout .

# Option 2: Reset to v0.1.0-monolithic tag
git reset --hard v0.1.0-monolithic
git clean -fd

# Diagnose issue:
cargo check 2>&1 | head -50

# Common issues:
# - Missing module declarations in lib.rs
# - Incorrect import paths (use crate:: not super::)
# - Visibility issues (missing pub)

# Fix and retry:
cargo check
```

### If Tests Fail (During Step 7)
```bash
# Run tests with verbose output
cargo test --workspace -- --nocapture 2>&1 | head -100

# Common issues:
# 1. Import errors: Check module paths
#    Fix: Update imports to reference correct modules
# 2. Missing items: Check pub visibility
#    Fix: Add 'pub' to exported structs/functions
# 3. Type errors: Check API compatibility
#    Fix: Reference api-design.md for correct types

# Run specific test:
cargo test -p midimon-core config::tests::test_load

# Retry all tests:
cargo test --workspace
```

### If Backward Compatibility Breaks (During Step 7.5)
```bash
# Load v0.1.0 config to diagnose
cargo run -p midimon-daemon -- 2

# If config.toml doesn't load:
# 1. Check config parsing in midimon-core/src/config.rs
# 2. Verify all Trigger and Action types still parsed
# 3. Check AdvancedSettings optional defaults

# Ensure fields are optional with #[serde(default)]
# Example:
#[derive(Deserialize)]
pub struct AdvancedSettings {
    #[serde(default = "default_chord_timeout")]
    pub chord_timeout_ms: u64,
}

# Retry config loading:
cargo test -p midimon-core config::tests
```

### If Binary Size Increases (During Step 8.1)
```bash
# Compare binary sizes
ls -lh target/release/midimon*
# Should be ±5% of v0.1.0 size (3-5MB)

# If larger:
cargo build --release
strip -x target/release/midimon
ls -lh target/release/midimon

# Check for duplicate dependencies
cargo tree --duplicates
```

### Full Rollback (Emergency Only)
```bash
# 1. Create safe point
git tag v0.2.0-rollback-attempt

# 2. Revert to known-good state
git reset --hard v0.1.0-monolithic

# 3. Create incident report
cat > docs/phase-2-incident.md << 'EOF'
# Phase 2 Rollback Incident

## What Failed
[Describe critical issue]

## When
[Date/time of discovery]

## Impact
[Who was affected]

## Root Cause
[Technical explanation]

## Lessons Learned
[What to do differently]

## Next Steps
[Plan for retry]
EOF

# 4. Schedule retrospective
# Contact team for post-mortem meeting
```

## Questions to Confirm

Before beginning execution, confirm:

1. **Approach**: Should I proceed with Step 1 (Preparation Phase)?
2. **Approval Gate**: Do you want to review the execution plan before I start work?
3. **Execution Mode**:
   - Sequential (one step at a time, safer) ← RECOMMENDED for Phase 2
   - Parallel (not applicable - steps are sequential)
   - Hybrid (not applicable - steps are sequential)
4. **Reporting Frequency**: How often should I report progress?
   - After each step (10 reports) ← RECOMMENDED
   - Daily summary
   - Weekly summary
5. **Autonomy Level**:
   - Full autonomy (execute all steps automatically)
   - Step-by-step approval (confirm before each major step) ← RECOMMENDED
   - Sub-step approval (confirm before each checklist item)
6. **Pull Request Strategy**:
   - Single PR with all 10 steps
   - Separate PRs per major phase (Prep, Workspace, Core, CLI, Modules, API, Testing, Docs)
   - One PR per step (10 total) ← RECOMMENDED for safety

## Emergency Procedures

### If Blocked
1. Document the blocker clearly (which step, what's missing)
2. Check if alternative work exists (unlikely - Phase 2 is sequential)
3. Escalate if blocker is external (needs user action)
4. Consider workarounds or temporary solutions
5. Update timeline if significant delay

### If Scope Changes
1. Pause execution immediately
2. Document scope change request
3. Update work queue and phase2-migration-checklist.md
4. Recalculate timeline (may exceed 3-4 weeks)
5. Get user approval before continuing

### If Quality Issues Found
1. Stop and report the issue immediately
2. Assess impact on dependent work
3. Create hotfix issue if needed
4. Adjust timeline if significant
5. Consider if rollback is needed

### If Test Coverage Drops Below 85%
1. Stop and identify which modules lost coverage
2. Write additional unit tests for affected modules
3. Run cargo llvm-cov to verify ≥85%
4. Do not proceed until coverage restored
5. Document in progress report

## Phase-Specific Notes for Phase 2

### Critical Success Factors
1. **Preserve v0.1.0 First**: MUST create v0.1.0-monolithic tag before any changes
2. **Sequential Execution**: Phase 2 steps CANNOT be parallelized due to dependencies
3. **Backward Compatibility**: Every step must maintain config.toml compatibility
4. **No UI in Core**: midimon-core MUST NOT depend on colored, chrono, or any UI crates
5. **100% Feature Parity**: All 26 v0.1.0 features must work identically after migration

### Common Pitfalls to Avoid
1. ❌ Skipping preparation phase (Step 1) → No rollback point
2. ❌ Moving colored/chrono to core → Violates architecture
3. ❌ Not testing after each step → Issues compound
4. ❌ Changing config format → Breaks backward compatibility
5. ❌ Forgetting to update docs-site/ → Violates documentation policy
6. ❌ Not verifying all 26 features → Silent regressions
7. ❌ Rushing through validation (Step 8) → Bugs in production

### Phase 1 Integration
Based on Phase 1 lessons learned:

**Technical Recommendations** (Applied):
- ✅ Start with Core Extraction: Steps 3-5 focus on pure logic extraction
- ✅ Maintain Backward Compatibility: Step 7.5 and 8.6 explicitly test compatibility
- ✅ Test Incrementally: Every step includes cargo check/test validation
- ✅ Use Feature Flags: Not needed (clean architectural separation)

**Process Recommendations** (Applied):
- ✅ Smaller Pull Requests: Each step can be a separate PR (recommended)
- ✅ Continuous Integration: GitHub Actions run on every push
- ✅ Documentation Updates: Step 9 dedicated, docs-site/ policy enforced
- ✅ Test Coverage Maintenance: Step 7.1 requires ≥85%, monitored throughout

### Timeline Reality Check
- **Optimistic (24 hours)**: Only if full-time, no issues, experienced with Rust workspaces
- **Realistic (30 hours)**: Part-time over 3 weeks, normal debugging
- **Pessimistic (36 hours)**: Part-time over 4 weeks, significant issues encountered
- **Target**: 2025-12-30 (allows buffer for holidays)

## Success Criteria & Sign-Off

### Phase 2 Completion Criteria

All of the following must be true:

- [ ] **Workspace Structure**
  - [ ] Root Cargo.toml defines workspace with 2 members (midimon-core, midimon-daemon)
  - [ ] midimon-core compiles independently: cargo build -p midimon-core
  - [ ] midimon-daemon compiles independently: cargo build -p midimon-daemon
  - [ ] cargo build --workspace succeeds

- [ ] **API Surface**
  - [ ] All public API documented in midimon-core/src/lib.rs
  - [ ] All types from api-design.md (AMI-123) exported
  - [ ] No colored/chrono in midimon-core dependencies
  - [ ] Error types properly defined with thiserror

- [ ] **Testing**
  - [ ] All unit tests pass: cargo test --workspace
  - [ ] All integration tests pass
  - [ ] Test coverage ≥85%: cargo llvm-cov --workspace
  - [ ] Config backward compatibility verified (v0.1.0 config.toml loads)
  - [ ] All 26 features work identically to v0.1.0
  - [ ] All diagnostic tools compile and run

- [ ] **Documentation**
  - [ ] README.md updated with workspace structure
  - [ ] CLAUDE.md updated with workspace commands
  - [ ] api-design.md reflects implementation
  - [ ] workspace-structure.md reflects implementation
  - [ ] rustdoc generated without warnings: cargo doc --no-deps
  - [ ] docs-site/ updated per Documentation Site Update Policy

- [ ] **Quality**
  - [ ] cargo fmt --all passes (no formatting issues)
  - [ ] cargo clippy --all-targets passes (no warnings)
  - [ ] No circular dependencies: cargo tree --workspace
  - [ ] Binary size within 5% of v0.1.0 (3-5MB)

- [ ] **Backward Compatibility**
  - [ ] v0.1.0 config.toml loads without errors
  - [ ] All 26 features work identically
  - [ ] CLI arguments unchanged
  - [ ] No deprecation warnings

- [ ] **Git**
  - [ ] v0.1.0-monolithic tag exists (preservation)
  - [ ] v0.2.0-workspace tag created (completion)
  - [ ] All changes committed to main via PR
  - [ ] PR merged with proper documentation

### Sign-Off Template

```markdown
## Phase 2 Completion Sign-Off

**Date**: 2025-12-30
**Duration**: 30 hours (realistic)
**Completed By**: [Name]
**Reviewed By**: [Name]

### Verification
- [x] Checklist items: 100% complete (all 10 steps)
- [x] Tests: 302+ passing, 85%+ coverage
- [x] Backward compatibility: Verified (v0.1.0 config works)
- [x] Documentation: Complete (docs-site updated)
- [x] Git: Main branch updated (v0.2.0-workspace tag created)

### Metrics
- Binary size: 3.2MB (v0.1.0: 3.1MB, +3.2% - within 5% target)
- Memory footprint: 8MB (v0.1.0: 7MB, +14% - within target)
- Response latency: 0.8ms (v0.1.0: 0.7ms, +14% - within target)
- Test coverage: 87.3% (target: ≥85%)
- Tests passing: 325/325 (100%)

### Issues Encountered
- Import path errors in Step 5: Fixed by using crate:: instead of super::
- Test failures in Step 7: Fixed by adding pub to internal structs
- Documentation build failed: Fixed by updating broken links

### Lessons Learned
- Smaller PRs per step made code review easier
- Continuous testing after each step caught issues early
- Documentation updates as we went prevented last-minute rush
- Maintaining ≥85% coverage throughout prevented regression

### Notes
All Phase 2 objectives achieved. Zero regressions from v0.1.0. Ready for Phase 3.

### Sign-Off
- **Tech Lead**: _________________ Date: _______
- **QA Lead**: _________________ Date: _______
- **Product Owner**: __________ Date: _______

### Phase 3 Readiness
Phase 3 (Daemon & Config Hot-Reload) can proceed immediately.
- [x] API stable and documented
- [x] Core engine ready for daemon integration
- [x] Test infrastructure in place
- [x] Documentation site structure supports Phase 3 additions
```

## Appendix: Quick Reference Commands

### Build Commands
```bash
# Build all crates
cargo build --workspace

# Build release
cargo build --workspace --release

# Build specific crate
cargo build -p midimon-core
cargo build -p midimon-daemon --release

# Clean build
cargo clean
cargo build --workspace
```

### Test Commands
```bash
# Test all
cargo test --workspace

# Test specific crate
cargo test -p midimon-core
cargo test -p midimon-daemon

# Test with output
cargo test --workspace -- --nocapture

# Test with coverage
cargo llvm-cov --workspace --html
open target/llvm-cov/html/index.html
```

### Diagnostic Commands
```bash
# Run main CLI
cargo run -p midimon-daemon --release -- 2

# Run diagnostic tools
cargo run -p midimon-daemon --bin midi_diagnostic -- 2
cargo run -p midimon-daemon --bin led_diagnostic
cargo run -p midimon-daemon --bin test_midi

# List MIDI ports
cargo run -p midimon-daemon --release
```

### Quality Assurance
```bash
# Format check
cargo fmt --all -- --check

# Format fix
cargo fmt --all

# Lint check
cargo clippy --all-targets --all-features -- -D warnings

# Lint fix
cargo clippy --fix --allow-dirty --all-targets

# Documentation
cargo doc --workspace --no-deps --open

# Dependency tree
cargo tree --workspace

# Check for duplicates
cargo tree --duplicates
```

### Documentation Site
```bash
# Build docs-site
cd docs-site && mdbook build

# Serve docs-site locally
cd docs-site && mdbook serve --open

# Watch for changes
cd docs-site && mdbook watch
```

### Git Commands
```bash
# Create preservation tag
git tag -a v0.1.0-monolithic -m "Phase 1 complete"

# Create completion tag
git tag -a v0.2.0-workspace -m "Phase 2 complete"

# Push tags
git push origin v0.1.0-monolithic
git push origin v0.2.0-workspace

# View tags
git tag -l

# Create PR (using gh CLI)
gh pr create --title "Phase 2: Core Engine Extraction" --body "..."
```

## References

**Phase 2 Planning Documents**:
- [phase2-migration-checklist.md](phase2-migration-checklist.md) - 1,892 line step-by-step guide (PRIMARY REFERENCE)
- [phase2-migration-guide.md](phase2-migration-guide.md) - Technical migration steps
- [PHASE2_CHECKLIST_SUMMARY.md](PHASE2_CHECKLIST_SUMMARY.md) - Quick reference

**Architecture Documents**:
- [api-design.md](api-design.md) (AMI-123) - Public API specification
- [workspace-structure.md](workspace-structure.md) (AMI-124) - Directory layout
- [config-compatibility.md](config-compatibility.md) (AMI-125) - Backward compatibility

**Project Documents**:
- [implementation-roadmap.md](implementation-roadmap.md) - Overall roadmap
- [linear-dependencies.md](linear-dependencies.md) - Dependency graph
- [traceability-matrix.md](traceability-matrix.md) - Feature tracking
- [CLAUDE.md](../CLAUDE.md) - Project development guide

**Linear**:
- AMI-106: Phase 2 Epic (Core Engine Extraction)
- AMI-123: API Design Document
- AMI-124: Workspace Structure Document
- AMI-125: Config Compatibility Document
- AMI-126: Phase 2 Migration Checklist (this execution guide implements it)

---

## Document History

| Version | Date | Changes |
|---------|------|---------|
| 1.0 | 2025-11-11 | Initial Phase 2 execution guide created from template |

---

**Template Version**: 1.0 (based on phase-execution-template.md)
**Last Updated**: 2025-11-11
**Maintained By**: MIDIMon Project Team
**Location**: docs/phase-2-execution.md

---

**End of Phase 2 Execution Guide**
