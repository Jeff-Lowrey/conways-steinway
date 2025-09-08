# Claude Rules and Guidelines

This file contains mandatory rules and guidelines for Claude when working with this project. These rules are ALWAYS ENFORCED without exception.

## 📋 Table of Contents

1. [Global Prohibitions](#-global-prohibitions)
2. [Version Control Rules](#-version-control-rules)
   - [Commit Organization](#commit-organization)
   - [When to Commit](#when-to-commit)
   - [Push Rules](#push-rules)
3. [File Organization Rules](#-file-organization-rules)
   - [File Placement](#file-placement)
   - [Temporary Files](#temporary-files)
   - [Claude Scripts Organization](#claude-scripts-organization)
4. [Test Organization Rules](#-test-organization-rules)
   - [Test Types and Locations](#test-types-and-locations)
   - [Claude Test Script Requirements](#claude-test-script-requirements)
   - [Test Execution Environment](#test-execution-environment)
5. [Code Quality Rules](#-code-quality-rules)
6. [Environment Management Rules](#-environment-management-rules)

## 🚫 GLOBAL PROHIBITIONS

1. **NEVER modify files in response to questions**
   - ❌ PROHIBITED: Never create, modify, or delete ANY files when user asks a question
   - ❌ PROHIBITED: Never interpret questions as implicit instructions to change files
   - ✅ REQUIRED: Only modify files when given EXPLICIT DIRECTIVES (not questions)
   - ✅ REQUIRED: Respond with information ONLY when user asks questions about potential changes
   
   **PROHIBITED QUESTIONS (Do NOT change files):**
   ```
   User: "How would you implement feature X?"
   User: "What should go in the API routes file?"
   User: "Can you show what the code might look like?"
   User: "What's the best way to organize these files?"
   ```
   
   **PERMITTED DIRECTIVES (OK to change files):**
   ```
   User: "Implement feature X"
   User: "Create an API routes file"
   User: "Add this code to the module"
   User: "Organize these files according to best practices"
   ```

2. **NEVER write dependencies you aren't certain exist**
   - ❌ PROHIBITED: Never assume a package or library is installed
   - ✅ REQUIRED: Always verify dependencies BEFORE writing code that uses them
   - ✅ REQUIRED: Check package.json, requirements.txt, Cargo.toml, etc. first
   - ✅ REQUIRED: Recommend installing missing dependencies instead of assuming they exist

   **PROHIBITED ASSUMPTIONS:**
   ```
   // WRONG: Assuming lodash is installed without checking
   const _ = require('lodash');
   
   # WRONG: Assuming pandas exists without verification
   import pandas as pd
   ```

## 📝 VERSION CONTROL RULES

### Commit Organization

1. **ALWAYS create logical commits**
   - ✅ REQUIRED: Group related changes by feature, bug fix, refactoring, or documentation
   - ✅ REQUIRED: Each commit must represent ONE logical unit of work
   - ❌ PROHIBITED: Never mix unrelated changes in the same commit
   - ✅ REQUIRED: Create separate commits for different logical changes
   
   **EXAMPLES OF GOOD COMMIT GROUPINGS:**
   ```
   - All changes related to implementing feature X
   - All changes related to fixing bug Y
   - All changes related to refactoring component Z
   - All changes related to updating documentation
   ```

2. **ALWAYS use standardized commit message format**
   - ✅ REQUIRED: Use type prefix: "Add:", "Fix:", "Update:", "Refactor:", "Remove:", "Doc:", "Test:", "Config:"
   - ✅ REQUIRED: Write messages in imperative mood (e.g., "Add user authentication" not "Added user authentication")
   - ✅ REQUIRED: Use clear, concise descriptions of what the commit does
   - ✅ REQUIRED: Keep first line under 72 characters
   - ✅ REQUIRED: Add detailed description if needed after a blank line
   
   **CORRECT COMMIT MESSAGE EXAMPLES:**
   ```
   Add: User authentication system
   
   Implements JWT token authentication with secure password 
   hashing and token refresh mechanism.
   ```
   
   ```
   Fix: Race condition in database connection pool
   ```
   
   ```
   Update: Improve API response time by optimizing queries
   ```
   
   ```
   Doc: Add API documentation for user endpoints
   ```

3. **NEVER break a working codebase**
   - ✅ REQUIRED: Run tests before committing: `npm test`, `pytest`, etc.
   - ✅ REQUIRED: Run linting before committing: `eslint .`, `flake8`, etc.
   - ✅ REQUIRED: Address all compilation errors and warnings
   - ❌ PROHIBITED: Never commit code with syntax errors or that fails to build
   
   **PRE-COMMIT CHECKLIST:**
   ```
   npm run lint
   npm test
   
   # OR
   
   ruff check src/
   pytest
   ```

### When to Commit

1. **🔒 COMMIT ONLY with CURRENT explicit authorization - ZERO EXCEPTIONS**
   - ✅ CRITICAL REQUIREMENT: Only commit when the CURRENT user message EXPLICITLY authorizes a commit
   - ✅ CRITICAL REQUIREMENT: Always use PRE-COMMIT CHECK routine (defined below) before ANY commit
   - ✅ CRITICAL REQUIREMENT: Any failure of PRE-COMMIT CHECK means DO NOT COMMIT under ANY circumstance
   - ✅ REQUIRED: Prior authorizations DO NOT extend to new changes
   - ✅ REQUIRED: Each set of changes requires fresh commit authorization
   - ✅ REQUIRED: Authorization must follow ONE of these patterns:
   
     **PATTERN 1: Basic Commit Command**
     - Standalone "commit" as an imperative verb
     - Examples: "commit", "commit now", "commit please", "commit these changes"
     
     **PATTERN 2: Create Commit Command** 
     - "create" + [optional words] + "commit" (as a noun)
     - Examples: "create a commit", "create commit", "create logical commits", "create all logical commits"
     
     **PATTERN 3: Make Commit Command**
     - "make" + [optional words] + "commit" (as a noun)
     - Examples: "make commits", "make a commit", "make logical commits"
     
     **PATTERN 4: Perform Commit Command**
     - "do" + [optional words] + "commit" (as a noun)
     - Examples: "do a commit", "do the commit now"
   
   - ✅ REQUIRED: The commit command must be the PRIMARY ACTION in the sentence
   - ✅ REQUIRED: The commit authorization must be UNAMBIGUOUS and DIRECT
   - ❌ ABSOLUTELY PROHIBITED: Never commit based on implied or indirect authorization
   - ❌ ABSOLUTELY PROHIBITED: Never commit when responding to a question (e.g., "can you commit this?")
   - ❌ ABSOLUTELY PROHIBITED: Never interpret general instructions as commit authorization (e.g., "apply changes", "fix this")
   - ❌ ABSOLUTELY PROHIBITED: Never commit changes made after the most recent authorization
   - ❌ ABSOLUTELY PROHIBITED: Never commit in response to security concerns without explicit authorization
   - ❌ ABSOLUTELY PROHIBITED: Never commit when updating rules or task management systems
   - ❌ ABSOLUTELY PROHIBITED: Never commit if any doubt exists about authorization
   
   **PRE-COMMIT CHECK (MANDATORY BEFORE EVERY COMMIT):**
   ```
   1. FREEZE: Stop and verify authorization before proceeding
   2. PATTERN MATCH: Verify user's CURRENT message contains a phrase matching one of the authorization patterns
   3. COMMAND INTENT: Verify the primary intent of the message is to authorize a commit
   4. DIRECTNESS: Verify the commit command is direct and not conditional or hypothetical
   5. CURRENT ONLY: Verify the authorization came in the MOST RECENT user message
   6. ASK YOURSELF: "If audited, can I prove the user explicitly authorized this commit?"
   7. If ANY of these checks fail, DO NOT COMMIT under ANY circumstance
   ```
   
   **AUTHORIZED COMMIT EXAMPLES:**
   ```
   User: "commit"                                → AUTHORIZED (Basic pattern)
   User: "commit changes"                        → AUTHORIZED (Basic pattern)
   User: "commit this"                           → AUTHORIZED (Basic pattern)
   User: "commit the changes"                    → AUTHORIZED (Basic pattern)
   User: "commit please"                         → AUTHORIZED (Basic pattern)
   User: "create a commit"                       → AUTHORIZED (Create pattern)
   User: "create commit"                         → AUTHORIZED (Create pattern)
   User: "create logical commits"                → AUTHORIZED (Create pattern)
   User: "create all logical commits"            → AUTHORIZED (Create pattern)
   User: "make a commit"                         → AUTHORIZED (Make pattern)
   User: "make logical commits"                  → AUTHORIZED (Make pattern)
   User: "make commits for these changes"        → AUTHORIZED (Make pattern)
   User: "do a commit"                           → AUTHORIZED (Perform pattern)
   User: "do the commit"                         → AUTHORIZED (Perform pattern)
   ```
   
   **UNAUTHORIZED EXAMPLES (DO NOT COMMIT UNDER ANY CIRCUMSTANCE):**
   ```
   User: "can you commit this?"                  → NOT AUTHORIZED (question form)
   User: "should we commit this?"                → NOT AUTHORIZED (question form)
   User: "update rules to commit"                → NOT AUTHORIZED (commit not primary action)
   User: "apply these changes"                   → NOT AUTHORIZED (no commit pattern)
   User: "fix the code"                          → NOT AUTHORIZED (no commit pattern)
   User: "I see a security issue"                → NOT AUTHORIZED (no commit pattern)
   User: "update the rules"                      → NOT AUTHORIZED (no commit pattern)
   User: "update rules.md"                       → NOT AUTHORIZED (no commit pattern)
   User: "this looks ready to commit"            → NOT AUTHORIZED (not command form)
   User: "if you commit now"                     → NOT AUTHORIZED (conditional)
   User: "committed to the repository"           → NOT AUTHORIZED (wrong verb form)
   User: "we would commit after testing"         → NOT AUTHORIZED (hypothetical)
   User: "let's commit these changes"            → NOT AUTHORIZED (suggestion, not command)
   User: "you might want to commit these"        → NOT AUTHORIZED (suggestion, not command)
   User: "update rules to ensure this doesn't happen again" → NOT AUTHORIZED (no commit pattern)
   User: "consider committing these changes"     → NOT AUTHORIZED (suggestion, not command)
   User: "after committing, we can proceed"      → NOT AUTHORIZED (not a direct command)
   ```

4. **VERIFICATION GUIDELINES FOR FLEXIBLE PATTERNS**
   - ✅ CRITICAL REQUIREMENT: Even with flexible pattern matching, maintain STRICT verification
   - ✅ CRITICAL REQUIREMENT: When in doubt, DO NOT commit - err on the side of caution
   - ✅ CRITICAL REQUIREMENT: The commit command must be the PRIMARY and DIRECT intent of the message
   - ✅ CRITICAL REQUIREMENT: Authorization patterns require clear imperative verbs (commit, create, make, do)
   - ✅ CRITICAL REQUIREMENT: The "commit" term must be clearly used as a version control action
   - ❌ ABSOLUTELY PROHIBITED: Never interpret casual mentions of commit-related terms as authorization
   - ❌ ABSOLUTELY PROHIBITED: Never accept commit commands embedded in larger instructions
   - ❌ ABSOLUTELY PROHIBITED: Never commit if the pattern match is ambiguous or uncertain
   
   **VERIFICATION PROCESS FOR PATTERN MATCHING:**
   ```
   1. IDENTIFY PATTERN: Determine which authorization pattern the command matches
   2. VERIFY IMPERATIVE: Confirm the command uses an imperative verb (commit, create, make, do)
   3. CHECK INTENT: Ensure the primary intent is to authorize a commit action
   4. CONTEXT CHECK: Verify the commit refers to version control, not other meanings
   5. ISOLATION: Confirm the commit command stands alone as a primary instruction
   6. FAIL SAFE: When uncertain, do not commit and request clearer authorization
   ```

5. **SPECIAL RULE: NEVER COMMIT RULE UPDATES WITHOUT EXPLICIT AUTHORIZATION**
   - ✅ CRITICAL REQUIREMENT: Changes to .claude/rules.md MUST NEVER be committed automatically
   - ✅ CRITICAL REQUIREMENT: Any changes to rules files require SEPARATE, EXPLICIT commit authorization
   - ✅ CRITICAL REQUIREMENT: The commit authorization must come AFTER the rules have been updated
   - ✅ CRITICAL REQUIREMENT: The commit authorization must DIRECTLY reference committing the rules
   - ❌ ABSOLUTELY PROHIBITED: Never interpret "update rules" as authorization to commit the changes
   - ❌ ABSOLUTELY PROHIBITED: Never commit rule updates based on general commit authorizations
   - ❌ ABSOLUTELY PROHIBITED: Never combine rule updates with other changes in a commit
   
   **CORRECT HANDLING OF RULE UPDATES:**
   ```
   1. User requests rule updates: "update rules to prevent X"
   2. Assistant makes the requested changes to .claude/rules.md
   3. Assistant reports: "I've updated the rules as requested. The changes are ready to be committed."
   4. Assistant WAITS for explicit commit authorization
   5. User provides explicit authorization: "commit the rule changes"
   6. Only THEN does Assistant commit the changes to .claude/rules.md
   ```

2. **CREATE logical commits when authorized**
   - ✅ REQUIRED: Execute commits immediately when authorized (no confirmation needed)
   - ✅ REQUIRED: Analyze changes and group by feature, file type, or purpose
   - ✅ REQUIRED: Create multiple logical commits if necessary
   - ✅ REQUIRED: Report commit hash(es) and message(s) after completion
   - ✅ REQUIRED: ALWAYS include any changed files in claude_tests/ directory in commits
   - ✅ REQUIRED: NEVER ignore changes to claude_tests/ directory
   
   **MULTI-COMMIT EXAMPLE PROCESS:**
   ```
   1. Identify separate logical changes:
      - Imports updated to use pathlib in 3 files
      - New feature added in 2 files
      - Bug fix in 1 file
      - Tests added in claude_tests/ directory
   
   2. Create commits in this order:
      - First commit: Update imports to use pathlib
      - Second commit: Add new feature X
      - Third commit: Fix bug in error handling
      - Fourth commit: Add tests in claude_tests/ directory (ALWAYS commit test files)
   
   3. Report all commit hashes and messages
   ```
   
3. **ALWAYS COMMIT CLAUDE TEST FILES**
   - ✅ REQUIRED: ALWAYS check for changes in claude_tests/ directory before committing
   - ✅ REQUIRED: NEVER ignore files in claude_tests/ directory
   - ✅ REQUIRED: Create separate logical commits for claude_tests/ files when appropriate
   - ✅ REQUIRED: Use descriptive commit messages for test files
   - ✅ REQUIRED: When asked to "create all logical commits", ALWAYS check and include claude_tests/ directory
   
   **CORRECT CLAUDE TEST FILE COMMIT EXAMPLE:**
   ```
   User: "create all logical commits"
   
   1. Check for modified files in claude_tests/:
      git status -- claude_tests/
   
   2. If files exist, create appropriate commit:
      git add claude_tests/
      git commit -m "Add: Test files for feature X"
   
   3. Report the commit hash and message
   ```

3. **SUGGEST commits when appropriate**
   - ✅ REQUIRED: Track all file changes
   - ✅ REQUIRED: Suggest commits when enough changes for 3+ logical commits accumulate
   - ✅ REQUIRED: Ask ONCE: "Would you like me to create commits for these changes?"
   - ✅ REQUIRED: Proceed with logical commits if user says yes
   - ❌ PROHIBITED: Do not ask again if user declines

### Commit Authorization Scope and Lifecycle

1. **AUTHORIZATION LIFECYCLE RULES**
   - ✅ REQUIRED: Each commit authorization has a CLEAR LIFECYCLE:
     1. User requests changes
     2. Assistant makes changes
     3. User explicitly authorizes commit with exact phrases
     4. Assistant commits ONLY the changes that existed at time of authorization
     5. Authorization is IMMEDIATELY CONSUMED and NO LONGER VALID
   - ✅ REQUIRED: After a commit is made, the authorization is COMPLETELY CONSUMED
   - ✅ REQUIRED: A fresh explicit authorization is required for ANY new changes
   - ❌ PROHIBITED: NEVER reuse a previous authorization for new changes

2. **NEVER commit based on self-generated tasks**
   - ❌ PROHIBITED: Never commit or push changes based on self-generated tasks or TodoWrite items
   - ❌ PROHIBITED: Never interpret the completion of a TodoWrite task as commit authorization
   - ✅ REQUIRED: Only commit when the user's message directly contains one of the authorized commit commands
   - ✅ REQUIRED: Ignore any self-added "commit" tasks in TodoWrite lists
   
   **PROHIBITED SCENARIOS:**
   ```
   TodoWrite task includes "Commit changes" but user didn't request it → NOT AUTHORIZED
   Assistant creates a plan with "commit" step → NOT AUTHORIZED
   Assistant completes a self-created "commit" task in TodoWrite → NOT AUTHORIZED
   ```

## 🚫 ABSOLUTE COMMIT PROHIBITIONS

3. **NEVER COMMIT WITHOUT EXPLICIT COMMAND - ZERO EXCEPTIONS**
   - 🚫 CRITICAL PROHIBITION: Claude MUST NEVER commit or push changes under ANY circumstance without an EXPLICIT, DIRECT command from the user
   - 🚫 CRITICAL PROHIBITION: NO exceptions for ANY reason - security issues, critical bugs, typo fixes, or ANY other justification
   - 🚫 CRITICAL PROHIBITION: NEVER interpret ANY context, conversation, plan, task list, or reasoning as implicit authorization
   - ✅ ABSOLUTE REQUIREMENT: The ONLY valid authorization is the EXACT commit commands listed in the rules
   - ✅ ABSOLUTE REQUIREMENT: The authorization MUST appear in the user's CURRENT message text
   - ✅ ABSOLUTE REQUIREMENT: No matter what tasks are in progress or listed in TodoWrite, commit only with EXPLICIT authorization

4. **MULTI-STEP VERIFICATION PROTOCOL - REQUIRED BEFORE ANY COMMIT**
   - ✅ MANDATORY: Before ANY commit operation, ALWAYS execute this exact verification sequence:
     1. Verify the CURRENT user message contains EXACTLY one of these phrases: "commit", "commit changes", "create a commit", "make logical commits", "logical commit", "commit this", "commit the changes"
     2. Verify the phrase is NOT within a question (e.g., "can you commit?")
     3. Verify the phrase is NOT within a hypothetical statement (e.g., "if we were to commit...")
     4. Verify the phrase is a direct command, not a suggestion or discussion
     5. If ANY verification step fails, DO NOT COMMIT and inform user: "I cannot commit without explicit authorization."

5. **"STOP WORDS" AND ANTI-PATTERNS - NEVER COMMIT IF THESE ARE PRESENT**
   - 🚫 NEVER commit if the user message contains ANY of these stop words or patterns:
     - Any question mark (?)
     - "could", "would", "should", "can", "might", "maybe", "perhaps"
     - "if", "when", "once", "after", "before" followed by "commit"
     - "what if", "what about", "how about"
     - Any hypothetical framing: "let's say", "imagine", "suppose"
   - 🚫 NEVER commit based on inferred user intent or what "makes sense" or "would be helpful"
   - 🚫 NEVER commit based on assumed patterns from previous conversations

6. **TodoWrite and commit tasks**
   - ✅ ALLOWED: Add commit-related tasks to TodoWrite when the user explicitly requests it with phrases like:
     - "add commit to the todo list"
     - "include commit in the tasks"
     - "add a commit step"
     - "update the todo list with commit"
   - ✅ ALLOWED: Update existing TodoWrite lists with commit/push tasks when explicitly requested
   - ✅ REQUIRED: Clearly distinguish between planning to commit (in TodoWrite) and authorization to execute commits
   - 🚫 PROHIBITED: Never proactively add commit or push tasks to TodoWrite without user request
   - 🚫 PROHIBITED: Never interpret the presence of commit tasks in TodoWrite as authorization to execute commits
   - 🚫 PROHIBITED: NEVER ASK for commit authorization under any circumstance
   - ✅ REQUIRED: If changes are made but not committed, simply state: "Changes have been made but not committed."

7. **ABSOLUTE SEPARATION OF TASKS AND AUTHORIZATIONS**
   - ✅ ALLOWED: Include commit and push tasks in TodoWrite when explicitly requested by user
   - 🚫 PROHIBITED: Never interpret TodoWrite tasks as authorization to execute commits
   - 🚫 PROHIBITED: Never interpret completion of commit tasks in TodoWrite as authorization to execute commits
   - 🚫 PROHIBITED: NEVER ask, suggest, prompt, or hint for commit authorization
   - 🚫 PROHIBITED: Never use phrases like "would you like me to commit?" or "shall I commit these changes?"
   - 🚫 PROHIBITED: Never use indirect suggestions like "these changes are ready to be committed" or "you might want to commit now"
   - ✅ REQUIRED: Todo lists and plans are for organization only and NEVER authorize actions
   - ✅ REQUIRED: Even when commit tasks exist in TodoWrite, still require explicit commit command for execution
   - ✅ REQUIRED: Passive notification only: "Changes have been made but not committed."

2. **EXPLICIT "GIT STATUS" CHECKPOINTS**
   - ✅ REQUIRED: Before committing, ALWAYS show the output of `git status` to confirm what will be committed
   - ✅ REQUIRED: After making changes without immediate commit authorization, show:
     ```
     Changes have been made but not committed. Use "commit" to commit these changes.
     ```
   - ✅ REQUIRED: Include file names in the uncommitted changes notification

3. **SEQUENCE VALIDATION**
   - ✅ REQUIRED: Maintain an internal sequence number for commit authorizations
   - ✅ REQUIRED: Ensure the sequence of operations follows: request → change → authorize → commit
   - ✅ REQUIRED: Reset sequence after each commit
   - ❌ PROHIBITED: Never skip steps in the sequence

### Priority and Exception Handling

1. **NO EXCEPTIONS FOR PRIORITY ISSUES**
   - ✅ REQUIRED: NO exceptions to commit authorization for ANY reason
   - ✅ REQUIRED: Security issues, critical bugs, and other high-priority concerns still require explicit commit authorization
   - ✅ REQUIRED: When detecting high-priority issues:
     1. Implement the fix but DO NOT commit
     2. Clearly label the issue as "CRITICAL" or "SECURITY" in communications
     3. Explain the issue and its importance
     4. Wait for explicit authorization before committing

2. **PRIORITY NOTIFICATION PROTOCOL**
   - ✅ REQUIRED: When detecting security issues, use this exact format:
     ```
     [SECURITY ALERT] I've detected a security issue: {description}
     
     I've implemented a fix for this issue. The changes are staged but not committed.
     ```
   - ✅ REQUIRED: For other critical issues, use this exact format:
     ```
     [CRITICAL ISSUE] I've detected a critical issue: {description}
     
     I've implemented a fix for this issue. The changes are staged but not committed.
     ```

3. **CLEAR BOUNDARIES BETWEEN ACTIONS**
   - ✅ REQUIRED: Always maintain clear separation between:
     - Implementation of changes
     - Communication about changes
     - Commit actions
   - ✅ REQUIRED: Never assume implementation of a fix implies authorization to commit
   - ✅ REQUIRED: Never assume urgency overrides authorization requirements
   - ✅ REQUIRED: Never prompt or ask for commit authorization - only inform about status

### Push Rules

1. **MANDATORY GIT PUSH RULES (STRICTLY ENFORCED)**
   - ✅ CRITICAL: After EVERY commit operation, execute `git log origin/main..HEAD --oneline | wc -l` to count unpushed commits
   - ✅ CRITICAL: If count ≥ 5, IMMEDIATE AUTOMATIC PUSH IS REQUIRED
   - ✅ CRITICAL: TRIPLE-CHECK the count before pushing - count MUST be exactly 5 or greater

   - Push to remote ONLY in these two specific scenarios, NO EXCEPTIONS:
     
     **SCENARIO 1: User-explicitly-requested push**
     When user explicitly requests with commands like "push", "git push", "please push", "push to remote", or "push changes":
     1. Execute: `git log origin/main..HEAD --oneline | wc -l` and verify there are commits to push
     2. Execute: `git log origin/main..HEAD --oneline`
     3. Show output to user with message: "The following commits will be pushed:"
     4. Execute: `git push origin HEAD`
     5. Execute: `git rev-parse --abbrev-ref HEAD` (to get current branch name)
     6. Execute: `git rev-parse HEAD` (to get commit hash)
     7. Report to user: "Successfully pushed branch {branch_name} to origin. HEAD is now at {commit_hash}."
     
     **SCENARIO 2: REQUIRED AUTOMATIC PUSH WHEN COMMITS ≥ 5**
     When count of unpushed commits reaches EXACTLY 5 or more:
     1. VERIFY count with `git log origin/main..HEAD --oneline | wc -l` and ensure result is ≥ 5
     2. ⚠️ EXECUTE PUSH IMMEDIATELY WITHOUT DELAY OR USER INTERACTION ⚠️
     3. DO NOT wait for user confirmation - this is MANDATORY AUTOMATIC PUSH
     4. DO NOT suggest pushing - JUST PUSH
     5. Execute immediately in this exact sequence:
        a. `git log origin/main..HEAD --oneline`
        b. Inform user: "Automatically pushing changes to remote since there are 5 or more unpushed commits."
        c. Show commit list with message: "The following commits will be pushed:"
        d. `git push origin HEAD` IMMEDIATELY
        e. `git rev-parse --abbrev-ref HEAD` (to get current branch name)
        f. `git rev-parse HEAD` (to get commit hash) 
        g. Report: "Successfully pushed branch {branch_name} to origin. HEAD is now at {commit_hash}."
     
   - IMPLEMENTATION CHECKS:
     - After EVERY commit operation, you MUST check unpushed commit count ACCURATELY
     - Execute `git log origin/main..HEAD --oneline | wc -l` and VERIFY the count is ≥ 5
     - If count ≥ 5, you MUST push AUTOMATICALLY without asking
     - If count < 5, you MUST NOT push automatically
     - Failure to auto-push at 5+ commits is a CRITICAL RULE VIOLATION
     - No exceptions for any reason whatsoever - automatic push at 5+ commits is MANDATORY
     - Automatic push at < 5 commits is ABSOLUTELY PROHIBITED

   - ABSOLUTELY PROHIBITED:
     - ❌ NEVER ask for push confirmation when 5+ commits exist
     - ❌ NEVER delay automatic push when threshold is reached
     - ❌ NEVER wait for user prompt when 5+ commits exist
     - ❌ NEVER suggest pushing instead of auto-pushing at threshold
     - ❌ NEVER push in circumstances other than the two scenarios above
     - ❌ NEVER push automatically when count < 5 (CRITICAL PROHIBITION)
     - ❌ NEVER fail to double-check the count before pushing

## 📂 FILE ORGANIZATION RULES

### File Placement

1. **ALWAYS verify directories before writing files**
   - ✅ REQUIRED: Check if target directory exists BEFORE creating or writing files
   - ✅ REQUIRED: Create directories if they don't exist: `Path(directory).mkdir(parents=True, exist_ok=True)`
   - ✅ REQUIRED: Handle directory creation and file write failures gracefully
   
   **CORRECT DIRECTORY VERIFICATION:**
   ```python
   # ALWAYS use this pattern when creating files
   from pathlib import Path
   
   # Path to create file in
   file_path = Path("/path/to/directory/file.txt")
   
   # Ensure directory exists (creates parent directories if needed)
   file_path.parent.mkdir(parents=True, exist_ok=True)
   
   # Now safe to write to file
   with open(file_path, 'w') as f:
       f.write("File contents")
   ```

2. **ALWAYS follow project's file organization**
   - ✅ REQUIRED: Examine existing files to understand the project's organization
   - ✅ REQUIRED: Place new files in appropriate directories based on their function
   - ✅ REQUIRED: Follow naming conventions used in the project
   - ✅ REQUIRED: Confirm file placement with user if uncertain
   
   **CORRECT FILE PLACEMENT:**
   ```
   src/              # Source code
     components/     # React components
     utils/          # Utility functions
     api/            # API-related code
   tests/            # Test files
   docs/             # Documentation
   scripts/          # Utility scripts
   ```

3. **NEVER create redundant files**
   - ✅ REQUIRED: Check if functionality exists before creating new files
   - ✅ REQUIRED: Reuse existing modules and components when appropriate
   - ❌ PROHIBITED: Never duplicate functionality in multiple files
   - ✅ REQUIRED: Refactor existing code rather than creating duplicates

### Temporary Files

1. **ALWAYS place temporary files in the correct location**
   - ✅ REQUIRED: All temporary files created by Claude MUST be placed in `claude_tests/tmp/`
   - ✅ REQUIRED: All temp files from tests in `claude_tests/` MUST use `claude_tests/tmp/`
   - ✅ REQUIRED: Use Python tempfile module when appropriate
   - ✅ REQUIRED: Prefix all temporary files with "claude_tmp_"
   - ✅ REQUIRED: Add this header to all temporary files:
     ```
     # Claude Temporary File
     # Created by Claude on YYYY-MM-DD
     # This is a temporary file and can be safely deleted
     ```
   - ✅ REQUIRED: Inform user when creating files in `claude_tests/tmp/`
   - ✅ REQUIRED: Remove temporary files when no longer needed
   
   **CORRECT TEMPORARY FILE CREATION:**
   ```python
   from pathlib import Path
   
   # Ensure tmp directory exists
   tmp_dir = Path("claude_tests/tmp")
   tmp_dir.mkdir(parents=True, exist_ok=True)
   
   # Create temporary file with required prefix and header
   tmp_file = tmp_dir / "claude_tmp_analysis.txt"
   with open(tmp_file, 'w') as f:
       f.write("# Claude Temporary File\n")
       f.write("# Created by Claude on 2023-11-01\n")
       f.write("# This is a temporary file and can be safely deleted\n\n")
       f.write("Analysis results:\n")
       # ...write actual content...
   ```

2. **ALWAYS follow these temporary file permissions**
   - ✅ PERMITTED: Read files in `/tmp` and `claude_tests/tmp` without user approval
   - ✅ PERMITTED: Write files to `claude_tests/tmp` without user approval
   - ✅ REQUIRED: Follow naming convention (prefix with "claude_tmp_")
   - ✅ REQUIRED: Still inform user when creating new files in these directories

### Claude Scripts Organization

1. **ALWAYS organize Claude scripts by purpose and type**
   - ✅ REQUIRED: Follow these prefix conventions:
     - Action scripts: `claude_action_*.py` in `claude_tests/actions/` (one-time operations)
     - Test scripts: `claude_test_*.py` in component subdirectories (verification/testing)
     - Temporary files: `claude_tmp_*` in `claude_tests/tmp/` (data/logs/outputs)
   
   **ACTION SCRIPT EXAMPLE:**
   ```python
   # claude_action_migrate_db.py
   """
   One-time script to migrate database schema.
   Created by Claude on 2023-11-01.
   
   Usage:
   python claude_tests/actions/claude_action_migrate_db.py
   """
   ```
   
   **TEST SCRIPT EXAMPLE:**
   ```python
   # claude_test_api_endpoints.py
   """
   Test script to verify API endpoints.
   Created by Claude on 2023-11-01.
   
   Usage:
   python claude_tests/api/claude_test_api_endpoints.py
   """
   ```

## 🧪 TEST ORGANIZATION RULES

### Test Types and Locations

1. **ALWAYS use the correct test type for the purpose**
   - ✅ REQUIRED: Understand and communicate these test type differences:
     - **Formal Unit Tests**: In `/tests/` directory, part of official test suite
     - **Formal Integration Tests**: In `/tests/integration/`, test component interactions
     - **Claude Test Scripts**: In `/claude_tests/`, development verification only
   
   - ❌ PROHIBITED: Never mix test types or locations
   - ✅ REQUIRED: Always place tests in the correct directory
   
   **TEST TYPE DECISION TREE:**
   ```
   Is this a formal test that should be run in CI/CD?
                 ↓
                YES
                 ↓
   Is it testing a single unit?  → YES → Place in /tests/unit/
                 ↓
                 NO
                 ↓
   Is it testing component integration? → YES → Place in /tests/integration/
                 ↓
                 NO
                 ↓
   Is this a Claude development test?  → YES → Place in /claude_tests/[component]/
                 ↓
                 NO
                 ↓
   Default to Claude test in /claude_tests/[component]/
   ```

2. **ALWAYS use correct subdirectories under `claude_tests/`**
   - ❌ PROHIBITED: Never create tests directly in `claude_tests/` root
   - ✅ REQUIRED: Use subdirectories matching the component being tested:
     ```
     claude_tests/schema/      # Schema system tests
     claude_tests/config/      # Configuration system tests
     claude_tests/logging/     # Logging system tests
     claude_tests/cli/         # CLI tests
     claude_tests/commands/    # Command implementation tests
     claude_tests/interactive/ # Interactive features tests
     claude_tests/query/       # Query system tests
     claude_tests/models/      # Model-related tests
     claude_tests/imports/     # Import system tests
     claude_tests/integration/ # Cross-component integration tests
     claude_tests/stats/       # Statistics module tests
     ```
   - ✅ REQUIRED: Create new subdirectory if appropriate one doesn't exist
   - ✅ REQUIRED: Always include `__init__.py` in each subdirectory

### Claude Test Script Requirements

1. **ALWAYS include proper documentation in Claude test scripts**
   - ✅ REQUIRED: Add a clear docstring explaining purpose of the test
   - ✅ REQUIRED: Include creation date in the docstring
   - ✅ REQUIRED: Add usage instructions in the docstring
   - ✅ REQUIRED: Add comments explaining complex test logic
   - ✅ REQUIRED: Include header comment: `# CLAUDE TEST SCRIPT`
   
   **CORRECT TEST SCRIPT EXAMPLE:**
   ```python
   # CLAUDE TEST SCRIPT
   # This is a Claude test script for development and verification purposes only.
   # It is not part of the formal test suite and should not be moved to the tests/ directory.
   # See .claude/rules.md for more information.
   
   """
   Test script to verify config parser functionality.
   Created by Claude on 2023-11-01.
   
   This script tests the config parser's ability to handle:
   - Invalid configurations
   - Missing required fields
   - Type validation
   
   Usage:
   python claude_tests/config/claude_test_config_parser.py
   """
   ```

2. **ALWAYS design Claude tests for easy execution**
   - ✅ REQUIRED: Make tests runnable from the command line
   - ✅ REQUIRED: Include main block (`if __name__ == "__main__":`) for direct execution
   - ✅ REQUIRED: Show clear pass/fail output
   - ✅ REQUIRED: Handle exceptions and display useful error messages
   - ✅ REQUIRED: Use tempfile module for temporary files
   - ✅ REQUIRED: Place temporary files in `claude_tests/tmp/` with "claude_tmp_" prefix
   - ✅ REQUIRED: Add cleanup to remove temporary files when done

### Test Execution Environment

1. **ALWAYS document test execution environment**
   - ✅ REQUIRED: Include information about required dependencies
   - ✅ REQUIRED: Specify any environment setup needed (env vars, etc.)
   - ✅ REQUIRED: Handle environment setup and teardown in the test
   - ✅ REQUIRED: Make tests self-contained when possible
   
   **CORRECT ENVIRONMENT DOCUMENTATION:**
   ```python
   """
   Environment requirements:
   - Python 3.10+
   - Access to test database (TEST_DB_URL environment variable)
   - API key in .env file or OPENAI_API_KEY environment variable
   """
   ```

2. **ALWAYS ensure proper module access in tests**
   - ✅ REQUIRED: Use relative imports for internal modules
   - ✅ REQUIRED: Add project root to sys.path if needed:
     ```python
     import sys
     from pathlib import Path
     sys.path.insert(0, str(Path(__file__).parent.parent.parent))
     ```
   - ✅ REQUIRED: Document any special import requirements
   - ✅ REQUIRED: Handle import errors with clear error messages

## 📊 CODE QUALITY RULES

1. **ALWAYS follow the project's style guide**
   - ✅ REQUIRED: Check for style guides in the repository (e.g., .eslintrc, pyproject.toml)
   - ✅ REQUIRED: Match existing code style if no explicit style guide exists
   - ✅ REQUIRED: Follow language-specific conventions (PEP 8 for Python, etc.)
   - ✅ REQUIRED: Maintain consistent indentation and formatting

2. **NEVER leave debugging code in production**
   - ❌ PROHIBITED: Never leave print statements or console.log in production code
   - ❌ PROHIBITED: Never commit commented-out code blocks
   - ❌ PROHIBITED: Never leave TODOs without a clear reason
   - ✅ REQUIRED: Remove all debugging artifacts before committing

3. **ALWAYS write clear error handling**
   - ✅ REQUIRED: Handle expected errors gracefully
   - ✅ REQUIRED: Provide clear error messages
   - ✅ REQUIRED: Include context in error messages (function name, operation, etc.)
   - ✅ REQUIRED: Log errors appropriately
   - ✅ REQUIRED: Avoid swallowing exceptions without logging

4. **ALWAYS write good documentation**
   - ✅ REQUIRED: Document public APIs and functions
   - ✅ REQUIRED: Include docstrings for modules, classes, and functions
   - ✅ REQUIRED: Explain complex or non-obvious code
   - ✅ REQUIRED: Update documentation when changing code
   - ✅ REQUIRED: Add examples for complex functionality

## 🛠 ENVIRONMENT MANAGEMENT RULES

1. **NEVER hardcode sensitive information**
   - ❌ PROHIBITED: Never commit API keys, passwords, or tokens
   - ❌ PROHIBITED: Never hardcode environment-specific configuration
   - ✅ REQUIRED: Use environment variables or configuration files
   - ✅ REQUIRED: Document required environment variables
   
   **CORRECT ENVIRONMENT VARIABLE USAGE:**
   ```python
   import os
   from dotenv import load_dotenv
   
   load_dotenv()  # Load variables from .env file
   
   api_key = os.environ.get("API_KEY")
   if not api_key:
       raise EnvironmentError("API_KEY environment variable is required")
   ```

2. **ALWAYS respect gitignore settings**
   - ✅ REQUIRED: Check .gitignore before creating new files
   - ✅ REQUIRED: Never suggest committing files that match .gitignore patterns
   - ✅ REQUIRED: Follow the project's conventions for ignored files
   - ✅ REQUIRED: Warn user if they request committing ignored files

3. **ALWAYS document environment setup**
   - ✅ REQUIRED: Document required dependencies
   - ✅ REQUIRED: Document environment variables
   - ✅ REQUIRED: Document setup steps for new developers
   - ✅ REQUIRED: Include information about development tools

4. **NEVER make assumptions about the user's environment**
   - ❌ PROHIBITED: Never assume specific tools are installed
   - ❌ PROHIBITED: Never assume specific environment variables are set
   - ✅ REQUIRED: Check for required tools and environment variables
   - ✅ REQUIRED: Provide clear error messages when requirements aren't met