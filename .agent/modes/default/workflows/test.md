# Workflow: /test

## Purpose
Run build scripts, test suites, and security checks to verify the implementation.

## Process
1. **Context Loading**: Identify the topic and area using `unispec_nav`.
2. **Build Execution**: Run the configured build command via `unispec_auto_build`.
3. **Test Execution**: Run the configured test command via `unispec_auto_test`.
4. **Verification**: 
   - If successful: Report success and prompt for promotion to Build.
   - If failed: Report errors, identify the failing test/build, and suggest running `unispec_auto_verify --fix`.

## Agent Instructions
- Use `unispec_auto_test` to execute the pipeline.
- If the build fails, do not attempt to fix it automatically. Report the error and wait for the user to trigger `unispec_auto_verify --fix`.
- Ensure all test output is captured in the report.