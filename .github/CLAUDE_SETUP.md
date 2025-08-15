# Setting Up Claude Code Integration

This repository uses Claude Code to provide assistance with code reviews, PR notifications, and answering code-related questions.

## Setup Instructions

To fully enable Claude Code integration, follow these steps:

### 1. Add Anthropic API Key as a Secret

1. **Get an Anthropic API Key**:
   - Sign up at [Anthropic's website](https://console.anthropic.com/)
   - Create an API key in your account settings

2. **Add the API Key to Repository Secrets**:
   - Go to your repository on GitHub
   - Navigate to Settings > Secrets and variables > Actions
   - Click "New repository secret"
   - Name: `ANTHROPIC_API_KEY`
   - Value: Your Anthropic API key
   - Click "Add secret"

### 2. Verify the GitHub Actions Workflow

- Ensure the `.github/workflows/claude-code.yml` file is present in your repository
- No additional configuration is needed for the workflow itself

## Using Claude Code

Once set up, you can use Claude Code in the following ways:

### Code Reviews

Comment on any PR with:
```
@claude review
```

For more specific reviews:
```
@claude review for security issues
@claude review for performance
@claude suggest improvements for this code
```

### Code Questions

Ask questions about the codebase:
```
@claude How does the piano.py module work?
@claude What's the relationship between game_board.rs and audio.rs?
@claude Explain the configuration system in this project
```

### Automatic Notifications

Claude will automatically notify you when:
- PRs are merged
- New issues are linked to PRs
- Changes are requested in reviews

## Troubleshooting

If Claude is not responding to comments:

1. Check that the `ANTHROPIC_API_KEY` secret is properly set
2. Verify that the GitHub Actions workflow is enabled
3. Look at the GitHub Actions logs for any errors
4. Make sure you're using the `@claude` mention correctly

For more help, see the [Claude Code GitHub Action documentation](https://github.com/anthropics/claude-code-action).