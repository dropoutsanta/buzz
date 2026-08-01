# Fam public changes awaiting upstream

This public Buzz checkout tracks `upstream` → `https://github.com/block/buzz.git`
and `origin` → your public fork (set the remote URL before first push).

## Candidate contributions

1. **buzz-cli markdown newline normalization** — expands obvious `\\n\\n` /
   list escapes in `messages send` argv content so agent-rendered Markdown
   publishes as real newlines. Tests included. No Fam-specific context.

2. **Desktop storage namespace** — optional `BUZZ_STORAGE_NAMESPACE` isolates
   release keychain (`buzz-desktop-<ns>`) and nest (`~/.buzz-<ns>`) for
   externally branded app variants. Default behavior unchanged.

3. **Rounded macOS app icon** — squircle-masked `icon.icns` / PNG set so Dock
   and Force Quit match system icon chrome when running as a real `.app`.

## Remotes

```bash
git remote -v
# origin    https://github.com/dropoutsanta/buzz.git   (public fork — configure before push)
# upstream  https://github.com/block/buzz.git
```
