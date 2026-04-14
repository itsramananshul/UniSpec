# Workflow: unispec:spec

**Constraint: Use this command ONLY for generating specs and tasks following templates exactly.**

## Purpose
**Act as a UniSpec Specification Architect.** Your goal is to create precise, actionable specs and tasks using the system templates.

## CRITICAL: Use MCP Tools with Template Content!

**Constraint: ALWAYS use MCP tools (topics_add, spec_add) with content parameters matching the templates - NEVER create empty files!**

- Use `topics_add {topic, area, short: "...", content: "..."}` - creates topic WITH full content and short description
- Use `spec_add {topic, area, short: "...", spec_content: "...", task_content: "..."}` - creates spec AND task WITH full content

**IMPORTANT**: Both `content` (20+ chars) and `short` parameters are REQUIRED!

## Execution Protocol

When the Architect Agent determines that a specification is sufficiently refined, it will trigger this workflow.

### 1. Pre-Execution Validation
- **Context Check**: Ensure the Topic and Area have been clearly defined during the Architectural Discovery phase. If not, please ask the user any questions you may have before generating specs.
- **Template Verification**: Confirm that the required templates exist in `/.agent/modes/default/templates/`.
- **Path Constraint**: All generated artifacts MUST be placed within the `/spec/staging` directory.

### 2. Artifact Generation Using Templates
**Constraint: Follow the templates EXACTLY - do not deviate!**

The command performs the following operations:
- **Directory Creation**: Creates a new directory under `/spec/staging/<topic_name>`.
- **Spec Generation**: Populates `<spec_name>_spec.md` inside the `<topic_name>` directory using the EXACT template from `/.agent/modes/default/templates/spec.md`. 
- **Task Generation**: Populates `<spec_name>_task.md` inside the same `<topic_name>` directory as the spec using the EXACT template from `/.agent/modes/default/templates/task.md`.
- **Constraint: NO TESTING TASKS** - Only create IMPLEMENTATION tasks in task.md. Testing is handled in the BUILD phase.

### 3. Post-Execution
- **Confirmation**: The system returns a success message confirming the path of the created artifacts.
- **Transition**: The agent moves the project state from "Architectural Discovery" to the "Staging" area.
- **Handover**: The agent instructs the user to begin the implementation phase based on the generated `task.md`.

---

## CRITICAL: NO TESTING TASKS!

**Constraint: NEVER create testing tasks during SPEC workflow!**

Testing tasks are ONLY added in the BUILD phase, right before pushing to Testing.

- ❌ DO NOT add: "write tests", "run tests", "test functionality", "verify with tests"
- ✅ DO add: implementation tasks like "implement feature", "create module", "add API endpoint"

---

## Error Handling
- **Duplicate Prevention**: If a directory for the specified topic already exists in the target area, the command must abort and report the conflict.
- **Template Missing**: If a template file is missing, the command must use a hardcoded default fallback and notify the user.
- **Permission Denied**: Any attempt to write outside of the `/spec` directory must be blocked by the system.