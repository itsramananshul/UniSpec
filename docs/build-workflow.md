# Skill: UniSpec BUILD Workflow

## Purpose
This workflow is for implementing specs into code. Use this when converting specifications into working code.

**IMPORTANT: All work starts in Staging. A topic must be listed in area's queue.md to be pushable.**

---

## KEY RULE: Topics First, Always!

**Before doing ANYTHING, you MUST research topics first!**

1. **ALWAYS start by listing topics** - Never assume you know what's being built
2. **ALWAYS read the topic first** - Understand the scope before building
3. **Topics define scope** - Each topic bounds what specs are being implemented

---

## READINESS SYSTEM - IMPORTANT!

A topic is ONLY ready to push if it is listed in the area's `queue.md` file (e.g., `Staging/queue.md`).

**Why?**
- Prevents pushing topics that aren't ready
- Ensures all work is tracked in the central to-do list
- Queue is deleted when moving from Working → Testing

**How to verify:**
```
# Check if topic is in the area queue
queue_check {topic: "<topic-name>", area: "Staging"}
```

If `ready: true`, you can push. If `ready: false`, add it first:
```
queue_add {topic: "<topic-name>", area: "Staging"}
```

---

## The BUILD Workflow

### Step 0: Research Topics FIRST
1. **List all topics in Staging:**
   ```
   topics_list {area: "Staging"}
   ```

2. **Read the topic:**
   ```
   topic_read {topic: "<topic-name>", area: "Staging"}
   ```

### Step 1: Check Readiness FIRST
**Before pushing anything, check if the topic is in the area queue:**

```
queue_check {topic: "<topic-name>", area: "Staging"}
```

If not ready, add it:
```
queue_add {topic: "<topic-name>", area: "Staging"}
```

**This is REQUIRED - you cannot push a topic that's not in the queue!**

### Step 2: Find queue.md in Staging
**Look for the area queue.md file in Staging to know what to build:**
```
queue_list {area: "Staging"}
```

The queue.md is the CENTRAL TO-DO LIST in the AREA (e.g., `Staging/queue.md`). It contains ALL topics that need to be built.

### Step 3: Create /src FIRST
**Before writing any code, create /src at project root:**
```
# At project root (same level as spec/)
# Create src/ directory - ALL code goes here
```

### Step 4: Push the Topic to Working
**Push a topic that is listed in the queue:**
```
topics_push {topic: "<topic-name>", area: "Working"}
```

This pushes the topic. The queue.md stays in Staging.

### Step 5: Read queue.md in Working
```
queue_list {area: "Working"}
```

### Step 6: Build Each Topic in Order
For each topic in the queue:

1. **Read spec and task files:**
   ```
   spec_read {topic: "<topic-name>", area: "Working"}
   task_read {topic: "<topic-name>", area: "Working"}
   ```

2. **Create code in /src** (NOT in topic directories)

3. **Link every file:**
   ```
   index_add {topic: "<topic-name>", path: "src/filename.rs", link_type: "implementation", tags: "..."}
   ```

4. **CHECK OFF TASKS IMMEDIATELY - THIS IS REQUIRED!**
   After completing each task, mark it complete:
   ```
   task_write {topic: "<topic-name>", area: "Working", content: "..."}
   ```
   Change `- [ ]` to `- [x]` for completed tasks.

   **NEVER skip this step! Always check off completed tasks before moving to the next one.**

### Step 7: Add Testing Tasks Before Pushing to Testing
**Testing tasks are ONLY created here - AFTER all implementation is done:**
```
task_read {topic: "<topic-name>", area: "Working"}
```

Add testing tasks at the end of task.md:
```
## Testing (added before moving to Testing)

- [ ] **T1** [Test the implementation]
- [ ] **T2** [Verify it works as expected]
```

**This is the ONLY place for testing tasks - they are added AFTER development is complete, right before moving to Testing.**

### Step 8: Push to Testing
When done with testing tasks added, push to Testing - queue.md is automatically deleted:
```
topics_push {topic: "<topic-name>", area: "Testing"}
```

---

## Key Rules

0. **CREATE /src FIRST** - Before any code

1. **MUST be in queue** - Topic must be listed in area queue.md to be pushable

2. **ALL code in /src** - At project root, never in topic directories

3. **Link every file** - Use index_add

4. **CHECK OFF TASKS RELIGIOUSLY** - After completing each task, mark it complete with `- [x]`. NEVER skip this!

5. **Add testing tasks last** - Before pushing to Testing, add test tasks

6. **Queue deleted at Testing** - Normal behavior

---

## Queue File Location

**The queue.md is in the AREA, not in topic folders:**
```
spec/
├── Staging/
│   └── queue.md    ← Central to-do list for Staging
├── Working/
│   └── queue.md    ← Central to-do list for Working
└── ...
```

---

## Example

```
# Check if topic is in queue
queue_check {topic: "auth", area: "Staging"}
# If not ready, add it:
queue_add {topic: "auth", area: "Staging"}

# List what's in the queue
queue_list {area: "Staging"}

# Push to Working
topics_push {topic: "auth", area: "Working"}

# Build in Working
# ... create files in /src ...
# ... check off tasks ...

# Add testing tasks, then push to Testing
topics_push {topic: "auth", area: "Testing"}
```