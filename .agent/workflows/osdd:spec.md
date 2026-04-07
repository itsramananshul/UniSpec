---
description: Create a specification in Staging area
---

# /osdd:spec

Create a new specification in the **Staging area only**.

## Usage
```
/osdd:spec <TopicName>
```

## Rules
1. **Staging only** - Never create specs in Working or Build
2. **WHAT not HOW** - Specify requirements, not implementation
3. **Unique names** - Topic names must not duplicate existing ones
4. **No `/` in names** - Use nested directory structure if needed

## Steps

### 1. Validate
- Check topic doesn't exist in `spec/Staging/`
- Parse topic name (PascalCase recommended)

### 2. Create Directory
```
spec/Staging/<TopicName>/
```

### 3. Create specs.md
```markdown
# Design: <TopicName>

## Overview
> What does this feature do, and why is it needed?

## Requirements
- REQ-1: [Requirement statement - use SHALL/SHOULD for clarity]

## Scenarios
### Scenario 1
- **Given** [precondition]
- **When** [action]
- **Then** [expected outcome]

## Acceptance Criteria
- [ ] Criterion 1
- [ ] Criterion 2

## Technical Decisions
- **Tooling**: [What libraries/frameworks to use]
- **Trade-offs**: [Justification for choices]
```

## Output
```
✅ Spec created: spec/Staging/<TopicName>/specs.md
Next: /osdd:build <TopicName> to implement
```
