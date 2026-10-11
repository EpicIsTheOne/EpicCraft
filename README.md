# EpicCraft

Epic's custom launcher and bug-fix workspace for the open-source Craft App Suite.
Based on [ArtCraft Launcher](https://github.com/storytold/craft-launcher) and the
[twelve Craft apps](https://getartcraft.com/apps). Independent community fork.

Download the Windows launcher from [Releases](https://github.com/EpicIsTheOne/EpicCraft/releases).
Extract the portable zip, then run `artcraft-launcher.exe` inside it. Keep
`portable.txt` beside the executable. Automatic updates are enabled by default.

## How updates work

The launcher checks every five minutes while running. It prefers our published,
SHA-256-verified app builds from `epiccraft/releases.json`. Apps we have not changed
use official releases. A running app is never replaced; close it when ready for
an update. The launcher updates itself from EpicCraft, so it stays on this fork.
It can run in the tray and start at login. Projects and app settings remain in
the apps' existing locations.

Saving source does not install an untested build. Push a finished fix to `main`:
the Windows workflow detects affected apps, tests and builds them, publishes a
versioned release, then updates the index only after every selected build succeeds.
The Windows pipeline is the supported fork release path; other platforms currently
use official app packages. A failed build never becomes an automatic update.

## Report bugs

Send Dexy the app name, version, steps to reproduce, and what you expected.
You can also [file a bug](https://github.com/EpicIsTheOne/EpicCraft/issues/new/choose).
Use a small sample project when useful, and avoid sharing private media publicly.

## Development

- Launcher: root Cargo workspace, `cargo xtask ci` and `cargo build --release -p artcraft-launcher`.
- Apps: independent workspaces under `suite/<app>`, with original licenses,
  attribution and agent instructions. Fix each app there, run its relevant tests,
  and push to trigger its build. Build numbers use a numeric patch offset above
  upstream so Windows Installer recognizes our fixes as upgrades.
- Source snapshot provenance: `epiccraft/upstream.json`.
- Shared app dependencies retain their upstream pinned Git revisions.
- Install a local launcher build and shortcuts: `./epiccraft/install-launcher.ps1`.
- Manually publish an app: dispatch **EpicCraft Windows releases** with its app id.

The initial suite import is unchanged upstream source. It installs the official
Windows apps and supplies the customized launcher. App-specific bug fixes will
be added as issues are reported.

## License and attribution

Retains the upstream MIT OR Apache-2.0 licensing and all existing notices.
See `LICENSE-MIT`, `LICENSE-APACHE`, `ATTRIBUTION.md`, and each app's license files.
Original trademarks and supplied brand assets belong to their owners. EpicCraft
is an independent fork and is not endorsed by ArtCraft. The upstream launcher
README is preserved in `README-UPSTREAM.md`.