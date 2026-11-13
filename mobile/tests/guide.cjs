// The five-example guide must remain searchable and saveable from the top.
const assert = require('node:assert/strict');
const { chromium } = require(process.env.PLAYWRIGHT_MODULE || 'playwright');
let browser;
(async () => {
  browser = await chromium.launch({ headless: true, executablePath: process.env.CHROME_PATH });
  const page = await browser.newPage({ viewport: { width: 390, height: 844 }, hasTouch: true, isMobile: true });
  const errors = [];
  page.on('pageerror', error => errors.push(error.message));
  await page.goto(process.env.TEST_URL || 'http://127.0.0.1:8765/');
  await page.waitForFunction(() => document.querySelector('#loading').textContent === '');
  await page.touchscreen.tap(150, 165);
  await page.keyboard.type('265');
  await page.waitForTimeout(300);
  await page.touchscreen.tap(150, 338);
  await page.waitForTimeout(300);
  await page.touchscreen.tap(309, 113);
  await page.waitForFunction(() => localStorage.getItem('stick-the-charge-saved')?.includes('VI:266'));
  for (const section of ['46(1)', '46(2)', '56.1', '72(1)', '82.3', '83.03(1)', '83.03(2)', '83.12', '83.202']) {
    await page.goto(process.env.TEST_URL || 'http://127.0.0.1:8765/');
    await page.waitForFunction(() => document.querySelector('#loading').textContent === '');
    await page.waitForTimeout(400);
    await page.touchscreen.tap(150, 165);
    await page.keyboard.type(section);
    await page.waitForTimeout(300);
    await page.touchscreen.tap(150, 338);
    await page.waitForTimeout(300);
    await page.touchscreen.tap(309, 113);
    await page.waitForFunction(id => localStorage.getItem('stick-the-charge-saved')?.includes(id), `PO:${section}`);
  }
  assert.deepEqual(errors, []);
  await browser.close();
  console.log('Guide search and save check passed.');
})().catch(async error => { console.error(error); if (browser) await browser.close(); process.exit(1); });
