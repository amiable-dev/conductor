# Governance

This document describes the governance structure and decision-making process for the MIDIMon project.

## Project Overview

MIDIMon is an open source MIDI controller mapping system distributed under the MIT License. The project aims to provide a powerful, extensible platform for transforming MIDI devices into advanced macro pads with comprehensive feedback and timing-based triggers.

## Governance Model

MIDIMon follows a **Benevolent Dictator** model in its initial phase, with plans to evolve toward a **committee-based model** as the community grows.

### Current Structure

- **Project Lead**: Christopher Joseph (@christopherjoseph, Amiable)
  - Has final decision authority on all matters
  - Responsible for project vision and direction
  - Coordinates releases and major initiatives
  - May delegate authority to maintainers as project matures

### Roles and Responsibilities

#### Project Lead

- Define project vision and long-term roadmap
- Make final decisions on controversial issues
- Appoint and remove maintainers
- Represent the project in external communications
- Ensure Code of Conduct enforcement

#### Core Maintainers

- Review and merge pull requests
- Triage and respond to issues
- Participate in technical design discussions
- Help enforce community guidelines
- Release management duties (as delegated)

See [MAINTAINERS.md](MAINTAINERS.md) for the current list of maintainers.

#### Contributors

- Anyone who submits a pull request, reports issues, or participates in discussions
- Expected to follow the [Code of Conduct](CODE_OF_CONDUCT.md)
- Recognized in release notes and CHANGELOG

#### Community Members

- Users, testers, documentation readers
- Provide feedback through issues and discussions
- Help other users in support channels
- Share configurations and device profiles

## Decision-Making Process

### Types of Decisions

1. **Minor Changes** (bug fixes, documentation, small features)
   - Decided by maintainers through normal PR review
   - No special process required
   - Merged when approved by at least one maintainer

2. **Major Features** (new trigger types, architectural changes)
   - Require RFC (Request for Comments) process
   - Discussed in GitHub Discussions or dedicated RFC issues
   - Consensus-seeking among maintainers
   - Final approval by project lead if consensus not reached

3. **Breaking Changes** (config format changes, API incompatibilities)
   - Must follow RFC process
   - Require deprecation period (minimum one minor version)
   - Documented prominently in CHANGELOG and migration guide
   - Announced in GitHub Discussions and release notes

4. **Governance Changes** (changes to this document)
   - Proposed via pull request to GOVERNANCE.md
   - Discussed in GitHub Discussions
   - Minimum 7-day comment period
   - Approved by project lead

### RFC Process

For major features or breaking changes:

1. **Create RFC Issue**: Use the "Feature Request" template with detailed proposal
2. **Discussion Period**: Minimum 7 days for community feedback
3. **Refinement**: Author updates proposal based on feedback
4. **Decision**: Project lead or maintainers approve/reject with reasoning
5. **Implementation**: Approved RFCs can proceed to implementation

### Voting (Future)

As the project matures, we may introduce voting for major decisions:

- Maintainers vote on RFCs and significant changes
- Simple majority required for approval
- Project lead retains veto power
- Votes conducted in GitHub Discussions or private maintainer channel

## Change Management

### Version Numbering

MIDIMon follows **Semantic Versioning (SemVer)**:

- **MAJOR** (x.0.0): Breaking changes to config format or public API
- **MINOR** (0.x.0): New features, backward-compatible additions
- **PATCH** (0.0.x): Bug fixes, performance improvements, documentation

Current version: v0.1.0 (pre-1.0 software, API not yet stable)

### Feature Proposal Process

1. **Search Existing Issues**: Check if feature already requested
2. **Open Feature Request**: Use GitHub issue template
3. **Community Discussion**: Gather feedback and use cases
4. **RFC (if major)**: Follow RFC process for significant features
5. **Implementation**: Contributor or maintainer implements
6. **Review & Merge**: Code review and testing
7. **Documentation**: Update docs and CHANGELOG
8. **Release**: Include in next minor version

### Breaking Change Policy

- **Advance Notice**: Announce breaking changes at least one release cycle ahead
- **Deprecation Warnings**: Emit warnings for deprecated features
- **Migration Guide**: Provide clear upgrade path
- **Version Bump**: Always increment MAJOR version for breaking changes
- **Documentation**: Update all affected documentation

Example deprecation timeline:
- v0.5.0: Announce deprecation, add warnings
- v0.6.0: Continue supporting old behavior
- v1.0.0: Remove deprecated feature, breaking change

## Conflict Resolution

### Technical Disagreements

1. Discussion in GitHub issue or PR
2. Seek consensus through constructive debate
3. Maintainers make decision if contributors can't agree
4. Project lead has final say if maintainers disagree
5. Document reasoning in issue/PR for transparency

### Code of Conduct Violations

1. Report to project lead via email (private)
2. Investigation by project lead or designated maintainer
3. Actions: warning, temporary ban, permanent ban (depending on severity)
4. Appeal process: email project lead with additional context
5. Final decision by project lead

See [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md) for enforcement guidelines.

### Escalation Path

1. **First**: Discuss with involved parties directly
2. **Second**: Raise in GitHub Discussions or issue comments
3. **Third**: Contact maintainers privately
4. **Final**: Project lead makes binding decision

## Maintainer Governance

### Becoming a Maintainer

Maintainers are appointed by the project lead based on:

- Sustained high-quality contributions over 6+ months
- Deep understanding of project architecture and goals
- Active participation in code review and issue triage
- Adherence to Code of Conduct and community values
- Demonstrated commitment to project's long-term success

**Nomination Process**:

1. Current maintainer or project lead proposes candidate
2. Discussion among existing maintainers
3. Project lead makes final appointment decision
4. New maintainer added to MAINTAINERS.md and granted repo access

### Removing Maintainers

Maintainers may be removed for:

- Sustained inactivity (6+ months without engagement)
- Code of Conduct violations
- Actions harmful to the project or community
- Request to step down (emeritus status)

**Process**:

1. Project lead initiates removal discussion
2. Private discussion with affected maintainer
3. Decision documented (privately for sensitive cases)
4. Update MAINTAINERS.md and revoke repo access

**Emeritus Status**: Maintainers who step down on good terms are recognized as "Emeritus Maintainers" for their contributions.

## Project Evolution

This governance model is designed for the current scale of the project. As MIDIMon grows, we will adapt our governance structure to ensure:

- Continued responsiveness to community needs
- Distributed decision-making authority
- Sustainability and continuity
- Transparency and accountability

Potential future changes:

- **Technical Steering Committee**: Group of maintainers for major decisions
- **Working Groups**: Specialized teams (UI, Core Engine, Documentation)
- **Community Elections**: Elected representatives for governance input
- **Foundation Model**: Donate project to foundation (CNCF, Apache, etc.)

## Transparency

### Communication Channels

- **GitHub Issues**: Bug reports, feature requests
- **GitHub Discussions**: Q&A, ideas, community conversations
- **Pull Requests**: Code changes, technical review
- **Release Notes**: Version announcements, changelogs
- **Discord** (if established): Real-time community chat

### Public Records

- All technical decisions documented in issues/PRs
- RFC discussions conducted publicly
- Governance changes tracked in this document's git history
- Release decisions explained in release notes

### Private Discussions

Some topics require private discussion:

- Code of Conduct enforcement
- Security vulnerabilities (until patched)
- Confidential personal matters
- Pre-release coordination

## License and Contributions

- All code contributions licensed under **MIT License**
- Contributors retain copyright to their contributions
- By submitting a PR, contributors agree to license their work under project's MIT License
- See [CONTRIBUTING.md](CONTRIBUTING.md) for contribution guidelines

## Amendments

This governance document may be amended by:

1. Opening a pull request with proposed changes
2. Minimum 7-day discussion period in GitHub Discussions
3. Approval by project lead
4. Merge and announce changes in next release notes

---

**Adopted**: 2025-11-11
**Last Updated**: 2025-11-11
**Version**: 1.0
