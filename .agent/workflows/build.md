# Workflow: /build

## Purpose
Transition a topic from Staging to Working and implement the specs as code.

## Steps
1. **Context Loading**:
   - Read `spec.md` to understand the requirements.
   - Read `task.md` to identify the implementation plan.
   - Use `unispec index list --topic <topic>` to identify bound files.

2. **Implementation**:
   - For each task in `task.md`:
     - If the file is not bound, use `unispec index add` to bind it.
     - Implement the code based on the spec.
     - Use `tree-sitter` to verify symbol definitions.
     - Update `task.md` status to `[x]` and add implementation notes.

3. **Verification**:
   - Run `unispec index callers <symbol>` to ensure the changes didn't break existing relationships.
   - Ensure the code aligns with the `AcceptanceCriteria` in `spec.md`.

4. **Transition**:
   - Move the topic directory to the `Working` area.
   - Notify the user that the build is complete and ready for `/test`.