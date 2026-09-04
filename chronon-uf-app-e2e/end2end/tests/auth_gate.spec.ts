import { test, expect, seedAuth, waitForHydrated } from "./fixtures";

test.describe("pw-chronon-auth-gate", () => {
  test("pw-chronon-auth-gate-sad-anonymous", async ({ page }) => {
    await seedAuth(page, "anonymous");
    await page.goto("/chronon", { waitUntil: "domcontentloaded" });
    await waitForHydrated(page);
    await expect(page.getByTestId("auth-required-empty-state")).toBeAttached({
      timeout: 60_000,
    });
    await expect(page.getByTestId("chronon-dashboard")).toHaveCount(0);
  });

  test("pw-chronon-auth-gate-happy-admin", async ({ page }) => {
    await seedAuth(page, "admin");
    await page.goto("/chronon", { waitUntil: "domcontentloaded" });
    await waitForHydrated(page);
    await expect(page.getByTestId("chronon-app-root")).toBeVisible({ timeout: 60_000 });
    await expect(page.getByTestId("chronon-dashboard")).toBeVisible({ timeout: 60_000 });
  });
});
