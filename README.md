# claw-safeskills
This is a repository of miscellaneous skills for OpenClaw agents that are designed to be safe, non-destructive, and non-intrusive.

Instructions are only available for macOS at this time, ymmv with Linux and Windows.

## Skills

All skills follow the expected [Agent Skills](https://agentskills.io/specification) specification.

### exa-search
Performs a search using the Exa Answers API to retrieve high-quality, answer-focused results for a given query. Use this when the user asks a question that requires external information or specific answers from the web.

Necessary Secrets:
- `exa-search-skill:API_KEY`

### nano-banana
>Replaces the default *extremely insecure* implementation.

Generates high-quality images using the Nano Banana Pro API. Use this when the user requests an image generation or "Make me an image".

Necessary Secrets:
- `nano-banana-skill:API_KEY`

## Install a safeskill:

> See the [Dependencies](#dependencies) section before getting started.


1. Clone this repository

2. Build the skill(s):
    ```bash
    # From the project root directory (builds all skills)
    make

    # Or build a specific skill
    make <skill-name>
    ```

3. Copy the skill to the OpenClaw skills directory
    ```bash
    # Global installation (all workspaces)
    cp -r <skill-name> ~/.openclaw/skills/

    # Per-workspace installation
    cp -r <skill-name> ~/.openclaw/workspaces/<workspace-name>/skills/
    ```

4. Dry-run the skill to grant permissions:
```bash
<path-to-skills>/<skill-name>/scripts/<skill-name>
# e.g., ~/.openclaw/skills/nano-banana/scripts/nano-banana
```
>A dialogue will appear requesting permission to access the secret(s) for the skill. Verify the program name matches the skill name, then grant it with "Always Allow".


## Add a secret
```bash
security add-generic-password -s <skill-name>-skill -a <secret-name> -w <secret-value>
# e.g., security add-generic-password -s nano-banana-skill -a API_KEY -w AI...8o
```

## Dependencies

### Build and install:
- Rust: https://rust-lang.org/tools/install/ 
- Make: https://www.gnu.org/software/make/

### Run:
- A **keyring** provider for secret storage:
    - macOS: "Keychain Access" (pre-installed)
    - Linux: Any keyring provider that implements "keyutils" or "Secrets Service" (e.g., gnome-keyring)
    - Windows: "Windows Credential Manager" (pre-installed)
- Necessary API keys/secrets stored in the keyring.


## Design Philosophy

All skills are written in Rust to reduce bloat, increase speed, and *most importantly* ensure safe access to secrets and other confidential information.

Secrets are stored in the system keychain and are accessed using the `keyring` crate. Unlike OpenClaw's approach, which stores secrets in plaintext on disk and environment variables, this ensures that neither the agent nor any other skill can access the API key.

A compiled language (Rust) was chosen so that the user does not have to grant "always allow" access to a generic runtime like Node.js or Python. If Node or Python were used and the user granted "always allow", then any code running under Node or Python could access the API key.

