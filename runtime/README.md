# runtime

The Simba engine and everything it needs to run the scripts. Driven by the
Elixer Scripts launcher (see the [root README](../README.md)).

## Credits & licensing

| Component | Author | License |
|-----------|--------|---------|
| Simba (`Simba64.exe`) | Villavu | GPL-3.0 |
| SRL-T (`Includes/SRL-T_v1`, `_v2`) | Torwent (fork of SRL) | GPL-3.0 |
| WaspLib (`Includes/WaspLib_v1`, `_v2`) | Torwent | GPL-3.0 |

The GPL libraries are vendored unmodified apart from the offline patches
below; original license texts and copyright headers are left intact.

`Scripts/` holds all scripts (flat). Most are from various authors (Torwent,
bigaussie, Flight, aetherdescent, bootje, and others) that were sold on
subscription before the platform shut down — **not** GPL, kept here for
personal use only. A few are freely published community scripts (e.g. from
BigAussie's public repo). The author is encoded in each filename (`-by-<name>`).

## Local changes vs. a stock install

Only these, all to run standalone/offline after waspscripts.com went down:

1. **Repointed paths** — `Data/packages.ini` and `Data/settings.ini` point at
   this folder instead of `AppData\Local\Simba`.
2. **Stats telemetry off** — `Configs/wasplib.json` → `"stats": false`.
3. **Headless/offline library patches** — a few spots in `Includes/` so
   scripts run via the launcher's headless `--run` on Simba 1400 (force
   `SIMBAHEADLESS`, guard GUI-only calls, native LoseFocus fallback, skip
   rate-the-game on logout). Each is tagged `// osrs-bot:` —
   `grep -r "osrs-bot:" Includes/` lists them all.

## Two library generations

Scripts target one of two library versions. The launcher repoints the
`Includes/WaspLib` and `Includes/SRL-T` junctions per run:

- **v1** (`SRL-T_v1` / `WaspLib_v1`) — pre-refactor libs, for scripts that
  include `osr.simba`.
- **v2** (`SRL-T_v2` / `WaspLib_v2`) — current libs, for everything else.

## Setting up on a fresh clone

The gitignored binaries below aren't vendored in this repo (see
`.gitignore`'s "re-obtainable binaries" section) and have to be fetched
separately after cloning:

- **`Simba64.exe`** — official Windows 64-bit build from the pinned
  `simba1400-release` tag:
  `https://github.com/Villavu/Simba/releases/download/simba1400-release/Simba-Win64.exe`.
  Note: this trips Windows Defender's `Trojan:Win32/Phonzy.A!ml` heuristic
  (a generic ML false-positive common for input-automation tools) — add a
  Defender exclusion for `runtime/` *before* downloading, or it'll be
  silently deleted on arrival.
- **`Includes/SRL-T_v2/plugins/`** — the native plugin DLLs SRL-T scripts
  load via `{$loadlib}` (`libasyncmouse`, `libremoteinput`, `libsimpleocr`,
  `libslacktree`, `libtpaex`). Not published as GitHub releases — they're
  committed directly as binaries in the upstream repo. Copy the whole
  folder from `https://github.com/Torwent/SRL-T` (`plugins/`).
- **`Includes/WaspLib_v2/plugins/`** — same story for WaspLib's own
  plugins (`ffmpeg.exe`, `libSound.dll`, `librecorder32.dll`,
  `librecorder64.dll`). Copy from `https://github.com/Torwent/WaspLib`
  (`plugins/`).

Without these, `check.ps1`'s Simba compile sweep fails every v2 script with
`Plugin ".../libslacktree/libslacktree" not found` (or similar for the
other plugins).

### v1 libraries not vendored

`Includes/WaspLib_v1` / `SRL-T_v1` (the pre-refactor libs for any script
that includes `osr.simba`) aren't present in this fork at all — only the
`_v2` generation was carried over. Scripts requiring `osr.simba` will fail
to compile until those are fetched too (same repos as above, at whatever
tag/commit predates the refactor).

## Running directly

Launch `Simba64.exe` (or the `osrs-bot Simba` desktop shortcut), open a script
from `Scripts/`, and Run. Scripts compile from source at runtime.
