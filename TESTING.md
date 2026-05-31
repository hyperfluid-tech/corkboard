# Golden Tests

To maintain Corkboard's detailed skeuomorphic styling and layout, the project uses a golden testing suite. This suite captures golden images of individual UI components and page layouts, ensuring that changes (such as accessibility updates or stylesheet tweaks) do not break the design.

---

## How It Works

Golden tests compare the current rendering of the website against a set of "golden" reference images.

```mermaid
graph TD
    A[Seeded PRNG in templates/testing.js] -->|deterministic=true| B(Stable Shaders & Torn Edges)
    B --> C[tests/bin/golden.rs Runner]
    C -->|Normal Mode| D[Generate *_gen.png images]
    C -->|Override Mode| E[Overwrite *_main.png reference goldens]
    D --> F[Compare *_gen.png vs *_main.png via ImageMagick]
    F -->|Visual Mismatch| G[Test Fail / Upload Artifacts]
    F -->|Match| H[Test Pass]
```

### 1. Deterministic Rendering

Skeuomorphic components (like WebGL paper grains and procedural torn paper edges) are generated using randomized noise. To make pixel-by-pixel image comparison possible, a testing helper script overrides `Math.random()` with a seeded pseudo-random number generator (PRNG) when the query parameter `?deterministic=true` is present in the URL.

This locks down the visual noise patterns so they are identical across test runs.

### 2. Golden Image Runner

A headless Chrome browser driver, located in [tests/bin/golden.rs](file:///Users/gilnobrega/git/carbon/tests/bin/golden.rs), is used to automatically load the pages and capture individual elements. It captures:
- Key global layouts (`header`, `#sidebar`, `footer`, `.article-card-wrapper`).
- Granular elements from the welcome article (`blockquote`, `table`, `pre` codeblocks, `.tipped-image-container` wrapper, `ul` lists, `hr` separators, and headings `h1` through `h6`).

---

## Running Tests Locally

### Prerequisite

Make sure the local development server is running in the background:

```bash
cargo run
```

### 1. Generate & Run Comparison Images

Run the golden test runner to generate comparison files ending in `_gen.png` under `tests/golden/`:

```bash
cargo run --bin golden
```

### 2. Overwrite Golden Reference Images

If you have intentionally modified styling or layout elements and want to update the master reference images, use the `--override` flag to overwrite the `_main.png` files directly:

```bash
cargo run --bin golden -- --override
```

---

## Continuous Integration (CI)

Our GitHub Actions workflow ([golden-tests.yml](file:///Users/gilnobrega/git/carbon/.github/workflows/golden-tests.yml)) automatically runs the golden tests on every push and pull request.

1. Spawns the Corkboard server in the background.
2. Compiles and executes the `golden` runner to produce `_gen.png` images.
3. Compares all `_gen.png` against their `_main.png` counterparts using **ImageMagick**:
   ```bash
   compare -metric AE -fuzz 1% <main_file> <gen_file> <diff_file>
   ```
4. **Tolerance**: The `1%` fuzz factor allows the suite to ignore subtle sub-pixel variations (such as minor rendering/image loading differences) while still catching actual layout regressions.
5. **Artifacts**: If tests fail, the visual diff files highlight mismatches in red and are uploaded as pipeline artifacts.
