---
description: Verify implementation matches specification
---

# /osdd:verify

Verify that implementation matches the specification. Trace each requirement to code.

## Usage
```
/osdd:verify <TopicName> [Working|Build]
```

## Rules
1. **Be thorough** - Check every REQ-*
2. **Show evidence** - Cite file and line number
3. **Don't assume** - If not found, mark as missing
4. **Distinguish partial** - Similar but not exact is partial

## Status Meanings
| Status | Meaning |
|--------|---------|
| ✓ Implemented | Code exists and matches spec |
| ⚠ Partial | Partially implemented or different |
| ✗ Missing | Not found in codebase |

## Steps

### 1. Locate Files
- Spec: `spec/<Area>/<TopicName>/specs.md`
- Implementation: Search `src/` or use `osdd index find --topic <TopicName>`

### 2. Extract Requirements
From specs.md, list all `REQ-*` items:
```markdown
## Requirements
- REQ-1: System SHALL do X
- REQ-2: System SHOULD do Y
```

### 3. Trace Each Requirement
For each REQ-*:
1. Search codebase for relevant code
2. Check if it matches the requirement
3. Record status with evidence

### 4. Generate Report
```markdown
# Verification Report: <TopicName>

## Coverage
| REQ | Status | Evidence |
|-----|--------|----------|
| REQ-1 | ✓ | src/auth.ts:42 |
| REQ-2 | ✗ | Not found |

## Summary
3/4 requirements verified (75%)
```

## Output Formats

### All Pass
```
✓ Verification Complete: <TopicName>
Coverage: 5/5 (100%)
All requirements verified.
Ready for Build.
```

### With Gaps
```
✓ Verification Complete: <TopicName>
Coverage: 4/5 (80%)

✗ REQ-3: Feature not implemented
  Recommendation: Add user preference storage

⚠ REQ-4: Partial implementation
  Found: logs with redaction
  Expected: no password logging at all

Options:
1. /osdd:build <TopicName> to fill gaps
2. Update spec to remove requirement
```
