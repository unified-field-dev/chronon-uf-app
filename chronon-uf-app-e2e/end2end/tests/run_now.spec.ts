import { test, expect, seedAuth, waitForHydrated, expectMutationDenied } from "./fixtures";

test.describe("pw-chronon-run-now", () => {
  test("pw-chronon-run-now-happy-admin", async ({ page }) => {
    const seeded = await seedAuth(page, "admin");
    await page.goto(`/chronon/jobs/${encodeURIComponent(seeded.fixtures.job_id)}`, {
      waitUntil: "domcontentloaded",
    });
    await waitForHydrated(page);
    await expect(page.getByTestId("chronon-job-detail")).toBeVisible({ timeout: 60_000 });
    await expect(page.getByTestId("run-now-button")).toBeVisible({ timeout: 60_000 });
    // Lab script has no params, so Run Now enqueues immediately (no dialog).
    await page.getByTestId("run-now-button").getByRole("button").click();
    await expect(page.locator(".orbital-message-bar--error")).toHaveCount(0, {
      timeout: 30_000,
    });
    // Button leaves the transient "Running..." label after enqueue completes.
    await expect(page.getByTestId("run-now-button").getByRole("button")).toContainText("Run Now", {
      timeout: 60_000,
    });
  });

  test("pw-chronon-run-now-sad-non-admin", async ({ page }) => {
    const seeded = await seedAuth(page, "outsider");
    await page.goto(`/chronon/jobs/${encodeURIComponent(seeded.fixtures.job_id)}`, {
      waitUntil: "domcontentloaded",
    });
    await waitForHydrated(page);
    await expect(page.getByTestId("chronon-job-detail")).toBeVisible({ timeout: 60_000 });
    // Outsiders may still open detail (session reads); run-now requires ChrononAdmin.
    const runNow = page.getByTestId("run-now-button");
    if ((await runNow.count()) > 0) {
      await runNow.getByRole("button").click();
      await expectMutationDenied(page);
    } else {
      // Permission-gated chrome hidden is also a valid deny containment.
      await expect(runNow).toHaveCount(0);
    }
  });
});
