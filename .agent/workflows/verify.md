# Workflow: /verify

## Purpose
Verify the alignment between the specification and the implementation. This workflow can be triggered at any stage to ensure quality and consistency.

## Process
1. **Context Loading**:
   - Read `spec.md` and `task.md` for the current topic.
   - Use `unispec index list --topic <topic>` to identify bound files.

2. **Alignment Check**:
   - Run `unispec auto verify <topic>`.
   - Analyze the output for mismatches between requirements and implementation.

3. **Relationship Check**:
   - Use `unispec index callers <symbol>` for key functions to ensure no regressions were introduced in dependent modules.

4. **Fixing (If --fix tag is present)**:
   - If issues are found, move the topic to the `Fixing` area.
   - Analyze failure logs or alignment issues.
   - Apply fixes to the code.
   - Update `task.md` notes with the fix details.
   - Return to `Testing` area.

5. **Reporting**:
   - Update `topic.md` with the verification status (aligned/misaligned).
   - If misaligned, list specific issues in the `Verification Summary` section.