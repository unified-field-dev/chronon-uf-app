import { test, expect, seedAuth, waitForHydrated } from "./fixtures";

test.describe("pw-chronon-jobs", () => {
  test("pw-chronon-jobs-happy-list-detail", async ({ page }) => {
    const seeded = await seedAuth(page, "admin");
    await page.goto("/chronon/jobs", { waitUntil: "domcontentloaded" });
    await waitForHydrated(page);
    await expect(page.getByTestId("chronon-jobs-page")).toBeVisible({ timeout: 60_000 });
    await expect(page.getByTestId("chronon-jobs-data-table")).toBeVisible({ timeout: 60_000 });
    await expect(page.getByText(seeded.fixtures.job_name).first()).toBeVisible({
      timeout: 60_000,
    });
    await page.goto(`/chronon/jobs/${encodeURIComponent(seeded.fixtures.job_id)}`, {
      waitUntil: "domcontentloaded",
    });
    await waitForHydrated(page);
    await expect(page.getByTestId("chronon-job-detail")).toBeVisible({ timeout: 60_000 });
    await expect(page.getByText(seeded.fixtures.job_name).first()).toBeVisible();
  });

  test("pw-chronon-jobs-sad-unknown-job", async ({ page }) => {
    await seedAuth(page, "admin");
    await page.goto("/chronon/jobs/__chronon_e2e_no_such_job__", {
      waitUntil: "domcontentloaded",
    });
    await waitForHydrated(page);
    await expect(page.getByTestId("chronon-job-detail")).toBeVisible({ timeout: 60_000 });
    await expect(page.getByText("Job not found.")).toBeVisible({ timeout: 60_000 });
  });

  test("pw-chronon-jobs-sad-unverified-create", async ({ page }) => {
    await seedAuth(page, "unverified");
    await page.goto("/chronon/jobs/new", { waitUntil: "domcontentloaded" });
    await waitForHydrated(page);
    await expect(page.getByTestId("chronon-job-create-page")).toHaveCount(0);
    await expect(
      page.getByTestId("email-verification-required-empty-state"),
    ).toBeAttached({ timeout: 60_000 });
  });
});
