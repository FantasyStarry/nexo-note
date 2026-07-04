#!/usr/bin/env bash
# Install the nexo-note skill into the local Trae or Claude Code skills directory.

set -e

SOURCE_DIR="$(cd "$(dirname "$0")" && pwd)"

# Try Trae first, then Claude Code
if [ -d "$HOME/.trae-cn/skills" ]; then
    TARGET_DIR="$HOME/.trae-cn/skills/nexo-note"
elif [ -d "$HOME/.claude/skills" ]; then
    TARGET_DIR="$HOME/.claude/skills/nexo-note"
else
    TARGET_DIR="$HOME/.trae-cn/skills/nexo-note"
fi

echo "Installing nexo-note skill to $TARGET_DIR ..."

mkdir -p "$TARGET_DIR"
cp "$SOURCE_DIR/skill.yaml" "$TARGET_DIR/"
cp "$SOURCE_DIR/instructions.md" "$TARGET_DIR/"
cp "$SOURCE_DIR/examples.md" "$TARGET_DIR/"

echo "nexo-note skill installed successfully."
