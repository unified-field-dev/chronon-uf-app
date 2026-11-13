import { test, expect, seedAuth, waitForHydrated } from "./fixtures";
import type { Locator, Page } from "@playwright/test";

// The host seeds 45 history runs cycling failed / timeout / success, so the UI
// "failed" status (failed + timeout) matches 30 runs: a full page of 20, then 10.
// Cursor pages report no total, so the footer reads "of more than N" until the end.

async function openRunsTable(page: Page): Promise<Locator> {
  await seedAuth(page, "admin");
  await page.goto("/chronon/runs", { waitUntil: "domcontentloaded" });
  await waitForHydrated(page);
  const table = page.getByTestId("chronon-runs-data-table");
  await expect(table).toBeVisible({ timeout: 60_000 });
  await expect(table.locator("tbody tr")).toHaveCount(20, { timeout: 60_000 });
  return table;
}

async function selectOption(wrapper: Locator, value: string) {
  const select = wrapper.locator("select");
  await expect(async () => {
    await select.selectOption(value);
    await expect(select).toHaveValue(value);
  }).toPass({ timeout: 5_000 });
}

async function filterFailed(page: Page, table: Locator) {
  await table.getByTestId("data-table-filter-panel-trigger").click();
  const panel = page.getByTestId("data-table-filter-panel");
  await selectOption(panel.getByTestId("data-table-filter-rule-0-field"), "status");
  await selectOption(panel.getByTestId("data-table-filter-rule-0-operator"), "equals");
  await panel.getByTestId("data-table-filter-rule-0-value").locator("input").fill("failed");
  await panel.getByRole("button", { name: "Apply" }).click();
}

function pager(table: Locator) {
  const bar = table.getByTestId("data-table-pagination");
  return {
    next: bar.getByRole("button", { name: "Next page" }),
    previous: bar.getByRole("button", { name: "Previous page" }),
  };
}

async function rowIds(table: Locator): Promise<string[]> {
  return table
    .locator("tbody tr")
    .evaluateAll((rows) => rows.map((row) => row.getAttribute("data-testid") ?? ""));
}

for (const scheme of ["light", "dark"] as const) {
  test.describe(`pw-chronon-runs-paging (${scheme})`, () => {
    test.beforeEach(async ({ page }) => {
      await page.emulateMedia({ colorScheme: scheme });
    });

    test("pw-chronon-runs-paging-happy-status-filter-next-previous", async ({ page }) => {
      const table = await openRunsTable(page);
      const footer = table.getByTestId("data-table-footer");
      const rows = table.locator("tbody tr");
      const { next, previous } = pager(table);

      await filterFailed(page, table);
      await expect(footer).toContainText("1–20 of more than 20", { timeout: 60_000 });
      await expect(rows).toHaveCount(20);
      await expect(rows.filter({ hasNotText: "Failed" })).toHaveCount(0);
      const firstPage = await rowIds(table);

      await next.click();
      await expect(footer).toContainText("21–30 of 30", { timeout: 60_000 });
      await expect(rows).toHaveCount(10);
      await expect(rows.filter({ hasNotText: "Failed" })).toHaveCount(0);
      const secondPage = await rowIds(table);
      expect(new Set([...firstPage, ...secondPage]).size).toBe(30);
      await expect(next).toBeDisabled();

      await previous.click();
      await expect(footer).toContainText("1–20 of", { timeout: 60_000 });
      await expect(rows).toHaveCount(20);
      expect(await rowIds(table)).toEqual(firstPage);
    });
  });
}

// Phone width pages the unfiltered table: the filter panel's popover opens partly
// off-screen there (see Orbital's own cursor spec).
test.describe("pw-chronon-runs-paging (phone)", () => {
  test.use({ viewport: { width: 390, height: 844 }, isMobile: true, hasTouch: true });

  test("pw-chronon-runs-paging-happy-phone-next-previous", async ({ page }) => {
    const table = await openRunsTable(page);
    const footer = table.getByTestId("data-table-footer");
    const { next, previous } = pager(table);
    await expect(footer).toContainText("1–20 of more than 20", { timeout: 60_000 });
    const firstPage = await rowIds(table);

    await next.click();
    await expect(footer).toContainText("21–40 of more than 40", { timeout: 60_000 });
    const secondPage = await rowIds(table);
    expect(new Set([...firstPage, ...secondPage]).size).toBe(40);

    await previous.click();
    await expect(footer).toContainText("1–20 of", { timeout: 60_000 });
    expect(await rowIds(table)).toEqual(firstPage);
  });
});
