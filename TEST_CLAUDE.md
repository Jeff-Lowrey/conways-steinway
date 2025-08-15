# Claude Integration Test

This file is created to test the Claude Code integration with GitHub.

## Test Goals

1. Verify that Claude responds to mentions in PR comments
2. Test code review capabilities
3. Confirm that notifications work when PRs are merged

## Sample Code for Review

```python
def fibonacci(n):
    """Return the nth Fibonacci number."""
    if n <= 0:
        return 0
    elif n == 1:
        return 1
    else:
        return fibonacci(n-1) + fibonacci(n-2)
```

## Expected Outcome

When this PR is commented on with `@claude review`, Claude should analyze the code and suggest improvements (like using memoization for the Fibonacci function to improve performance).