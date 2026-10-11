# EpicCraft issue desk

Dashboard and submission form: https://techexplore.us/epiccraft/

The launcher Feedback button selects an app and opens the screenshot-enabled report
form. The dashboard saves edited context before copying a link or dispatching it.
Screenshots are decoded, validated, and re-encoded to remove metadata. Limits are
five images, 5 MB each and 15 MB total. Reports and original submissions are retained
in SQLite outside the public checkout.

Receipt links are read-only. Dashboard-generated agent links grant access to one
report and its completion endpoint. Anyone holding an agent link can read that
report and mark it done. Keep links private. The dashboard password is stored as a
salted scrypt hash in a root-readable runtime environment file, never in Git.

The PC runs `bridge.cjs` using an external config with `url`, `token`, `codex`,
`codexHome`, and `project`. It polls KVM2 over HTTPS and starts persistent Codex
app-server threads with the current project directory. Jobs are serialized because
they use the same checkout. Offline PCs leave reports queued. Local thread/turn
receipts prevent repeated dispatch, and interrupted ambiguous starts require review.
Finishing a model turn does not mark a report done. Codex must explicitly call the
per-report completion endpoint after verifying the fix. Approval requests are never
automatically accepted by this bridge.

Runtime environment: `EPICCRAFT_DATA`, `EPICCRAFT_ORIGIN`,
`EPICCRAFT_PASSWORD_HASH` (hex salt:hex scrypt result), `EPICCRAFT_BRIDGE_TOKEN`.
Run `uvicorn server:app --host 0.0.0.0 --port 8946 --proxy-headers` behind TLS.
Trust forwarded client addresses only from the reverse proxy. Back up the database
with SQLite's backup API together with its images directory. Static source contains
no credentials or report data.
