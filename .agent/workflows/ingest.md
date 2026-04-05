# Workflow: /ingest-all

## Purpose
Analyze a codebase bottom-up, starting from a user-defined entry point, and generate a complete spec tree.

## Process
1. **Analyze**: Use `unispec parse` on the starting directory to extract functions, structs, and enums.
2. **Map**: Use `unispec index callers` to identify relationships between files and symbols.
3. **Synthesize**:
   - Create a Topic for the current module.
   - Generate `topic.md` (with a one-liner description).
   - Generate `spec.md` (describing existing functionality).
   - Generate `task.md` (marking implementation as complete).
4. **Recurse**: Move to sub-directories and repeat until the entire tree is mapped.

## Agent Rules
- Do not modify existing code.
- If you encounter a complex dependency, use `unispec index callers` to map the relationship.
- If a directory is already mapped, skip it.
- Ensure every generated spec is bound to the relevant file using `unispec index add`.