# Skill: UniSpec BUILD Workflow

## Purpose
This workflow is for implementing specs into code, testing, and fixing issues. Use this when converting specifications into working code.

---

## When to Use This Workflow

Use BUILD workflow when:
- Implementing features from specs
- Writing code that matches specifications
- Running tests and verifying functionality
- Debugging and fixing issues
- Running build/verify commands

---

## Available Tools

### Get Context
- `unispec_read_spec {topic, area}` - Read spec requirements
- `tasks_list {topic, area}` - See what tasks need doing
- `index_find {query, by}` - Find files linked to this spec
- `index_backlinks {topic}` - See what references this spec

### Track Progress
- `tasks_complete {topic, task_index, note, area}` - Mark task complete
- `tasks_incomplete {topic, task_index, note, area}` - Mark task incomplete
- `notes_add {topic, note, area}` - Add implementation notes
- `topics_progress [area]` - See overall progress

### Link New Files
- `index_add {topic, path, area, link_type, tags, annotation}` - Link new file to spec
- `unispec_bind_spec {spec_path, file_path, topic, area}` - Bind code file to spec

### Move Work
- `topics_push {topic, area}` - Move to next area
- `topics_pull {topic, source_area}` - Pull back to Working

### Query Index
- `index_graph` - See full relationship graph
- `index_find {query, by}` - Find by topic/path/tag
- `index_lookup {id}` - Find by export ID

---

## BUILD Workflow Steps

### Step 1: Pick Up Work
```
# Check queue for priority
queue_list

# Get first item from queue
queue_list

# Read the spec for that topic
unispec_read_spec {topic: "feature-name", area: "Working"}

# See what tasks need doing
tasks_list {topic: "feature-name", area: "Working"}
```

### Step 2: Understand Requirements
Read the spec carefully. Understand:
- What needs to be built
- The expected behavior
- Any constraints or requirements

### Step 3: Find Existing Files
```
# Find files already linked to this spec
index_find {query: "feature-name", by: "topic"}

# See all files in the project
index_graph
```

### Step 4: Implement
Write the code to match the spec. Key rules:
- Follow the spec exactly
- Create one task worth of work at a time
- Test as you go

### Step 5: Link New Files
```
# After creating a new file, link it to the spec
index_add {topic: "feature-name", path: "src/new_file.rs", link_type: "implementation", tags: "feature"}
```

### Step 6: Update Tasks
```
# Mark the task you completed as done
tasks_complete {topic: "feature-name", task_index: 0}

# Or add a note about what you did
tasks_complete {topic: "feature-name", task_index: 0, note: "Implemented user login"}
```

### Step 7: Add Notes
```
# Track implementation decisions
notes_add {topic: "feature-name", note: "Using bcrypt for password hashing"}
```

### Step 8: Verify
```
# Check what files are now linked
index_find {query: "feature-name", by: "topic"}

# See full graph
index_graph
```

### Step 9: Push to Next Area
```
# When all tasks complete, push to Testing
topics_push {topic: "feature-name", area: "Testing"}
```

---

## TESTING Phase

When topic is in Testing area:

### Step 1: Read Spec Again
```
unispec_read_spec {topic: "feature-name", area: "Testing"}
tasks_list {topic: "feature-name", area: "Testing"}
```

### Step 2: Run Tests
Run your project's test suite:
```bash
# Run tests (example)
cargo test
# or
npm test
```

### Step 3: Fix Issues
If tests fail:
- Understand the error
- Fix the code
- Update the task status

### Step 4: Push or Pull
```
# If all tests pass, push to Fixing (final review) or Build
topics_push {topic: "feature-name", area: "Fixing"}

# If issues found, can pull back to Working
topics_pull {topic: "feature-name", source_area: "Testing"}
```

---

## FIXING Phase

When topic is in Fixing area:

### Step 1: Review Issues
```
# See what's been linked
index_find {query: "feature-name", by: "topic"}

# Check task status
tasks_list {topic: "feature-name", area: "Fixing"}
```

### Step 2: Fix
Make necessary changes, link any modified files:
```
index_add {topic: "feature-name", path: "src/fixed_file.rs", link_type: "implementation"}
```

### Step 3: Push to Build
```
# When all fixes are done
topics_push {topic: "feature-name", area: "Build"}
```

---

## BUILD Phase (Final)

When topic is in Build area:

### Step 1: Final Verification
```
# Get full picture
index_graph
index_backlinks {topic: "feature-name"}
```

### Step 2: Run Build
```bash
# Run build command
cargo build --release
# or
npm run build
```

### Step 3: Done
The feature is complete!

---

## Key Rules for BUILD Workflow

1. **Read the spec first** - Never code without understanding the requirements

2. **One task at a time** - Work through tasks sequentially

3. **Mark progress** - Update tasks as you complete them

4. **Link every file** - Use `index_add` for every file you create or modify

5. **Add notes** - Track implementation decisions in notes

6. **Test as you go** - Run tests frequently

7. **Verify with index** - Use `index_find` and `index_graph` to understand relationships

---

## Example: Implementing a Feature

```
# Start: Get work from queue
queue_list
# Say queue shows: ["user-auth"]

# Read the spec
unispec_read_spec {topic: "user-auth", area: "Working"}

# See tasks
tasks_list {topic: "user-auth", area: "Working"}
# Say tasks are: ["Create auth module", "Add login endpoint", "Add logout endpoint"]

# Task 0: Create auth module
# (Write code in src/auth/)

# Link the new file
index_add {topic: "user-auth", path: "src/auth/mod.rs", link_type: "implementation", tags: "auth"}

# Mark task complete
tasks_complete {topic: "user-auth", task_index: 0}

# Task 1: Add login endpoint
# (Write code)

# Link it
index_add {topic: "user-auth", path: "src/auth/login.rs", link_type: "implementation", tags: "auth,login"}

# Mark complete
tasks_complete {topic: "user-auth", task_index: 1}

# Add implementation note
notes_add {topic: "user-auth", note: "Using JWT tokens with 1hr expiry"}

# Check progress
tasks_list {topic: "user-auth", area: "Working"}

# When all done, push to Testing
topics_push {topic: "user-auth", area: "Testing"}
```

---

## Example: Fixing an Issue

```
# Topic in Fixing area
unispec_read_spec {topic: "bug-fix", area: "Fixing"}
tasks_list {topic: "bug-fix", area: "Fixing"}

# Make fix
# (edit the code)

# Link the fixed file
index_add {topic: "bug-fix", path: "src/fixed.rs", link_type: "implementation"}

# Push to Build
topics_push {topic: "bug-fix", area: "Build"}
```