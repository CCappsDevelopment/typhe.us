#!/bin/bash

# =============================================================================
# Git Cleanup Script for Svelte/Rust Application
# =============================================================================
# This script removes files from git tracking that should be ignored
# according to the new .gitignore file.
#
# WARNING: This will remove files from git tracking. Make sure you have
# backups and understand what files will be removed before running this.
#
# Usage: ./cleanup-git.sh [--dry-run]
# Use --dry-run to see what would be removed without actually removing it.

set -e

DRY_RUN=false
if [[ "$1" == "--dry-run" ]]; then
    DRY_RUN=true
    echo "🔍 DRY RUN MODE - No files will actually be removed"
    echo "========================================================"
fi

echo "🧹 Git Cleanup Script for Typhe.us"
echo "===================================="
echo

# Function to remove files from git tracking
remove_from_git() {
    local pattern="$1"
    local description="$2"
    
    echo "📂 $description"
    echo "   Pattern: $pattern"
    
    # Find files matching the pattern
    local files=$(git ls-files | grep -E "$pattern" || true)
    
    if [[ -n "$files" ]]; then
        echo "   Files found:"
        echo "$files" | sed 's/^/     - /'
        
        if [[ "$DRY_RUN" == "false" ]]; then
            echo "$files" | xargs git rm --cached
            echo "   ✅ Removed from git tracking"
        else
            echo "   🔍 Would be removed in actual run"
        fi
    else
        echo "   ℹ️  No files found matching pattern"
    fi
    echo
}

# Remove OS files
remove_from_git "^\.DS_Store$|^\._.DS_Store$" "macOS system files"

# Remove Rust build artifacts
remove_from_git ".*/target/.*" "Rust build artifacts (target directories)"

# Remove Node.js artifacts
remove_from_git ".*/node_modules/.*" "Node.js dependencies"
remove_from_git ".*/build/.*" "Frontend build artifacts"
remove_from_git ".*\.svelte-kit/.*" "SvelteKit build artifacts"

# Remove WASM build artifacts (keep source, remove generated)
remove_from_git ".*\.wasm$" "WebAssembly compiled files"
remove_from_git ".*\.wasm\.d\.ts$" "WebAssembly TypeScript definitions"

# Remove log files
remove_from_git ".*\.log$" "Log files"

# Remove environment files (if any)
remove_from_git ".*\.env$|.*\.env\..*" "Environment files"

# Remove SSL certificates (if any)
remove_from_git ".*\.pem$|.*\.key$|.*\.crt$|.*\.cert$" "SSL certificates"

if [[ "$DRY_RUN" == "false" ]]; then
    echo "🎯 Summary"
    echo "=========="
    echo "Files have been removed from git tracking but still exist on disk."
    echo "To commit these changes, run:"
    echo "  git add .gitignore .dockerignore"
    echo "  git commit -m 'Add comprehensive .gitignore and .dockerignore files'"
    echo
    echo "⚠️  Important Notes:"
    echo "- Build artifacts will be regenerated when needed"
    echo "- Make sure your build processes can recreate removed files"
    echo "- The files still exist on disk, just not tracked by git"
else
    echo "🔍 Dry run complete"
    echo "=================="
    echo "To actually perform the cleanup, run:"
    echo "  ./cleanup-git.sh"
fi
