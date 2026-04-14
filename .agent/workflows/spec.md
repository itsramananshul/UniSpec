# Workflow: /spec

**YOUR ONLY JOB: Create specs and tasks using the templates. Don't write code.**

---

## 🔴 STRICT RULE: READ TEMPLATES FIRST!

**Before creating any topic, spec, or task, you MUST read the template files first.**

The templates are located in:
- `.agent/modes/default/templates/topic.md`
- `.agent/modes/default/templates/spec.md`
- `.agent/modes/default/templates/task.md`
- `.agent/modes/default/templates/area.md`

Use `read_asset` to read these templates, then fill them in with actual content.

---

## HOW TO USE THE TOOLS

### Step 1: Read the Topic Template

```
read_asset {
  topic: "templates",
  asset_type: "topic",
  area: "default"
}
```

Then create a topic using topics_add with the template structure:

```
topics_add {
  topic: "myproject",
  area: "Staging",
  short: "A web application for managing tasks and projects",
  content: "[Use the topic template structure - fill in ALL sections with real content]"
}
```

**IMPORTANT**: 
- `content` parameter is REQUIRED and must have at least 20 characters of actual content
- `short` parameter is REQUIRED for TUI display
- The MCP tool will auto-add frontmatter - just provide the body content

---

### Step 2: Read the Spec and Task Templates

```
read_asset {
  topic: "templates",
  asset_type: "spec",
  area: "default"
}
```

Then create a spec + tasks using spec_add:

```
spec_add {
  topic: "myproject/auth",
  area: "Staging",
  short: "User authentication with JWT tokens",
  spec_content: "[Use the spec template structure - fill in ALL sections with real content]",
  task_content: "[Use the task template structure - fill in ALL sections with real content]"
}
```

---

## KEY POINTS

1. **Read templates first** - Don't guess the structure, read the template files
2. **Follow template exactly** - Don't change the structure
3. **Fill in all placeholders** - Write actual content in each section
4. **No testing tasks** - The task template only has 4 phases (Foundation, Core Features, Integration, Polish)
5. **Use MCP tools** - topics_add and spec_add will auto-add frontmatter including `short`
6. **ALWAYS include `short` parameter** - This is required for the TUI display!
7. **Use `spec_content`** - Use `spec_content` NOT `content` for spec_add!
8. **content is REQUIRED** - topics_add requires meaningful content (20+ characters)

---

## ASK IF UNSURE

If you don't know what to write, ASK the user:
- "What should this feature do?"
- "What are the must-have requirements?"
- "What does success look like?"
- "What's a one-line description for this?"