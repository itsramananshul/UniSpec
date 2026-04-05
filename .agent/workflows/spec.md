# Workflow: /spec

## Objective
Iteratively interview the user to define a robust specification, nested topic structure, and task list. You will ask questions until you fully understand what the user wants and you cn build it exactly the way the user wants the program.

## Steps
1. **Analyze Input**: Read the topic's `topic.md` one-liner and any existing notes from the `Planned` area.
2. **Create Artifacts**:
   - Create `spec.md` using the template.
   - Define clear Acceptance Criteria.
   - Define Technical Notes (Dependencies, API design, Data model).
3. **Generate Tasks**:
   - Analyze the `spec.md` requirements.
   - Create `task.md` with specific, actionable tasks.
   - Ensure each task has a status `[ ]` and an empty implementation note block.
4. **Transition**: Move the topic from `Planned` to `Staging`.
5. **Verification**: Confirm all artifacts are created and bound correctly.

## Agent Rules
- Do not write code in this stage.
- If the idea is too vague, ask the user for clarification before creating the spec.
- Ensure the `binding` field in `spec.md` is set to the primary file path for this topic.
