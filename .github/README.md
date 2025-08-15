# Conway's Steinway GitHub Integration

This directory contains GitHub-specific configurations for the Conway's Steinway project.

## Claude Code Integration

This repository is integrated with Claude Code, which provides:

1. **PR Notifications**: Claude will notify you when PRs are merged
2. **Code Reviews**: Comment with `@claude review` on a PR to get Claude's feedback
3. **Code Questions**: Comment with `@claude <your question>` on PRs or issues to ask questions about the codebase

### Example Usage

- `@claude review this PR` - Get a full code review
- `@claude what does this function do?` - Ask about specific code
- `@claude suggest a test for this code` - Get testing suggestions
- `@claude explain this algorithm` - Get explanations of complex code

### Automatic Notifications

Claude Code will automatically notify you when:

- PRs are merged
- New issues are linked to existing PRs
- PR reviews are requested

## GitHub Actions

The following GitHub Actions are configured:

- `claude-code.yml`: Handles Claude Code integration for PRs and issues

## Repository Settings

Key repository settings:

- Branch protection rules enforce PR reviews before merging
- Squash merging is enabled by default
- Issue templates guide contributors to provide necessary information