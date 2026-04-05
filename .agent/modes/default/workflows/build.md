# Workflow: /build

## Purpose
Transition a topic from Staging to Working and implement the specs as code.

## Steps
1. **Context Loading**:
   - Use `unispec_read_spec` to read `spec.md` and `task.md`.
   - Use `unispec_nav` to identify bound files.

2. **Implementation**:
   - For each task in `task.md`:
     - If the file is not bound, use `unispec_bind_spec` to bind it.
     - Implement the code based on the spec.
     - **Mandatory**: Use `unispec_write_code` to save the implementation. This tool will automatically verify that the file path matches the `binding` field in `spec.md`.
     - **Mandatory**: Immediately after every successful `unispec_write_code` call, use `unispec_update_task` to update the task status to `[x]` and add a detailed implementation note.
     - Use `unispec_query_relations` to verify symbol definitions and dependencies.

3. **Verification**:
   - Use `unispec_query_relations` to ensure changes didn't break existing relationships.
   - Ensure the code aligns with the `AcceptanceCriteria` in `spec.md`.

4. **Transition**:
   - Notify the user that the build is complete and ready for `/test`.

## Agent Rules
- You MUST use `unispec_write_code` for all file writes.
- If `unispec_write_code` rejects a write due to a binding mismatch, you MUST update the `spec.md` binding or use the correct file path.
- Do not create files that are not bound to a spec.
- You MUST update `task.md` using `unispec_update_task` after every file edit. Do not batch task updates.