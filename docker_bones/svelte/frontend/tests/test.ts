import { expect, test } from '@playwright/test';

// test('index page has expected h1', async ({ page }) => {
// 	await page.goto('/');
// 	await expect(page.getByRole('heading', { name: 'Welcome to SvelteKit' })).toBeVisible();
// });


test('login username field populating',async ({ page, context })=>{
	await page.goto('/');
	await page.getByTestId('username_login').fill('bee');
	await page.getByTestId('password_login').fill('bee');
	await page.getByTestId('button_login').click();
	const cookies = await context.cookies();
    console.log(JSON.stringify(cookies, null, 2));
	await expect(page.getByText('Username')).toBeVisible();


});
