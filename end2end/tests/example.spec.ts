import { test, expect } from "@playwright/test";

test("homepage has title and links to intro page", async ({ page }) => {
	// In Tauri, we need to navigate to the actual Tauri app
	// For development, it will be the dev server
	// For production, it will be served by the Tauri webview

	let baseUrl: string;

	// Check if we're running against Tauri or development server
	if (process.env.TAURI_MODE === "production") {
		// In production mode, Tauri serves the app
		baseUrl = "http://localhost:3000/";
	} else {
		// In development mode, use the dev server
		baseUrl = "http://localhost:3000/";
	}

	await page.goto(baseUrl);

	// Wait for the page to load completely
	await page.waitForLoadState("networkidle");

	await expect(page).toHaveTitle("Welcome to Leptos");

	await expect(page.locator("h1")).toHaveText("Welcome to Leptos!");

	// Check the button exists and has the correct initial text
	const button = page.getByRole("button");
	await expect(button).toHaveText("Click Me: 0");

	// Click the button
	await button.click();

	// Verify the counter incremented
	await expect(button).toHaveText("Click Me: 1");
});

test("app loads and displays content correctly", async ({ page }) => {
	await page.goto("http://localhost:3000/");

	// Wait for the app to be fully loaded
	await page.waitForLoadState("networkidle");

	// Check that the main heading is visible
	await expect(page.locator("h1")).toBeVisible();

	// Check that the button is visible and interactive
	const button = page.getByRole("button");
	await expect(button).toBeVisible();
	await expect(button).toBeEnabled();

	// Test multiple clicks
	for (let i = 1; i <= 3; i++) {
		await button.click();
		await expect(button).toHaveText(`Click Me: ${i}`);
	}
});
