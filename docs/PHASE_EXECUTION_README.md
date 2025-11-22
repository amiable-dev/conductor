# Phase Execution System - README

This directory contains a complete system for executing MIDIMon phases with proper dependency tracking and workflow management.

## 📋 Overview

The Phase Execution System provides structured prompts and templates for systematically completing each phase of the MIDIMon project, from Phase 0 (Open Source Setup) through Phase 5 (Advanced Features).

## 📁 Files

### Core Templates
- **phase-execution-template.md** - Master template for any phase (reusable)
- **phase-execution-guide.md** - Quick reference guide and best practices

### Phase-Specific Execution Plans
- **phase-0-execution.md** - Phase 0: Current State Preservation & Open Source Setup
- **phase-1-execution.md** - Phase 1: Documentation & Test Coverage
- **phase-2-execution.md** - *(To be created)* Phase 2: Core Engine Extraction
- **phase-3-execution.md** - *(To be created)* Phase 3: Daemon & Config Hot-Reload
- **phase-4-execution.md** - *(To be created)* Phase 4: Tauri UI & Visual Configuration
- **phase-5-execution.md** - *(To be created)* Phase 5: Advanced Features

### Supporting Documents
- **implementation-roadmap.md** - Overall project roadmap with phase details
- **linear-dependencies.md** - Complete dependency graph for all phases
- **traceability-matrix.md** - Feature tracking across requirements, specs, implementation, tests

## 🚀 Quick Start

### To Execute Current Phase (Phase 0)

```bash
# 1. Read the phase execution plan
cat docs/phase-0-execution.md

# 2. Copy the entire content and paste it into Claude Code as your prompt

# 3. Claude will:
#    - Analyze all issues in the phase
#    - Create a dependency-ordered work queue
#    - Present an execution plan for approval
#    - Execute issues in order, updating Linear and docs-site/
#    - Report progress and completion
```

### To Execute Next Phase (Phase 1)

```bash
# Same process with phase-1-execution.md
cat docs/phase-1-execution.md
# Copy and paste into Claude Code
```

## 🎯 Key Features

### 1. Dependency-Aware Execution
- Automatically detects issue dependencies
- Creates topological sort for proper execution order
- Identifies parallel work opportunities
- Highlights critical path

### 2. Progress Tracking
- Regular progress reports
- Linear status updates
- Completion percentage tracking
- Blocker identification

### 3. Documentation Enforcement
- Requires docs-site/ updates per policy
- Verifies mdbook builds succeed
- Ensures documentation stays current

### 4. Flexible Execution Modes
- **Sequential**: One issue at a time (safest)
- **Parallel**: Multiple issues simultaneously (fastest)
- **Hybrid**: Parallel where possible, sequential for critical path (recommended)

## 📊 Phase Status

| Phase | Status | Duration | Issues | Version |
|-------|--------|----------|--------|---------|
| Phase 0 | 🔄 In Progress | 1.5 weeks | 11 | v0.1.0 |
| Phase 1 | 📋 Planned | 2 weeks | 19 | v0.2.0 |
| Phase 2 | 📋 Planned | 3-4 weeks | 17 | v1.0.0 |
| Phase 3 | 📋 Planned | 3-4 weeks | 15 | v1.5.0 |
| Phase 4 | 📋 Planned | 4-6 weeks | 23 | v2.0.0 |
| Phase 5 | 🔮 Future | Ongoing | TBD | v2.5+ |

## 🔗 Dependency Graph Example

### Phase 0 Dependencies (Open Source Setup)
```
AMI-247 (Repo Setup)
  ├─→ AMI-248 (Core Docs)
  │     └─→ AMI-249 (Community Files)
  │           └─→ AMI-254 (Community Building)
  ├─→ AMI-250 (CI/CD)
  │     └─→ AMI-258 (Docs Infrastructure)
  │           └─→ AMI-251 (Docs Content)
  └─→ AMI-252 (Governance)
        ├─→ AMI-253 (Dev Setup)
        └─→ AMI-255 (Legal)
              └─→ AMI-256 (Release Prep)
```

## 📖 How to Use

### Step 1: Choose Your Phase
Determine which phase you're executing (currently Phase 0).

### Step 2: Read the Execution Plan
Open the corresponding `phase-[N]-execution.md` file and read through it.

### Step 3: Verify Prerequisites
Check the "Prerequisites Check" section to ensure all dependencies are met.

### Step 4: Copy Prompt to Claude Code
Copy the entire phase execution plan and paste it into Claude Code.

### Step 5: Follow the Workflow
Claude will guide you through:
1. **Analysis**: List issues, create dependency graph
2. **Planning**: Present execution plan for approval
3. **Execution**: Work through issues in dependency order
4. **Validation**: Verify Definition of Done for each issue before marking complete
5. **Completion**: Verify criteria met, update docs, create tag

### Step 6: Monitor Progress
Review progress reports and address any blockers.

### Step 7: Complete Phase
When all issues done, complete the phase closeout and move to next phase.

## 🎨 Customization

### For a New Phase
1. Copy `phase-execution-template.md`
2. Replace all `[PLACEHOLDER]` values
3. Fill in phase-specific details from:
   - implementation-roadmap.md (phase section)
   - Linear epic issue (AMI-XXX)
   - linear-dependencies.md (dependency graph)
4. Save as `phase-[N]-execution.md`

### For Modified Dependencies
1. Update `linear-dependencies.md` with new relationships
2. Update phase execution plan with new dependency graph
3. Re-run analysis step to get new work queue

## 🛠️ Tools & Integrations

### Linear Integration
- Lists child issues from phase epics
- Updates issue statuses (Todo → In Progress → Done)
- Tracks blocking relationships
- Uses labels: phase-[N], Feature, Documentation, Testing

### Documentation Site Integration
- Updates docs-site/ per Documentation Site Update Policy
- Verifies mdbook builds succeed
- Enforces documentation completeness
- See implementation-roadmap.md lines 159-219

### Git Integration
- Creates phase completion tags (e.g., v0.2.0-pre)
- Documents commit hashes for reference
- Enables rollback if needed

## 📈 Success Metrics

### Phase Completion Criteria
Each phase has specific success criteria defined in:
- Phase epic (AMI-XXX) in Linear
- Phase section in implementation-roadmap.md
- Phase-specific execution plan

### Common Metrics
- ✅ All issues marked "Done" in Linear (with Definition of Done validated)
- ✅ All success criteria met
- ✅ Test coverage targets achieved (if applicable)
- ✅ Documentation site fully updated
- ✅ No critical bugs or blockers
- ✅ Performance metrics within targets
- ✅ Phase review meeting completed

### Definition of Done (Every Issue)
Before marking any issue as "Done" in Linear, MUST validate:
1. **Acceptance Criteria**: All acceptance criteria from Linear issue are met
2. **Success Criteria**: All success criteria checkboxes are completed
3. **Deliverables**: All required deliverables are created and committed
4. **Testing**: Tests pass locally (if code changes)
5. **Test Coverage**: Minimum 80% code coverage, 100% tests passing (if code changes)
6. **GitHub Actions**: All CI/CD checks pass (build, lint, test, security)
   - Build successful on all target platforms
   - `cargo fmt --check` passes (no formatting issues)
   - `cargo clippy` passes with no warnings
   - `cargo test` passes with 100% success rate
   - `cargo audit` passes with no vulnerabilities
   - Documentation builds successfully
7. **Documentation**: docs-site/ updated per Documentation Site Update Policy
8. **Review**: Code/content reviewed (self-review minimum, PR review ideal)
9. **Quality**: No known critical bugs or incomplete implementations
10. **Validation**: Manually verify the implementation works as specified

**Validation Process**:
- Read the complete Linear issue description
- Check off each acceptance criteria item
- Verify all task breakdown checkboxes are complete
- Run tests locally and verify 80%+ coverage
- Push changes and verify all GitHub Actions pass
- Review GitHub Actions logs for any warnings or issues
- Test the implementation manually
- Only then mark issue as "Done"

## 🚨 Troubleshooting

### Issue is Blocked
1. Check blocker status in Linear
2. Find alternative non-blocked work
3. Escalate if blocker is external
4. Update timeline if significant delay

### Documentation Build Fails
1. Check error message from mdbook
2. Fix formatting or broken links
3. Re-run `cd docs-site && mdbook build`
4. Don't proceed until build succeeds

### Scope Changed Mid-Phase
1. Pause execution
2. Document scope change
3. Update work queue and timeline
4. Get stakeholder approval
5. Resume with new plan

### Estimate Was Wrong
1. Update issue estimate in Linear
2. Recalculate phase timeline
3. Inform stakeholders
4. Identify acceleration opportunities
5. Document lessons learned

## 📚 Related Documents

### Project Documentation
- [PRD-main.md](./PRD-main.md) - Product Requirements Document
- [implementation-roadmap.md](./implementation-roadmap.md) - Overall roadmap
- [traceability-matrix.md](./traceability-matrix.md) - Feature tracking
- [linear-dependencies.md](./linear-dependencies.md) - Dependency graph

### Phase Execution
- [phase-execution-template.md](./phase-execution-template.md) - Master template
- [phase-execution-guide.md](./phase-execution-guide.md) - Quick reference
- [phase-0-execution.md](./phase-0-execution.md) - Phase 0 plan
- [phase-1-execution.md](./phase-1-execution.md) - Phase 1 plan

### Policies
- Documentation Site Update Policy: implementation-roadmap.md lines 159-219
- Phase gate requirements: linear-dependencies.md

## 🤝 Contributing

### Adding a New Phase Execution Plan
1. Use phase-execution-template.md as base
2. Fill in all placeholders with phase-specific data
3. Include complete dependency graph
4. Add phase-specific notes and considerations
5. Test the prompt with Claude Code
6. Submit PR for review

### Updating Existing Plans
1. Verify changes against Linear and roadmap
2. Update dependency graphs if needed
3. Maintain consistent formatting
4. Update PHASE_EXECUTION_README.md if structure changes

## 📞 Support

### Questions?
- Check [phase-execution-guide.md](./phase-execution-guide.md) for common patterns
- Review [implementation-roadmap.md](./implementation-roadmap.md) for context
- Ask in Linear discussions for the phase epic

### Feedback?
- Open an issue in the project repository
- Suggest improvements to templates
- Share lessons learned from phase execution

---

## 🎯 Current Status

**Active Phase**: Phase 0 (Current State Preservation & Open Source Setup)
**Progress**: In Progress
**Next Phase**: Phase 1 (Documentation & Test Coverage)
**Target Completion**: 2025-11-18

---

**Last Updated**: 2025-11-11
**Version**: 1.0
**Maintained By**: MIDIMon Project Team
