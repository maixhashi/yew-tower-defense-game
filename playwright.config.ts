import { defineConfig, devices } from "@playwright/test";

/**
 * App must already be running (e.g. `docker compose up`).
 * Do not start Trunk via webServer — avoids a second Wasm build.
 */
export default defineConfig({
  testDir: "e2e",
  timeout: 180_000,
  fullyParallel: false,
  forbidOnly: !!process.env.CI,
  retries: 1,
  workers: 1,
  reporter: process.env.CI ? [["list"], ["html", { open: "never" }]] : "list",
  use: {
    baseURL: "http://127.0.0.1:8080",
    viewport: { width: 1280, height: 720 },
    trace: "on-first-retry",
  },
  // System Chrome: bundled Chromium install is unavailable on older macOS (e.g. mac13).
  projects: [
    {
      name: "chromium",
      use: { ...devices["Desktop Chrome"], channel: "chrome" },
    },
  ],
});

