import { test, expect, seedAuth, waitForHydrated } from "./fixtures";
import type { Page } from "@playwright/test";

async function completeVisibleTour(page: Page) {
  const footer = page.locator('[data-testid="spotlight-footer"]:visible');
  const next = footer.getByTestId("spotlight-tour-next");
  await expect(footer).toBeVisible({ timeout: 60_000 });
  for (let i = 0; i < 24; i++) {
    if ((await footer.count()) === 0) {
      break;
    }
    // Spotlight panels can sit partially off-screen; DOM click avoids Playwright
    // viewport hit-testing failures that still occur with { force: true }.
    await next.evaluate((el: HTMLElement) => el.click());
    try {
      await expect(footer).toHaveCount(0, { timeout: 2_000 });
      break;
    } catch {
      /* more steps */
    }
  }
  await expect(footer).toHaveCount(0, { timeout: 30_000 });
}

test.describe("help-spotlight", () => {
  test("help-spotlight-skips-when-seeded", async ({ page }) => {
    await seedAuth(page, "admin");
    await page.goto("/chronon", { waitUntil: "domcontentloaded" });
    await waitForHydrated(page);
    await expect(page.getByTestId("chronon-dashboard")).toBeVisible({ timeout: 60_000 });
    await expect(page.getByTestId("help-step-chronon-intro")).toHaveCount(0);
    await expect(page.locator('[data-testid="spotlight-footer"]:visible')).toHaveCount(0);
  });

  test("help-spotlight-skips-auth-gate", async ({ page }) => {
    await seedAuth(page, "anonymous", { help_tour: true });
    await page.goto("/chronon", { waitUntil: "domcontentloaded" });
    await waitForHydrated(page);
    await expect(page.getByTestId("auth-required-empty-state")).toBeAttached({
      timeout: 60_000,
    });
    await expect(page.getByTestId("help-step-chronon-intro")).toHaveCount(0);
    await expect(page.locator('[data-testid="spotlight-footer"]:visible')).toHaveCount(0);
  });

  test("help-spotlight-dashboard-green", async ({ page }) => {
    await seedAuth(page, "admin", { help_tour: true });
    await page.goto("/chronon", { waitUntil: "domcontentloaded" });
    await waitForHydrated(page);
    await expect(page.getByTestId("help-step-chronon-intro")).toBeVisible({ timeout: 60_000 });
    await completeVisibleTour(page);
    await expect(page.getByTestId("help-step-chronon-intro")).toHaveCount(0);

    await page.reload({ waitUntil: "domcontentloaded" });
    await waitForHydrated(page);
    await expect(page.getByTestId("help-step-chronon-intro")).toHaveCount(0);
  });

  test("help-spotlight-jobs-green", async ({ page }) => {
    await seedAuth(page, "admin", { help_tour: true });
    await page.goto("/chronon/jobs", { waitUntil: "domcontentloaded" });
    await waitForHydrated(page);
    await expect(page.getByTestId("help-step-chronon-jobs-intro")).toBeVisible({
      timeout: 60_000,
    });
    await completeVisibleTour(page);
  });

  test("help-spotlight-job-create-green", async ({ page }) => {
    await seedAuth(page, "admin", { help_tour: true });
    await page.goto("/chronon/jobs/new", { waitUntil: "domcontentloaded" });
    await waitForHydrated(page);
    await expect(page.getByTestId("help-step-chronon-create-intro")).toBeVisible({
      timeout: 60_000,
    });
    await completeVisibleTour(page);
  });

  test("help-spotlight-job-detail-green", async ({ page }) => {
    const seeded = await seedAuth(page, "admin", { help_tour: true });
    const jobId = seeded.fixtures.job_id;
    await page.goto(`/chronon/jobs/${encodeURIComponent(jobId)}`, {
      waitUntil: "domcontentloaded",
    });
    await waitForHydrated(page);
    await expect(page.getByTestId("help-step-chronon-job-header")).toBeVisible({
      timeout: 60_000,
    });
    await completeVisibleTour(page);
  });

  test("help-spotlight-runs-green", async ({ page }) => {
    await seedAuth(page, "admin", { help_tour: true });
    await page.goto("/chronon/runs", { waitUntil: "domcontentloaded" });
    await waitForHydrated(page);
    await expect(page.getByTestId("help-step-chronon-runs-intro")).toBeVisible({
      timeout: 60_000,
    });
    await completeVisibleTour(page);
  });

  test("help-spotlight-run-detail-green", async ({ page }) => {
    const seeded = await seedAuth(page, "admin", { help_tour: true });
    const runId = seeded.fixtures.run_id;
    await page.goto(`/chronon/runs/${encodeURIComponent(runId)}`, {
      waitUntil: "domcontentloaded",
    });
    await waitForHydrated(page);
    await expect(page.getByTestId("help-step-chronon-run-header")).toBeVisible({
      timeout: 60_000,
    });
    await completeVisibleTour(page);
  });

  test("help-spotlight-scripts-green", async ({ page }) => {
    await seedAuth(page, "admin", { help_tour: true });
    await page.goto("/chronon/scripts", { waitUntil: "domcontentloaded" });
    await waitForHydrated(page);
    await expect(page.getByTestId("help-step-chronon-scripts-intro")).toBeVisible({
      timeout: 60_000,
    });
    await completeVisibleTour(page);
  });
});
