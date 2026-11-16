# Wild Magic Academy project guidance

## Scope

This repository contains the Bevy core prototype, separate development
workstreams, design documents/source art, and an earlier Godot experiment.
Read the root README, PROJECT.md and the relevant workstream brief before edits.
User instructions and later accepted playtest decisions supersede the design draft.

## Validation

For core gameplay changes, run `projects/core-gameplay/check.sh`. For full Bevy
adapter changes, source `projects/core-gameplay/scripts/environment.sh` and run
the relevant Cargo tests, strict Clippy and build with the adapter manifest.
GUI/network actions may need approved execution outside the sandbox. Do not claim
manual gameplay tests passed merely because compilation or window startup works.

## Git progress

The user wants the whole project pushed to their Git repository as work progresses.
After a meaningful, validated milestone, review changes, commit them and push to
the configured remote/current working branch. This ongoing authorization does
not include force-pushing, replacing existing remote history, deleting remote
branches, changing repository visibility, or publishing unrelated local content.
When no remote or author identity exists, prepare the changes and obtain the
missing repository/identity details; do not invent an account or email address.

Keep source, tests, briefs, lockfiles, final design documents and source artwork
in Git. Never stage credentials, `.cache`, `target`, Godot engine downloads, or
regenerable document review exports. Review the staged file list before pushing.
