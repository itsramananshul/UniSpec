# Fixing

## Purpose
This area is dedicated to debugging and resolving issues identified during the Testing phase. It serves as a focused workspace for diagnosing failures, applying fixes, and ensuring the code aligns with the established specifications before returning to the Testing area.

## Guidelines
- Analyze failure logs and stack traces thoroughly before modifying code.
- Use `unispec index callers` to identify potential side effects of your fixes.
- Once a fix is applied, verify it against the failing test case.
- Document the root cause and the fix in the topic's task notes.