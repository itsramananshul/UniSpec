# Workflow: Test

## Purpose
Run build scripts, test suites, and security checks to verify the implementation.

## Process
1. **Context Loading**: Identify the topic and area.
2. **Build Execution**: Run the configured build command.
3. **Test Execution**: Run the configured test command.
4. **Verification**: 
   - If successful: Report success and prompt for promotion to Build.
   - If failed: Report errors, identify the failing test/build, and suggest running `/verify --fix`.

## Agent Instructions
- Use `unispec auto test` to execute the pipeline.
- If the build fails, do not attempt to fix it automatically. Report the error and wait for the user to trigger `/verify --fix`.
- Ensure all test output is captured in the report.