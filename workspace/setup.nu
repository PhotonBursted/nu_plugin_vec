use std log


# ==== SET UP GIT PRE-COMMIT HOOK ===
log info "Setting up pre-commit hooks"
log debug (^pipx install pre-commit | complete).stdout
log debug (^pre-commit install --hook-type commit-msg | complete).stdout


log info "✅ Set up workspace, ready to go!"