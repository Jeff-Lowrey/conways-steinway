#!/bin/bash

# Script to fetch and display PR status in Claude Code
# This script requires the GitHub CLI (gh) to be installed and authenticated

# Set this to your repository
REPO="Jeff-Lowrey/conways-steinway"

# ANSI color codes for formatting
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
NC='\033[0m' # No Color
BOLD='\033[1m'

echo -e "${BOLD}${BLUE}🔍 PR Status for ${REPO}${NC}\n"

# Fetch open PRs
echo -e "${BOLD}Open PRs:${NC}"
gh pr list --repo "$REPO" --limit 5 --json number,title,author,reviewDecision,mergeable,baseRefName,headRefName,url | \
  jq -r '.[] | "PR #\(.number): \(.title) by @\(.author.login) [\(.baseRefName) ← \(.headRefName)] \(.url)"' | \
  while read -r line; do
    echo -e "  ${GREEN}•${NC} $line"
  done

echo ""

# Fetch recent PR activity
echo -e "${BOLD}Recent PR Activity:${NC}"
gh pr list --repo "$REPO" --state all --limit 5 --json number,title,state,updatedAt,author,url | \
  jq -r '.[] | "\(.updatedAt) - PR #\(.number): \(.title) [\(.state)] by @\(.author.login) \(.url)"' | \
  while read -r line; do
    if [[ "$line" == *"[MERGED]"* ]]; then
      echo -e "  ${PURPLE}•${NC} $line"
    elif [[ "$line" == *"[CLOSED]"* ]]; then
      echo -e "  ${YELLOW}•${NC} $line"
    else
      echo -e "  ${BLUE}•${NC} $line"
    fi
  done

# Show PR mentions for the user
echo -e "\n${BOLD}PR Mentions:${NC}"
gh api graphql -f query='
  query {
    search(query: "repo:'$REPO' is:pr is:open mentions:@me", type: ISSUE, first: 5) {
      edges {
        node {
          ... on PullRequest {
            number
            title
            url
            author {
              login
            }
            updatedAt
          }
        }
      }
    }
  }
' | jq -r '.data.search.edges[] | .node | "PR #\(.number): \(.title) by @\(.author.login) \(.url)"' | \
  while read -r line; do
    if [ -n "$line" ]; then
      echo -e "  ${YELLOW}•${NC} $line"
    fi
  done

if [ -z "$(jq -r '.data.search.edges[] | .node | "PR #\(.number)"' <<< "$(gh api graphql -f query='
  query {
    search(query: "repo:'$REPO' is:pr is:open mentions:@me", type: ISSUE, first: 5) {
      edges {
        node {
          ... on PullRequest {
            number
          }
        }
      }
    }
  }
')")" ]; then
  echo -e "  ${GREEN}•${NC} No PR mentions found"
fi

echo -e "\n${BOLD}${BLUE}End of PR Status Report${NC}"