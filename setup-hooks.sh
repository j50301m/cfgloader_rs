#!/bin/bash

# Setup script for Git hooks in this Rust project

set -e

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}Setting up Git hooks for CFGLoader project...${NC}"

# Make hooks executable
chmod +x .githooks/pre-push

# Configure Git to use our hooks directory
git config core.hooksPath .githooks

echo -e "${GREEN}✅ Git hooks setup complete!${NC}"
echo ""
echo "📝 Available hooks:"
echo "   • pre-push: Runs cargo fmt, clippy, check, and tests before pushing"
echo ""
echo "🔧 Usage tips:"
echo "   • To skip tests during push: SKIP_TESTS=1 git push"
echo "   • To bypass all hooks: git push --no-verify"
echo ""
echo "🎉 You're all set! The hooks will now run automatically on git push."
