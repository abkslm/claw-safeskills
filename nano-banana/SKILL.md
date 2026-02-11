---
name: nano-banana
description: Generates high-quality images using the Nano Banana Pro API. Use this when the user requests an image generation or "Make me an image".
allowed-tools: RunCommand
---

# Nano Banana Skill

This skill allows the agent to generate images using the Nano Banana Pro API.

## Usage

To use this skill, the agent should run the `nano-banana` binary located in the `scripts` directory.

### Prerequisites

- The Nano Banana Pro API key must be available via:
    1.  Stored in the system keychain (service: `gemini-api-key`, account: `GEMINI_API_KEY`).

### Instructions

1.  **Prepare the Prompt**: Formulate a clear and descriptive prompt for the image.
```bash
# Generate an image with prompt, optional filename, and resolution
skills/nano-banana/scripts/nano-banana --prompt "A cute robot" --filename "robot.png" --resolution "4K"
```

### Script Arguments

- `--prompt` (required): The text description of the image to generate.
- `--filename` (optional): The name of the file to save the image to.
- `--resolution` (optional): The resolution (1K, 2K, 4K). Defaults to 1K.

### Output

The script will decode the generated image and save it to the current directory with a filename `image_<timestamp>.<ext>`.
It outputs a JSON object containing the status and the path to the saved file.

## Examples

**Example 1: Basic Generation**

```bash
skills/nano-banana/scripts/nano-banana "A futuristic cityscape with neon lights"
```

**Output:**

```json
{
  "status": "success",
  "message": "Image generated and saved.",
  "mime_type": "image/jpeg",
  "file_path": "image_1739238355.jpeg"
}
```
