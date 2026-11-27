import { readFileSync } from "node:fs";
import { test, expect } from "@playwright/test";
import AxeBuilder from "@axe-core/playwright";
import { installMockWorker } from "./mock-worker.mjs";

const schema = JSON.parse(readFileSync(new URL("../../contract/settings.json", import.meta.url), "utf8"));

test.beforeEach(async ({ page }) => {
  await page.addInitScript(installMockWorker, schema);
  await page.goto("/");
  await expect(page.getByText("Worker: Connected")).toBeVisible();
  await expect(page.getByRole("button", { name: "Start Stream" })).toBeEnabled();
});

test("settings, discovery, stream, pin, compare and stop", async ({ page }) => {
  await page.getByText("Preview tuning", { exact: true }).click();
  await page.getByLabel("Max FPS").selectOption("30");
  await page.getByRole("button", { name: "Discover" }).click();
  await expect(page.getByLabel("KMS output").getByRole("option", { name: "HDMI-A-1" })).toBeAttached();
  await page.getByLabel("KMS output").selectOption("HDMI-A-1");
  await page.getByLabel("Bitrate").first().selectOption("30M");
  await page.getByRole("button", { name: "Start Stream" }).click();
  await expect(page.getByRole("button", { name: "Stop Stream" })).toBeVisible();
  await page.getByRole("button", { name: "Statistics", exact: true }).click();
  await expect(page.getByText("80.0 fps", { exact: true })).toBeVisible();
  await page.getByRole("button", { name: "Pin Current Run" }).click();
  await page.getByRole("button", { name: "Compare", exact: true }).click();
  await expect(page.getByLabel("Run Name")).toHaveValue("Run 1");
  await expect(page.getByRole("row", { name: /^Sent FPS/ })).toContainText("70.0 fps");
  await expect(page.getByRole("row", { name: /^Dropped Frames/ })).toContainText("2.00");
  await page.getByLabel("Run Name").fill("Baseline");
  await page.getByLabel("Run Name").press("Tab");
  await page.getByRole("button", { name: "Stop Stream" }).click();
  await expect(page.getByRole("button", { name: "Start Stream" })).toBeVisible();
  await page.reload();
  await expect(page.getByText("Worker: Connected")).toBeVisible();
  await page.getByText("Preview tuning", { exact: true }).click();
  await expect(page.getByLabel("Max FPS")).toHaveValue("30");
  await page.getByRole("button", { name: "Compare", exact: true }).click();
  await expect(page.getByLabel("Run Name")).toHaveValue("Baseline");
  await page.getByRole("button", { name: "Delete Selected" }).click();
  await expect(page.getByText("No runs pinned.", { exact: false })).toBeVisible();
  await expect(page.getByRole("button", { name: "Start Stream" })).toBeVisible();
});

test("rejected settings roll back", async ({ page }) => {
  await page.evaluate(() => window.mockWorker.failNextSetting());
  const bitrate = page.getByLabel("Bitrate").first();
  await bitrate.selectOption("40M");
  await expect(page.getByRole("alert")).toContainText("Mock setting rejected");
  await expect(bitrate).toHaveValue("20M");
});

test("GUI and pane opacity apply independently without a restart", async ({ page }) => {
  await page.getByRole("button", { name: "Start Stream" }).click();
  const transparency = page
    .locator("article")
    .filter({ has: page.getByRole("heading", { name: "Transparency", exact: true }) });
  const pip = page
    .locator("article")
    .filter({ has: page.getByRole("heading", { name: "Picture-in-Picture", exact: true }) });
  await expect(page.getByLabel("Enable Transparency")).not.toBeChecked();
  await expect(page.getByLabel("Enable Picture-in-Picture")).not.toBeChecked();
  const opacity = page.getByRole("slider", { name: "DJI GUI opacity (%)" });
  const paneOpacity = page.getByRole("slider", { name: "GFX1 pane opacity (%)" });
  await expect(transparency.getByText("Adjust DJI GUI opacity at runtime.")).toBeVisible();
  await expect(transparency.getByText("Adjust GFX1 pane opacity at runtime.")).toBeVisible();
  await expect(transparency.getByRole("slider", { name: "DJI GUI opacity (%)" })).toBeVisible();
  await expect(transparency.getByRole("slider", { name: "GFX1 pane opacity (%)" })).toBeVisible();
  await expect(pip.getByRole("slider", { name: "GFX1 pane opacity (%)" })).toHaveCount(0);
  await expect(opacity).toHaveValue("100");
  await opacity.fill("50");
  await expect(opacity).toHaveValue("50");
  await expect(paneOpacity).toHaveValue("100");
  await paneOpacity.fill("25");
  await expect(paneOpacity).toHaveValue("25");
  await expect(opacity).toHaveValue("50");
  await paneOpacity.fill("0");
  await expect(paneOpacity).toHaveValue("0");
  await expect(page.getByRole("button", { name: "Restart", exact: true })).toHaveCount(0);
  await page.getByLabel("Enable Transparency").check();
  await page.getByLabel("Enable Picture-in-Picture").check();
  await expect(opacity).toHaveValue("50");
  await expect(paneOpacity).toHaveValue("0");
  await page.getByRole("button", { name: "Settings", exact: true }).click();
  await expect(page.getByText("Adjust DJI GUI opacity at runtime.")).toBeVisible();
  await expect(page.getByText("Adjust GFX1 pane opacity at runtime.")).toBeVisible();
  await expect(opacity).toHaveValue("50");
  await expect(paneOpacity).toHaveValue("0");
});

test("available opacity controls remain visible without the transparency setting", async ({ page }) => {
  const limitedSchema = {
    ...schema,
    settings: schema.settings
      .filter((setting) => setting.name !== "alpha")
      .map((setting) => (setting.name === "pane_opacity" ? { ...setting, visible: false } : setting)),
  };
  await page.addInitScript(installMockWorker, limitedSchema);
  await page.reload();
  await expect(page.getByText("Worker: Connected")).toBeVisible();
  const transparency = page
    .locator("article")
    .filter({ has: page.getByRole("heading", { name: "Transparency", exact: true }) });
  await expect(transparency.getByRole("slider", { name: "DJI GUI opacity (%)" })).toBeVisible();
  await expect(page.getByRole("slider", { name: "GFX1 pane opacity (%)" })).toHaveCount(0);
  await expect(page.getByLabel("Enable Transparency")).toHaveCount(0);
});

for (const width of [375, 1024, 1440]) {
  test(`transparency sliders retain usable width at ${width}px`, async ({ page }) => {
    await page.setViewportSize({ width, height: 900 });
    await page.getByLabel("Enable Transparency").check();
    for (const name of ["GFX1 pane opacity (%)", "DJI GUI opacity (%)", "Key Color tolerance", "Key Color softness"]) {
      const slider = page.getByRole("slider", { name });
      await expect(slider).toBeVisible();
      const sizes = await slider.evaluate((input) => ({
        slider: input.getBoundingClientRect().width,
        label: input.closest("label").getBoundingClientRect().width,
      }));
      expect(sizes.slider, name).toBeGreaterThanOrEqual(150);
      expect(sizes.slider / sizes.label, name).toBeGreaterThanOrEqual(0.7);
    }
    expect(await page.evaluate(() => document.documentElement.scrollWidth <= window.innerWidth)).toBe(true);
  });
}

test("range controls use their full track in Overview and Settings", async ({ page }) => {
  await page.getByLabel("Enable Transparency").check();
  for (const name of ["GFX1 pane opacity (%)", "DJI GUI opacity (%)", "Key Color tolerance", "Key Color softness"]) {
    const slider = page.getByRole("slider", { name });
    const style = await slider.evaluate((input) => {
      const style = getComputedStyle(input);
      return { padding: style.padding, border: style.borderWidth, background: style.backgroundColor };
    });
    expect(style).toEqual({ padding: "0px", border: "0px", background: "rgba(0, 0, 0, 0)" });
    const bounds = await slider.boundingBox();
    await slider.click({ position: { x: 1, y: bounds.height / 2 } });
    await expect(slider).toHaveValue("0");
    await slider.click({ position: { x: bounds.width - 1, y: bounds.height / 2 } });
    await expect(slider).toHaveValue(name.startsWith("Key Color") ? "1" : "100");
  }
  await page.getByRole("button", { name: "Settings", exact: true }).click();
  const slider = page.getByRole("slider", { name: "DJI GUI opacity (%)" });
  const style = await slider.evaluate((input) => {
    const style = getComputedStyle(input);
    return { padding: style.padding, border: style.borderWidth, background: style.backgroundColor };
  });
  expect(style).toEqual({ padding: "0px", border: "0px", background: "rgba(0, 0, 0, 0)" });
  const bounds = await slider.boundingBox();
  await slider.click({ position: { x: 1, y: bounds.height / 2 } });
  await expect(slider).toHaveValue("0");
  await slider.click({ position: { x: bounds.width - 1, y: bounds.height / 2 } });
  await expect(slider).toHaveValue("100");
});

test("styled checkboxes remain keyboard accessible", async ({ page }) => {
  const checkbox = page.getByLabel("Enable Transparency");
  await checkbox.focus();
  await page.keyboard.press("Space");
  await expect(checkbox).toBeChecked();
  const style = await checkbox.evaluate((input) => {
    const style = getComputedStyle(input);
    return { appearance: style.appearance, image: style.backgroundImage, outline: style.outlineStyle };
  });
  expect(style.appearance).toBe("none");
  expect(style.image).toContain("data:image/svg+xml");
  expect(style.outline).not.toBe("none");
  await page.keyboard.press("Space");
  await expect(checkbox).not.toBeChecked();
});

test("picture-in-picture resizes from every side and corner", async ({ page }) => {
  const enabled = page.getByLabel("Enable Picture-in-Picture");
  const cases = [
    ["top", [960, 500, 480, 310]],
    ["bottom", [960, 580, 480, 270]],
    ["left", [920, 540, 520, 270]],
    ["right", [1000, 540, 480, 270]],
    ["top left", [920, 500, 520, 310]],
    ["top right", [1000, 500, 480, 310]],
    ["bottom left", [920, 580, 520, 270]],
    ["bottom right", [1000, 580, 480, 270]],
  ];
  for (const [direction, expected] of cases) {
    await enabled.check();
    const handle = page.getByRole("button", {
      name: `Resize picture-in-picture pane from ${direction} (use size and position fields for keyboard input)`,
      exact: true,
    });
    await handle.scrollIntoViewIfNeeded();
    const handleBounds = await handle.boundingBox();
    const panelBounds = await page.getByLabel("1920 by 1080 panel").boundingBox();
    await page.mouse.move(handleBounds.x + handleBounds.width / 2, handleBounds.y + handleBounds.height / 2);
    await page.mouse.down();
    await page.mouse.move(
      handleBounds.x + handleBounds.width / 2 + (panelBounds.width * 40) / 1920,
      handleBounds.y + handleBounds.height / 2 + (panelBounds.height * 40) / 1080,
    );
    await page.mouse.up();
    for (const [index, label] of ["Width", "Height", "X", "Y"].entries()) {
      await expect(page.getByLabel(`${label} (px)`)).toHaveValue(String(expected[index]));
    }
    await enabled.uncheck();
  }
});

test("picture-in-picture moves are limited to 25 Hz and retain the final position", async ({ page }) => {
  await page.getByLabel("Enable Picture-in-Picture").check();
  const pane = page.getByRole("button", {
    name: "Move picture-in-picture pane (use position fields for keyboard input)",
  });
  await pane.scrollIntoViewIfNeeded();
  await page.clock.install();
  await page.clock.pauseAt(await page.evaluate(() => Date.now() + 1000));
  await page.evaluate(() => {
    window.mockWorker.paneUpdates.length = 0;
  });

  const bounds = await pane.boundingBox();
  const x = bounds.x + bounds.width / 2;
  const y = bounds.y + bounds.height / 2;
  await page.mouse.move(x, y);
  await page.mouse.down();
  await page.mouse.move(x + 10, y);
  await expect.poll(() => page.evaluate(() => window.mockWorker.paneUpdates.length)).toBe(1);
  await page.mouse.move(x + 20, y);
  await page.mouse.move(x + 30, y);
  expect(await page.evaluate(() => window.mockWorker.paneUpdates.length)).toBe(1);
  await page.clock.fastForward(39);
  expect(await page.evaluate(() => window.mockWorker.paneUpdates.length)).toBe(1);
  await page.clock.fastForward(1);
  await expect.poll(() => page.evaluate(() => window.mockWorker.paneUpdates.length)).toBe(2);
  await page.mouse.move(x + 40, y);
  const finalX = await page.getByLabel("X (px)").inputValue();
  await page.mouse.up();
  await page.getByRole("button", { name: "Settings", exact: true }).click();
  await page.clock.fastForward(40);
  const updates = await page.evaluate(() => window.mockWorker.paneUpdates);
  expect(updates).toHaveLength(3);
  expect(updates[2].value).toBe(`960x540+${finalX}+270`);
  expect(updates[1].at - updates[0].at).toBeGreaterThanOrEqual(40);
  expect(updates[2].at - updates[1].at).toBeGreaterThanOrEqual(40);
});

test("picture-in-picture resizing is limited to 15 Hz and cancellation restores geometry", async ({ page }) => {
  await page.getByLabel("Enable Picture-in-Picture").check();
  const handle = page.getByRole("button", {
    name: "Resize picture-in-picture pane from right (use size and position fields for keyboard input)",
  });
  await handle.scrollIntoViewIfNeeded();
  await page.clock.install();
  await page.clock.pauseAt(await page.evaluate(() => Date.now() + 1000));
  await page.evaluate(() => {
    window.mockWorker.paneUpdates.length = 0;
  });
  const bounds = await handle.boundingBox();
  const x = bounds.x + bounds.width / 2;
  const y = bounds.y + bounds.height / 2;
  await page.mouse.move(x, y);
  await page.mouse.down();
  await page.mouse.move(x + 10, y);
  await expect.poll(() => page.evaluate(() => window.mockWorker.paneUpdates.length)).toBe(1);
  await page.mouse.move(x + 20, y);
  await page.clock.fastForward(66);
  expect(await page.evaluate(() => window.mockWorker.paneUpdates.length)).toBe(1);
  await page.clock.fastForward(1);
  await expect.poll(() => page.evaluate(() => window.mockWorker.paneUpdates.length)).toBe(2);
  await page.mouse.move(x + 30, y);
  await handle.dispatchEvent("pointercancel", { pointerId: 1 });
  await page.mouse.up();
  await page.clock.fastForward(67);
  const updates = await page.evaluate(() => window.mockWorker.paneUpdates);
  expect(updates).toHaveLength(3);
  expect(updates[0].value).not.toBe("960x540+480+270");
  expect(updates[1].value).not.toBe(updates[0].value);
  expect(updates[2].value).toBe("960x540+480+270");
  expect(updates[1].at - updates[0].at).toBeGreaterThanOrEqual(1000 / 15);
  expect(updates[2].at - updates[1].at).toBeGreaterThanOrEqual(1000 / 15);
});

test("disabling picture-in-picture cancels queued drag updates", async ({ page }) => {
  const enabled = page.getByLabel("Enable Picture-in-Picture");
  await enabled.check();
  const pane = page.getByRole("button", {
    name: "Move picture-in-picture pane (use position fields for keyboard input)",
  });
  await pane.scrollIntoViewIfNeeded();
  await page.clock.install();
  await page.evaluate(() => {
    window.mockWorker.paneUpdates.length = 0;
  });
  const bounds = await pane.boundingBox();
  const x = bounds.x + bounds.width / 2;
  const y = bounds.y + bounds.height / 2;
  await page.mouse.move(x, y);
  await page.mouse.down();
  await page.mouse.move(x + 10, y);
  await page.mouse.move(x + 20, y);
  await enabled.evaluate((input) => input.click());
  await page.mouse.up();
  await page.clock.fastForward(200);
  const updates = await page.evaluate(() => window.mockWorker.paneUpdates);
  expect(updates).toHaveLength(2);
  expect(updates[1].value).toBeNull();
});

test("PiP and transparency controls stay gated and synchronized across tabs", async ({ page }) => {
  const pip = page.getByLabel("Enable Picture-in-Picture");
  const transparency = page.getByLabel("Enable Transparency");
  const alphaSource = page.getByLabel("Alpha Source");
  const width = page.getByLabel("Width (px)");
  const panel = page.getByLabel("1920 by 1080 panel");
  const colorKey = page.getByLabel("Key Color RRGGBB");

  await expect(width).toHaveCount(0);
  await expect(panel).toHaveCount(0);
  await expect(alphaSource).toHaveCount(0);
  await pip.check();
  await transparency.check();
  await expect(panel).toBeVisible();
  await expect(alphaSource).toHaveValue("color_key");
  await expect(alphaSource.locator("option:checked")).toHaveText("Key Color");
  await expect(colorKey).toHaveValue("00ff00");
  await expect(colorKey).toBeVisible();
  await colorKey.fill("123456");
  await expect
    .poll(() =>
      page.evaluate(() =>
        window.mockWorker.propertyUpdates.some(
          (property) => property.name === "color_key" && property.value.text === "123456",
        ),
      ),
    )
    .toBe(true);

  await page.getByRole("button", { name: "Settings", exact: true }).click();
  await expect(pip).toBeChecked();
  await expect(transparency).toBeChecked();
  await expect(width).toHaveValue("960");
  await expect(panel).toHaveCount(0);
  await expect(alphaSource).toHaveValue("color_key");
  await expect(colorKey).toHaveCount(0);
  await width.fill("800");
  await width.press("Tab");
  await page.getByRole("button", { name: "Overview", exact: true }).click();
  await expect(width).toHaveValue("800");
  await expect(colorKey).toHaveValue("123456");
  await page.getByRole("button", { name: "Settings", exact: true }).click();
  await pip.uncheck();
  await transparency.uncheck();
  await expect(width).toHaveCount(0);
  await expect(alphaSource).toHaveCount(0);
  await transparency.check();
  await expect(alphaSource).toHaveValue("color_key");
  await alphaSource.selectOption("source");
  await expect(alphaSource).toHaveValue("source");
  await transparency.uncheck();

  await page.getByRole("button", { name: "Overview", exact: true }).click();
  await expect(pip).not.toBeChecked();
  await expect(transparency).not.toBeChecked();
  await expect(panel).toHaveCount(0);
  await expect(alphaSource).toHaveCount(0);
  await expect(colorKey).toHaveCount(0);
});

test("Key Color does not change stream preview subscriptions", async ({ page }) => {
  await page.evaluate(() => {
    window.VideoDecoder = class {
      static async isConfigSupported() {
        return { supported: true };
      }
    };
    window.EncodedVideoChunk = class {};
    window.mockWorker.previewUpdates.length = 0;
  });
  await page.getByLabel("Enable Transparency").check();
  await expect(page.getByLabel("Key Color RRGGBB")).toBeVisible();
  await expect(page.getByText("Key Color Preview")).toHaveCount(0);
  expect(await page.evaluate(() => window.mockWorker.previewUpdates)).toEqual([]);
  await page.getByText("Preview tuning", { exact: true }).click();
  await page.getByLabel("Keyframes only").check();
  await page.getByRole("button", { name: "Start Preview" }).click();
  await expect
    .poll(() => page.evaluate(() => window.mockWorker.previewUpdates.at(-1)))
    .toEqual({ enabled: true, keyframesOnly: true });
  const count = await page.evaluate(() => window.mockWorker.previewUpdates.length);
  await page.getByRole("button", { name: "Settings", exact: true }).click();
  await page.getByLabel("Enable Transparency").uncheck();
  await page.getByRole("button", { name: "Overview", exact: true }).click();
  await page.getByLabel("Enable Transparency").check();
  expect(await page.evaluate(() => window.mockWorker.previewUpdates.length)).toBe(count);
  await page.getByRole("button", { name: "Pause Preview" }).click();
  await expect.poll(() => page.evaluate(() => window.mockWorker.previewUpdates.at(-1)?.enabled)).toBe(false);
});

test("generated run names stay unique after deleting a pin", async ({ page }) => {
  await page.getByRole("button", { name: "Start Stream" }).click();
  await page.getByRole("button", { name: "Statistics", exact: true }).click();
  const pinRun = page.getByRole("button", { name: "Pin Current Run" });
  await expect(pinRun).toBeEnabled();
  await pinRun.click();
  await pinRun.click();

  await page.getByRole("button", { name: "Compare", exact: true }).click();
  await page.getByRole("button", { name: "Select None" }).click();
  await page.getByLabel("Run 1").check();
  await page.getByRole("button", { name: "Delete Selected" }).click();
  await page.getByRole("button", { name: "Statistics", exact: true }).click();
  await pinRun.click();
  await page.getByRole("button", { name: "Compare", exact: true }).click();
  await page.getByRole("button", { name: "Select All" }).click();
  await expect(page.getByLabel("Run Name")).toHaveCount(2);
  expect(await page.getByLabel("Run Name").evaluateAll((inputs) => inputs.map((input) => input.value))).toEqual([
    "Run 2",
    "Run 3",
  ]);
});

test("head tracking control is disabled when setting is unavailable", async ({ page }) => {
  const schemaWithoutHeadTracking = {
    ...schema,
    settings: schema.settings.filter((setting) => setting.name !== "head_tracking"),
  };
  await page.addInitScript(installMockWorker, schemaWithoutHeadTracking);
  await page.reload();
  await expect(page.getByText("Worker: Connected")).toBeVisible();
  await expect(page.getByRole("button", { name: "Start Head Tracking" })).toBeDisabled();
});

for (const viewport of [
  { width: 1440, height: 900 },
  { width: 375, height: 812 },
]) {
  test(`layout, keyboard and accessibility at ${viewport.width}px`, async ({ page }) => {
    await page.setViewportSize(viewport);
    for (const tab of ["Overview", "Statistics", "Compare", "Settings", "Log"]) {
      await page.getByRole("button", { name: tab, exact: true }).focus();
      await page.keyboard.press("Enter");
      await expect(
        page.getByRole("heading", {
          name: {
            Overview: "Stream Preview",
            Statistics: "Stream Statistics",
            Compare: "Compare Runs",
            Settings: "Settings",
            Log: "Session Log",
          }[tab],
          exact: true,
        }),
      ).toBeVisible();
      expect(await page.evaluate(() => document.documentElement.scrollWidth <= window.innerWidth)).toBe(true);
      const results = await new AxeBuilder({ page }).withTags(["wcag2a", "wcag2aa", "wcag21a", "wcag21aa"]).analyze();
      expect(results.violations).toEqual([]);
    }
  });
}

test("the video file is chosen with eyepiece-pc's file dialog", async ({ page }) => {
  await page.locator(".capture-group select").first().selectOption("file");
  await page.getByRole("button", { name: "Browse…" }).click();
  await expect(page.getByRole("textbox", { name: "Video file", exact: true })).toHaveValue("/videos/clip.mkv");
  expect(await page.evaluate(() => window.mockWorker.fileDialogs)).toEqual(["mock"]);
  expect(await page.evaluate(() => window.mockWorker.propertyUpdates.some((update) => update.name === "input"))).toBe(true);
  await expect(page.getByRole("button", { name: /Keyframe/ })).toHaveCount(0);
});

test("reset asks first, then restores the defaults with undo", async ({ page }) => {
  const bitrate = page.getByLabel("Bitrate").first();
  await bitrate.selectOption("40M");
  await expect(bitrate).toHaveValue("40M");
  await page.getByRole("button", { name: "Reset Settings" }).click();
  const dialog = page.getByRole("dialog", { name: "Reset all settings?" });
  await expect(dialog).toBeVisible();
  await dialog.getByRole("button", { name: "Cancel" }).click();
  await expect(dialog).toBeHidden();
  await expect(bitrate).toHaveValue("40M");
  await page.getByRole("button", { name: "Reset Settings" }).click();
  await dialog.getByRole("button", { name: "Reset Settings" }).click();
  await expect(bitrate).toHaveValue("20M");
  await expect(page.getByText("Settings reset.")).toBeVisible();
  await page.getByRole("button", { name: "Settings", exact: true }).click();
  await page.getByRole("button", { name: "Reset to Defaults" }).click();
  await expect(dialog).toBeVisible();
});
