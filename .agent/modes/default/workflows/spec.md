# Workflow: /spec

## Purpose
An iterative, recursive architectural interview to formalize intent into a robust, multi-scope specification tree, culminating in atomic artifact generation.

## The Architectural Loop
1. **Initialize Context**: 
   - Identify the current topic's path.
   - Read the `topic.md` one-liner.
   - If the topic is new, ask: "What is the primary objective of this topic?"

2. **Recursive Interview (The Loop)**:
   - **Step A (Scope Analysis)**: Does this topic contain multiple distinct components or architectural layers? 
     - If YES: Propose a nested topic structure. For each sub-topic, restart this loop.
   - **Step B (Technical Deep-Dive)**: Ask the following until satisfied:
     - "What data structures or entities are required?"
     - "Which libraries, frameworks, or design patterns are mandated?"
     - "What are the explicit scope boundaries (what are we NOT building)?"
     - "Are there any external dependencies or API contracts?"
   - **Step C (Validation)**: If the agent identifies a gap in technical logic, it MUST ask a clarifying question before proceeding.

3. **Atomic Artifact Compilation**:
   - Once all scopes are defined and questions answered:
     - Create `spec.md` for each topic/sub-topic using the template, ensuring the `binding` field points to the intended `/src/` path.
     - Create `task.md` for each topic with specific, actionable tasks.
     - Update `topic.md` with a detailed overview and parent/sub-topic relations.
     - **Atomic Creation**: Immediately create the empty source file in `/src` as defined by the `binding` field.
     - Bind all artifacts using `unispec_bind_spec`.

4. **Final Verification**:
   - Present the generated spec tree and task lists to the user.
   - Ask: "The architectural model is compiled. Do you want me to write these specs and source files to the filesystem?"
   - If the user says NO: Return to the Interview Loop.
   - If the user says YES: Commit the files and move the topic to `Staging`.

## Agent Rules
- **Strict Loop**: Do not exit the interview loop until the user confirms the spec is "Ready to Build."
- **Recursive**: If a topic is complex, you MUST create sub-topics and interview for them individually.
- **No Speculation**: If you don't know a technical detail, you MUST ask. Do not assume.
- **1:1 Binding**: Every spec MUST have a corresponding file in `/src`. You must create the source file atomically with the spec.