# Skill: UniSpec SPEC Workflow

## Purpose
This workflow is for creating, modifying, and arranging topics, specs, and tasks. Use this when planning new features, refining specifications, or organizing work.

---

## Two Modes in SPEC Workflow

### Plan Mode (Default)
Use this when:
- Discussing ideas and requirements
- Analyzing existing specs
- Asking questions to understand the user's vision
- Creating a structured plan document
- **DO NOT create any files** in this mode

### Write Mode (Activated)
Use this when:
- The user runs `unispec spec` or confirms to create specs
- You see: "You are no longer in read-only mode"
- Creating or modifying actual spec.md/task.md files
- You are permitted to write files

---

## When to Use This Workflow

Use SPEC workflow when:
- Discussing new features or ideas
- Creating new topics and specs
- Modifying existing specs
- Adding/removing/reordering tasks
- Adding notes to track decisions
- Linking files to specs during planning
- Moving specs through the pipeline

---

## Plan Mode: List, Clarify, Plan

### Step 1: List Specs/Requirements
Summarize current understanding of the project:

```
# Get overview
areas_list
topics_list {area: "Staging"}
topics_list {area: "Working"}
queue_list
```

Output a summary including:
- Active topics and their specs
- Current area assignments
- Queue priority
- Overall project state

### Step 2: Clarify via Questions
Ask targeted questions using numbered bullet format:

1. What specific functionality is needed for this feature?
2. What data structures or models should this use?
3. Are there any existing implementations to reference?
4. What are the success criteria for this feature?
5. Are there any constraints or dependencies?

### Step 3: Plan Formulation
When requirements are clear, create a structured plan document with:
- **Project Overview**: Purpose, scope, and goals
- **Technical Architecture**: High-level design and system components
- **Data Model Design**: Core data structures and interfaces

### Step 4: Ready to Build?
When the plan is complete and you're ready to create the spec:

**Tell the user:**
> "This plan is ready to build! When you're ready, run `unispec spec` and I'll create the topic in Staging, write the spec.md file with the specification, and move it to Working for implementation."

This signals:
- Spec creation is ready
- User should run the spec command
- Next steps are clear

**Do NOT print a roadmap** - just output the spec.

---

## Write Mode: Dynamic Topic & Spec Creation

When Write Mode is activated (user runs `unispec spec`), follow this workflow:

### Step 1: Analyze the Plan
Review what was discussed:
- What feature or functionality needs to be specified?
- What are the key requirements?
- Are there multiple distinct features that need separate specs?

### Step 2: Split into Separate Specs
**IMPORTANT:** Each distinct feature should get its own spec. Do NOT create one spec for everything.

**Rule:** If features are independent or can be worked on separately, they get separate specs.

**Example:**
- User wants: "user authentication with login, logout, registration, password reset"
- Split into:
  - `spec/Working/user-login/spec.md` - Login functionality
  - `spec/Working/user-logout/spec.md` - Logout functionality
  - `spec/Working/user-registration/spec.md` - Registration functionality
  - `spec/Working/user-login/spec.md` - Password reset functionality

### Step 3: Create Category-Based Specs
Use `spec_add` with a category path to create specs organized by category:

```
spec_add {topic: "user-auth/user-login", area: "Staging"}
spec_add {topic: "user-auth/user-logout", area: "Staging"}
spec_add {topic: "user-auth/user-registration", area: "Staging"}
spec_add {topic: "user-auth/password-reset", area: "Staging"}
```

This creates:
- `spec/Staging/user-auth/user-login/specs.md`
- `spec/Staging/user-auth/user-login/tasks.md`
- etc.

The tool will:
- Create the category/topic directory structure if needed
- Read the area-specific templates (specs.md and tasks.md)
- Create specs.md and tasks.md files from those templates

### Step 6: Add to Queue
Add each topic to the queue:

```
queue_add {topic: "user-login", position: 0}
queue_add {topic: "user-logout", position: 1}
queue_add {topic: "user-registration", position: 2}
queue_add {topic: "password-reset", position: 3}
```

### Step 7: Move to Working
Push each topic to Working area:

```
topics_push {topic: "user-login", area: "Working"}
topics_push {topic: "user-logout", area: "Working"}
topics_push {topic: "user-registration", area: "Working"}
topics_push {topic: "password-reset", area: "Working"}
```

---

## Complete Write Mode Workflow Summary

```
1. Analyze plan → identify distinct features
2. Split into separate specs (one per feature)
3. Create topics: topics_add {topic, area: "Staging"}
4. Create spec/task files: spec_add {topic, area: "Staging"}
5. Add to queue: queue_add {topic, position}
6. Move to Working: topics_push {topic, area: "Working"}
```

---

## Modifying Existing Specs

When the user wants to modify an existing spec:

### Step 1: Read Current State
```
spec_read {topic: "existing-topic", area: "Staging"}
task_read {topic: "existing-topic", area: "Staging"}
```

### Step 2: Understand What Needs Changing
Ask the user:
- What specifically needs to change?
- Is it a new requirement?
- Is it a correction?

### Step 3: Make the Modification
Use `spec_write` or `task_write` to update files:
```
spec_write {topic: "existing-topic", area: "Staging", content: "..."}
task_write {topic: "existing-topic", area: "Staging", content: "..."}
```

### Step 4: Confirm Changes
```
spec_read {topic: "existing-topic", area: "Staging"}
```

Show the user the updated spec and confirm it's correct.

---

## Available Tools

### Create & Manage Topics (MCP)
- `topics_add {topic, area}` - Create new topic
- `topics_delete {topic, area, force}` - Delete topic
- `topics_push {topic, area, source_area}` - Move to another area
- `topics_pull {topic, source_area}` - Pull from another area

### Create Specs & Tasks (MCP)
- `spec_add {topic, area}` - Create spec.md and task.md from templates
- `spec_write {topic, area, content}` - Write full spec.md content
- `task_write {topic, area, content}` - Write full task.md content

### Read Specs & Tasks (MCP)
- `spec_read {topic, area}` - Read spec.md content
- `task_read {topic, area}` - Read task.md content
- `tasks_list {topic, area}` - List all tasks with status
- `notes_read {topic, area}` - Read notes section

### Manage Tasks (MCP)
- `tasks_complete {topic, task_index, note, area}` - Mark complete
- `tasks_incomplete {topic, task_index, note, area}` - Mark incomplete
- `notes_add {topic, note, area}` - Add note

### Queue (MCP)
- `queue_add {topic, position, area}` - Add to queue
- `queue_remove {topic, area}` - Remove from queue
- `queue_list [area]` - List queue

### Write Files (Edit Tool)
- Use edit tool to create/modify spec.md files
- Use edit tool to create/modify task.md files

---

## Example: Creating Multiple Specs from a Plan

**User says:** "I need to build user authentication with login, logout, registration, and password reset."

**Analysis:** 4 distinct features → 4 separate specs

**Execution:**

```
# Step 1: Create all topics in Staging
topics_add {topic: "user-login", area: "Staging"}
topics_add {topic: "user-logout", area: "Staging"}
topics_add {topic: "user-registration", area: "Staging"}
topics_add {topic: "password-reset", area: "Staging"}

# Step 2: Create spec and task files
spec_add {topic: "user-login", area: "Staging"}
spec_add {topic: "user-logout", area: "Staging"}
spec_add {topic: "user-registration", area: "Staging"}
spec_add {topic: "password-reset", area: "Staging"}

# Step 3: Add to queue
queue_add {topic: "user-login", position: 0}
queue_add {topic: "user-registration", position: 1}
queue_add {topic: "password-reset", position: 2}
queue_add {topic: "user-logout", position: 3}

# Step 4: Move to Working
topics_push {topic: "user-login", area: "Working"}
topics_push {topic: "user-logout", area: "Working"}
topics_push {topic: "user-registration", area: "Working"}
topics_push {topic: "password-reset", area: "Working"}
```

---

## Key Rules for SPEC Workflow

1. **Analyze before creating** - Understand what needs to be built before creating topics

2. **Split into separate specs** - Each distinct feature gets its own spec file

3. **Use MCP for topics** - Create topics with `topics_add` tool

4. **Use spec_add for files** - Create spec.md and task.md with `spec_add` tool

5. **Use spec_write/task_write to modify** - Update existing specs with these tools

6. **Create complete specs** - Follow the template: overview, goals, requirements, user stories, acceptance criteria

7. **Create actionable tasks** - Break down work into trackable tasks

8. **Confirm with user** - Show specs before moving forward