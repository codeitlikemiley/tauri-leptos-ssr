import { test, expect } from "@playwright/test";

const BASE_URL = "http://localhost:3000";

// ---------------------------------------------------------------------------
// Tier 0 – SSR Smoke Tests (no JavaScript required)
// ---------------------------------------------------------------------------
test.describe("SSR", () => {
  test("page returns HTTP 200", async ({ request }) => {
    const response = await request.get(BASE_URL);
    expect(response.status()).toBe(200);
  });

  test("response body contains DOCTYPE", async ({ request }) => {
    const response = await request.get(BASE_URL);
    const html = await response.text();
    expect(html).toContain("<!DOCTYPE html>");
  });

  test("title is 'Welcome to Tauri Leptos SSR'", async ({ page }) => {
    await page.goto(BASE_URL);
    await expect(page).toHaveTitle("Welcome to Tauri Leptos SSR");
  });

  test("SSR renders heading, label, and button without JS", async ({
    browser,
  }) => {
    const context = await browser.newContext({ javaScriptEnabled: false });
    const page = await context.newPage();
    await page.goto(BASE_URL);

    // h1 heading
    await expect(page.locator("h1")).toContainText("spin-counter");

    // "Count Value" label
    await expect(page.getByText("Count Value")).toBeVisible();

    // Increment button (text may read "Increment Counter" or "Updating...")
    await expect(
      page.getByRole("button", { name: /Increment Counter|Updating/ }),
    ).toBeVisible();

    // Subtitle
    await expect(page.getByText("Powered by Leptos + WASM")).toBeVisible();

    // Footer
    await expect(page.getByText("Running in Tauri WebView")).toBeVisible();

    await context.close();
  });
});

// ---------------------------------------------------------------------------
// Tier 1 – Hydration & Interactivity
// ---------------------------------------------------------------------------
test.describe("Hydration", () => {
  test("status shows 'Ready' after hydration", async ({ page }) => {
    await page.goto(BASE_URL);
    await expect(page.getByText("Ready")).toBeVisible({ timeout: 10_000 });
  });

  test("clicking 'Increment Counter' increases the displayed count", async ({
    page,
  }) => {
    await page.goto(BASE_URL);

    // Wait for hydration – status turns to "Ready" and counter is a number
    await expect(page.getByText("Ready")).toBeVisible({ timeout: 10_000 });

    // Read the current count (could be any number from prior runs)
    const counterLocator = page.locator(".tabular-nums");
    const initialText = await counterLocator.innerText();
    const initialCount = parseInt(initialText, 10);

    // Click the button
    await page
      .getByRole("button", { name: "Increment Counter" })
      .click();

    // The counter should optimistically update to initialCount + 1
    await expect(counterLocator).toHaveText(String(initialCount + 1), {
      timeout: 5_000,
    });
  });

  test("button text changes to 'Updating...' while action is pending", async ({
    page,
  }) => {
    await page.goto(BASE_URL);
    await expect(page.getByText("Ready")).toBeVisible({ timeout: 10_000 });

    // Slow down server responses so we can observe the pending state
    await page.route("**/api/increment_count", async (route) => {
      await new Promise((r) => setTimeout(r, 500));
      await route.continue();
    });

    await page
      .getByRole("button", { name: "Increment Counter" })
      .click();

    // During the server round-trip the button should say "Updating..."
    await expect(
      page.getByRole("button", { name: "Updating..." }),
    ).toBeVisible({ timeout: 2_000 });

    // And the status should change to "Syncing"
    await expect(page.getByText("Syncing")).toBeVisible({ timeout: 2_000 });

    // Eventually it settles back to "Ready"
    await expect(page.getByText("Ready")).toBeVisible({ timeout: 10_000 });
  });
});

// ---------------------------------------------------------------------------
// Tier 2 – Server Function API Tests
// ---------------------------------------------------------------------------
test.describe("Server Functions", () => {
  test("POST /api/get_count returns a valid count", async ({ request }) => {
    const response = await request.post(`${BASE_URL}/api/get_count`, {
      headers: { "Content-Type": "application/x-www-form-urlencoded" },
      data: "",
    });
    expect(response.status()).toBe(200);

    const body = await response.text();
    // Leptos returns the value as a JSON-encoded number
    const count = JSON.parse(body);
    expect(typeof count).toBe("number");
    expect(count).toBeGreaterThanOrEqual(0);
  });

  test("POST /api/increment_count succeeds", async ({ request }) => {
    // Get current count
    const before = await request.post(`${BASE_URL}/api/get_count`, {
      headers: { "Content-Type": "application/x-www-form-urlencoded" },
      data: "",
    });
    const countBefore: number = JSON.parse(await before.text());

    // Increment
    const incResponse = await request.post(
      `${BASE_URL}/api/increment_count`,
      {
        headers: { "Content-Type": "application/x-www-form-urlencoded" },
        data: "",
      },
    );
    expect(incResponse.status()).toBe(200);

    // Verify count went up
    const after = await request.post(`${BASE_URL}/api/get_count`, {
      headers: { "Content-Type": "application/x-www-form-urlencoded" },
      data: "",
    });
    const countAfter: number = JSON.parse(await after.text());
    expect(countAfter).toBe(countBefore + 1);
  });

  test("POST to a non-existent server function returns 404", async ({
    request,
  }) => {
    const response = await request.post(
      `${BASE_URL}/api/does_not_exist`,
      {
        headers: { "Content-Type": "application/x-www-form-urlencoded" },
        data: "",
      },
    );
    expect(response.status()).toBe(404);
  });
});
