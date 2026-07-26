import { test, expect } from "@playwright/test";
import path from "node:path";
import fs from "node:fs";

test("when headquarters UI is open, a full-page smoke screenshot is saved", async ({
  page,
}) => {
  await page.goto("/");
  await expect(page.getByRole("heading", { name: "古城防衛戦" })).toBeVisible({
    timeout: 120_000,
  });

  const outDir = path.join("e2e", "artifacts");
  fs.mkdirSync(outDir, { recursive: true });
  const outPath = path.join(outDir, "hq-smoke.png");
  await page.screenshot({ path: outPath, fullPage: true });
  expect(fs.existsSync(outPath)).toBeTruthy();
});

test("when box castle canvas is present, a scene screenshot is saved", async ({
  page,
}) => {
  await page.goto("/");
  const canvas = page.getByTestId("game-canvas");
  await expect(canvas).toBeVisible({ timeout: 120_000 });
  await page.waitForTimeout(500);

  const outDir = path.join("e2e", "artifacts");
  fs.mkdirSync(outDir, { recursive: true });
  const outPath = path.join(outDir, "box-castle.png");
  await page.screenshot({ path: outPath, fullPage: true });
  expect(fs.existsSync(outPath)).toBeTruthy();
});
