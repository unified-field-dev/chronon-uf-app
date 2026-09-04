import { test, expect, seedAuth, waitForHydrated } from "./fixtures";

test.describe("pw-chronon-runs", () => {
  test("pw-chronon-runs-happy-list-detail", async ({ page }) => {
    const seeded = await seedAuth(page, "admin");
    await page.goto("/chronon/runs", { waitUntil: "domcontentloaded" });
    await waitForHydrated(page);
    await expect(page.getByTestId("chronon-runs-page")).toBeVisible({ timeout: 60_000 });
    await expect(page.getByTestId("chronon-runs-data-table")).toBeVisible({ timeout: 60_000 });
    await page.goto(`/chronon/runs/${encodeURIComponent(seeded.fixtures.run_id)}`, {
      waitUntil: "domcontentloaded",
    });
    await waitForHydrated(page);
    await expect(page.getByTestId("chronon-run-detail")).toBeVisible({ timeout: 60_000 });
  });

  test("pw-chronon-runs-sad-unknown-run", async ({ page }) => {
    await seedAuth(page, "admin");
    await page.goto("/chronon/runs/__chronon_e2e_no_such_run__", {
      waitUntil: "domcontentloaded",
    });
    await waitForHydrated(page);
    await expect(page.getByTestId("chronon-run-detail")).toBeVisible({ timeout: 60_000 });
    await expect(page.getByText("Run not found.")).toBeVisible({ timeout: 60_000 });
  });
});
