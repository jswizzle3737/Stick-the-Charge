// Check the visible theme control and persistence on both supported phone sizes.
const assert = require('node:assert/strict');
const crypto = require('node:crypto');
const { chromium } = require(process.env.PLAYWRIGHT_MODULE || 'playwright');

const hash = bytes => crypto.createHash('sha256').update(bytes).digest('hex');
let browser;
(async () => {
  browser = await chromium.launch({ headless: true, executablePath: process.env.CHROME_PATH });
  for (const phone of [{ width: 390, x: 316, y: 110 }, { width: 320, x: 252, y: 170 }]) {
    const page = await browser.newPage({ viewport: { width: phone.width, height: 844 }, hasTouch: true, isMobile: true });
    const errors = [];
    page.on('pageerror', error => errors.push(error.message));
    await page.goto(process.env.TEST_URL || 'http://127.0.0.1:8765/');
    await page.waitForFunction(() => document.querySelector('#loading').textContent === '');
    assert.equal(await page.locator('html').getAttribute('data-theme'), 'light');
    const light = await page.screenshot();

    await page.touchscreen.tap(phone.x, phone.y);
    await page.waitForFunction(() => document.documentElement.dataset.theme === 'dark');
    assert.equal(await page.evaluate(() => localStorage.getItem('stick-the-charge-theme')), 'dark');
    assert.notEqual(hash(await page.screenshot()), hash(light), 'the rendered screen must change');
    await page.reload();
    await page.waitForFunction(() => document.documentElement.dataset.theme === 'dark' && document.querySelector('#loading').textContent === '');
    await page.locator('#open-photos').click();
    assert.equal(await page.locator('#photos').evaluate(el => getComputedStyle(el).backgroundColor), 'rgb(15, 17, 22)');
    await page.locator('#close-photos').click();

    await page.touchscreen.tap(phone.x, phone.y);
    await page.waitForFunction(() => document.documentElement.dataset.theme === 'light');
    await page.reload();
    await page.waitForFunction(() => document.documentElement.dataset.theme === 'light' && document.querySelector('#loading').textContent === '');
    assert.equal(await page.evaluate(() => localStorage.getItem('stick-the-charge-theme')), 'light');
    await page.locator('#open-photos').click();
    assert.equal(await page.locator('#photos').evaluate(el => getComputedStyle(el).backgroundColor), 'rgb(255, 255, 255)');
    assert.deepEqual(errors, []);
    await page.close();
    console.log(`${phone.width}px: both themes and saved preference passed`);
  }
  await browser.close();
})().catch(async error => { console.error(error); if (browser) await browser.close(); process.exit(1); });
