# Workflow: /build

## Purpose
Build topics from Staging. Topic MUST be listed in area queue.md to be pushable.

## Key Requirements
1. **CREATE /src FIRST** - Before writing any code, create /src at project root
2. **ALL source files in /src** - At project root, NOT in topic directories
3. **MUST be in queue** - Topic must be listed in area's queue.md to be pushable
4. **Check off tasks** - After completing each task, mark it complete in task.md
5. **Add testing tasks last** - Add test tasks AFTER building, before Testing
6. **queue.md deleted at Testing** - Normal behavior
7. **Use read_asset** to read topic.md, spec, or task files during build

## Read Asset Tool

Use `read_asset` to read any file:

```
read_asset {
  topic: "myproject/auth",
  asset_type: "spec",   // "topic", "spec", or "task"
  area: "Working"
}
```

## Queue File Location

**The queue.md is in the AREA ROOT, not in topic folders:**
```
spec/
├── Staging/
│   └── queue.md    ← List of topics ready to push
├── Working/
│   └── queue.md    ← What to build next
└── ...
```

## Readiness Rule

**Only topics listed in area queue.md can be pushed.**

Check:
```
queue_check {topic: "<topic-name>", area: "Staging"}
```

## Steps

### 1. Check Readiness - MUST be in queue
- Topic must be listed in Staging/queue.md
- Add to queue if not: `queue_add {topic, area: "Staging"}`

### 2. Create /src First
- System creates this automatically when pushing to Working
- All code goes in /src at project root

### 3. Push Topic to Working
- Push topic that is listed in queue
- Do this ONE TOPIC AT A TIME

### 4. Build in Working
- Work through queue in ORDER
- Create code in /src
- Link every file to spec
- **CHECK OFF EVERY TASK** - Mark complete as you go

### 5. Add Testing Tasks Before Testing
- **ONLY add testing tasks here** - AFTER all implementation is done
- This is the LAST step before Testing
- Testing tasks are created in the BUILD phase only, not during SPEC phase

### 6. Push to Testing
- When done, push to Testing
- queue.md automatically deleted

## File Placement

```
PROJECT ROOT/
├── src/                    <-- ALL CODE HERE
└── spec/                   <-- Specs here (NOT code)
```

## Important Notes

- **Check off each task** - Don't skip! Mark tasks complete immediately
- **Queue is in area root** - Not in topic folders
- **Testing comes last** - Add test tasks after all implementation done
- **NO testing in spec phase** - Only development tasks during SPEC workflow