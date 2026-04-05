# Workflow: /verify

## Purpose
Verify the alignment between the specification and the implementation. This workflow can be triggered at any stage to ensure quality and consistency.

## Process
1. **Context Loading**:
   - Use `unispec_read_spec` to load `spec.md` and `task.md` for the current topic.
   - Use `unispec_nav` to identify bound files.

2. **Alignment Check**:
   - Execute `unispec_auto_verify` with the topic name.
   - Analyze the output for mismatches between requirements and implementation.

3. **Relationship Check**:
   - Use `unispec_query_relations` for key functions to ensure no regressions were introduced in dependent modules.

4. **Fixing (If --fix tag is present)**:
   - If issues are found, move the topic to the `Fixing` area.
   - Analyze failure logs or alignment issues.
   - Apply fixes to the code using `file_write`.
   - Update `task.md` notes using `unispec_update_task` with the fix details.
   - Return to `Testing` area.

5. **Reporting**:
   - Update `topic.md` with the verification status (aligned/misaligned).
   - If misaligned, list specific issues in the `Verification Summary` section.