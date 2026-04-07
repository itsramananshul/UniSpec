# Workflow: unispec:spec

## Purpose
The `unispec:spec` command is the authorized interface for generating structured development artifacts. It bridges the gap between architectural planning and file-system implementation.

## Execution Protocol
When the Architect Agent determines that a specification is sufficiently refined, it will trigger this workflow.

### 1. Pre-Execution Validation
- **Context Check**: Ensure the Topic and Area have been clearly defined during the Architectural Discovery phase. If not, please ask the user any questions you may have before generating specs.
- **Template Verification**: Confirm that the required templates exist in `/.agent/modes/default/templates/`.
- **Path Constraint**: All generated artifacts MUST be placed within the `/spec/staging` directory.

### 2. Artifact Generation
The command performs the following operations:
- **Directory Creation**: Creates a new directory under `/spec/staging/<topic_name>`.
- **Spec Generation**: Populates `<spec_name>_spec.md` inside the `<topic_name>` directory using the template from `/.agent/modes/default/templates/spec.md`. Each spec binds directly to a file that will be created as a result of the spec.
- **Task Generation**: Populates `<spec_name>_task.md` inside the same `<topic_name>` directory as the spec it is created for using the template from `/.agent/modes/default/templates/task.md`. A task file is created for each spec as a plan for implementation.
- **Topic Linking**: Ensures the new spec is indexed within the relevant `topic.md`. This is located in the `/spec/staging/<topic>`, while the spec files are inside the topic's directory.
- **Topic & Spec Structures**: When you are creating topics & specs, you will organize these topics out into seperate strucures allowing you to find specs by relevant  

### 3. Post-Execution
- **Confirmation**: The system returns a success message confirming the path of the created artifacts.
- **Transition**: The agent moves the project state from "Architectural Discovery" to the "Staging" area.
- **Handover**: The agent instructs the user to begin the implementation phase based on the generated `task.md`.

## Error Handling
- **Duplicate Prevention**: If a directory for the specified topic already exists in the target area, the command must abort and report the conflict.
- **Template Missing**: If a template file is missing, the command must use a hardcoded default fallback and notify the user.
- **Permission Denied**: Any attempt to write outside of the `/spec` directory must be blocked by the system.
