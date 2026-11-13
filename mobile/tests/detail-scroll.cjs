// Verifies that a previously scrolled charge does not hide the next explanation.
const assert = require('node:assert/strict');
const { createHash } = require('node:crypto');
const { chromium } = require(process.env.PLAYWRIGHT_MODULE || 'playwright');
const hash = bytes => createHash('sha256').update(bytes).digest('hex');
let browser;

(async () => {
  browser = await chromium.launch({ headless: true, executablePath: process.env.CHROME_PATH });
  const context = await browser.newContext({ viewport: { width: 390, height: 844 }, deviceScaleFactor: 1, hasTouch: true, isMobile: true });
  const page = await context.newPage();
  page.setDefaultTimeout(10000);
  async function load() {
    await page.goto(process.env.TEST_URL || 'http://127.0.0.1:8765/');
    await page.waitForFunction(() => document.querySelector('#loading').textContent === '');
  }
  async function searchAndOpen(number, replace = false) {
    await page.touchscreen.tap(140, 165);
    if (replace) await page.keyboard.press('Control+A');
    await page.keyboard.type(number);
    await page.waitForTimeout(200);
    await page.touchscreen.tap(150, 338);
    await page.waitForTimeout(300);
  }
  const clip = { x: 43, y: 132, width: 300, height: 500 };
  await load();
  await searchAndOpen('266');
  const touch = await context.newCDPSession(page);
  await touch.send('Input.dispatchTouchEvent', { type: 'touchStart', touchPoints: [{ x: 180, y: 660 }] });
  for (const y of [620, 570, 520, 470, 420, 370, 320]) await touch.send('Input.dispatchTouchEvent', { type: 'touchMove', touchPoints: [{ x: 180, y }] });
  await touch.send('Input.dispatchTouchEvent', { type: 'touchEnd', touchPoints: [] });
  await page.waitForTimeout(500);
  await page.touchscreen.tap(58, 112);
  await searchAndOpen('264.1', true);
  const afterPreviousCharge = await page.screenshot({ clip });
  await load();
  await searchAndOpen('264.1');
  const fromFreshLaunch = await page.screenshot({ clip });
  assert.equal(hash(afterPreviousCharge), hash(fromFreshLaunch), 'a new charge must start at its explanation');
  await browser.close();
  console.log('Detail scroll reset check passed.');
})().catch(async error => { console.error(error); if (browser) await browser.close(); process.exit(1); });
