---
description: Build implementation from Staging spec, pushes to Working
---

# /osdd:build

Build implementation from a Staging spec. Pushes to Working and implements.

## Usage
```
/osdd:build <TopicName>
```

## Rules
1. **Read spec first** - Never guess requirements
2. **Follow tasks in order** - Complete each before moving on
3. **Document decisions** - Add notes for anything not in spec
4. **Mark tasks complete** - Update tasks.md immediately
5. **Stay faithful** - If spec says X, implement X

## Steps

### 1. Locate Spec
- Verify `spec/Staging/<TopicName>/specs.md` exists
- Read it thoroughly

### 2. Push to Working
```bash
osdd push <TopicName> Working
```
This creates `spec/Working/<TopicName>/` with specs.md copied.

### 3. Create tasks.md (if missing)
```markdown
# Tasks for <TopicName>

## Phase 1: Foundation
- [ ] **1.1** [Task based on REQ-1]

## Phase 2: Core
- [ ] **2.1** [Task based on REQ-2]

## Phase 3: Testing
- [ ] **3.1** [Verify implementation]

## Notes
- **YYYY-MM-DD**: Initial tasks created
```

### 4. Implement Each Task
For each pending task:

1. **Announce**: "Working on Task X.X: [description]"
2. **Read spec** for relevant requirements
3. **Implement** code
4. **Add notes** if you make decisions not in spec
5. **Mark complete**: `- [ ]` → `- [x]`

### 5. Track Progress
Show progress after each task:
```
Progress: 2/5 tasks (40%)
```

## Notes Format
```markdown
## Notes
- **YYYY-MM-DD**: [Decision made]
  - Reason: [Why]
  - Alternative: [What else was considered]
```

## On Completion
```
## Build Complete: <TopicName>

Tasks: 5/5 (100%)
Ready for /osdd:verify <TopicName>
```

## Guardrails
- Don't skip tasks
- Don't leave TODOs in code
- If spec is unclear, ask instead of guessing
- If implementation differs from spec, document it
