# todo

A command line tool to detect TODOs in a codebase associated with a given issue ID.

 `todo <path-to-codebase> --issue <issue-id>`

By providing a base branch the tool can extract issues from the commit messages from the current branch ahead of a given base branch and emits an error if unresolved TODOs are found for issues that the commits claim to fix.

 `todo <path-to-codebase> --git <base-branch>`
