// Run against the local server; see README.md for the Playwright setup.
const assert = require('node:assert/strict');
const { createHash } = require('node:crypto');
const fs = require('node:fs/promises');
const path = require('node:path');
const { chromium } = require(process.env.PLAYWRIGHT_MODULE || 'playwright');
const hash = bytes => createHash('sha256').update(bytes).digest('hex');
let browser;

(async () => {
  browser = await chromium.launch({ headless: true, executablePath: process.env.CHROME_PATH, args: ['--use-fake-device-for-media-stream', '--use-fake-ui-for-media-stream'] });
  const context = await browser.newContext({ viewport: { width: 390, height: 844 }, deviceScaleFactor: 1, hasTouch: true, isMobile: true, acceptDownloads: true });
  const page = await context.newPage();
  page.setDefaultTimeout(10000);
  await context.route('https://laws-lois.justice.gc.ca/**', route => route.fulfill({ status: 200, contentType: 'text/html', body: '<!doctype html><title>Legislation test</title>' }));
  const errors = [];
  page.on('pageerror', error => errors.push(error.message));
  const load = async () => {
    await page.goto(process.env.TEST_URL || 'http://127.0.0.1:8765/');
    await page.waitForFunction(() => document.querySelector('#loading').textContent === '');
  };

  await load();
  await page.evaluate(() => navigator.serviceWorker.ready);
  // A category selected after swiping should show its first result, not retain the old offset.
  const listClip = { x: 45, y: 302, width: 295, height: 345 };
  await page.waitForTimeout(500);
  const beforeSwipe = await page.screenshot({ clip: listClip });
  const touch = await context.newCDPSession(page);
  await touch.send('Input.dispatchTouchEvent', { type: 'touchStart', touchPoints: [{ x: 170, y: 610 }] });
  for (const y of [570, 520, 470, 420, 380]) await touch.send('Input.dispatchTouchEvent', { type: 'touchMove', touchPoints: [{ x: 170, y }] });
  await touch.send('Input.dispatchTouchEvent', { type: 'touchEnd', touchPoints: [] });
  await page.waitForTimeout(500);
  const afterSwipe = await page.screenshot({ clip: listClip });
  assert.notEqual(hash(afterSwipe), hash(beforeSwipe), 'swiping the charge list should move its contents');
  await page.touchscreen.tap(113, 214);
  await page.waitForTimeout(600);
  const filteredAfterSwipe = await page.screenshot({ clip: listClip });
  await load();
  await page.touchscreen.tap(113, 214);
  await page.waitForTimeout(600);
  const filteredFromTop = await page.screenshot({ clip: listClip });
  assert.equal(hash(filteredAfterSwipe), hash(filteredFromTop), 'selecting a category should return its results to the top');
  await load();
  await page.touchscreen.tap(150, 165);
  await page.keyboard.type('344');
  await page.touchscreen.tap(150, 338);
  await page.waitForTimeout(200);
  const [definition] = await Promise.all([
    page.waitForEvent('popup', { timeout: 4000 }),
    page.touchscreen.tap(140, 319),
  ]);
  await definition.waitForURL('https://laws-lois.justice.gc.ca/eng/acts/C-46/section-344.html');
  await definition.close();
  await load();
  await page.waitForTimeout(400);
  // Open an entry without a long draft and save it through the rendered Rust UI.
  await page.touchscreen.tap(150, 165);
  await page.keyboard.type('6(1)');
  await page.mouse.click(160, 338);
  await page.waitForTimeout(200);
  const [legislation] = await Promise.all([
    page.waitForEvent('popup', { timeout: 4000 }),
    page.touchscreen.tap(160, 346),
  ]);
  await legislation.waitForURL('https://laws-lois.justice.gc.ca/eng/acts/C-38.8/section-6.html');
  await legislation.close();
  assert.match(page.url(), /127\.0\.0\.1/, 'opening the statute must leave the app open');
  await page.mouse.click(309, 113);
  await page.waitForFunction(() => localStorage.getItem('stick-the-charge-saved')?.includes('DR:6(1)'));
  await load();
  assert.match(await page.evaluate(() => localStorage.getItem('stick-the-charge-saved')), /DR:6\(1\)/);
  await page.touchscreen.tap(195, 706);
  await page.waitForTimeout(400);
  const savedClip = { x: 45, y: 155, width: 295, height: 160 };
  const savedAfterReload = await page.screenshot({ clip: savedClip });
  await page.touchscreen.tap(81, 706);
  await page.touchscreen.tap(113, 214);
  await page.touchscreen.tap(140, 165);
  await page.keyboard.type('assault');
  await page.touchscreen.tap(195, 706);
  await page.waitForTimeout(400);
  const savedAfterSearch = await page.screenshot({ clip: savedClip });
  assert.equal(hash(savedAfterSearch), hash(savedAfterReload), 'saved charges must stay visible when the main list is filtered');
  async function savedView(id) {
    await page.evaluate(value => localStorage.setItem('stick-the-charge-saved', value), id);
    await load();
    await page.touchscreen.tap(195, 706);
    await page.waitForTimeout(300);
    return page.screenshot({ clip: savedClip });
  }
  const legacyProperty = await savedView('PR:355');
  const correctedProperty = await savedView('PR:354');
  const emptySaved = await savedView('');
  assert.equal(hash(legacyProperty), hash(correctedProperty), 'the old saved section must open the corrected charge');
  assert.notEqual(hash(correctedProperty), hash(emptySaved), 'the corrected charge must be visible in the saved list');
  await page.evaluate(() => localStorage.setItem('stick-the-charge-saved', 'DR:4(1)'));
  await load();

  await page.getByRole('button', { name: 'Photos', exact: true }).click();
  await page.setViewportSize({ width: 320, height: 568 });
  await page.getByRole('button', { name: 'Open camera' }).click({ timeout: 4000 });
  await page.waitForFunction(() => document.querySelector('#camera-preview')?.videoWidth > 0);
  await page.locator('#open-camera').waitFor({ state: 'hidden' });
  const dialogBounds = await page.locator('#photos').boundingBox();
  const captureBounds = await page.getByRole('button', { name: 'Take photo', exact: true }).boundingBox();
  assert.ok(captureBounds.y + captureBounds.height <= dialogBounds.y + dialogBounds.height, 'capture button must be visible on a small screen');
  const mediaKinds = await page.evaluate(() => {
    const tracks = document.querySelector('#camera-preview').srcObject.getTracks();
    window.testCameraTrack = tracks[0];
    return tracks.map(track => track.kind);
  });
  assert.deepEqual(mediaKinds, ['video'], 'opening the camera must not request audio');
  await page.getByRole('button', { name: 'Take photo', exact: true }).click();
  await page.getByText('Photo saved on this device.').waitFor();
  assert.equal(await page.evaluate(() => window.testCameraTrack.readyState), 'ended', 'camera must turn off after capture');
  await page.getByRole('button', { name: 'Open camera' }).click();
  await page.waitForFunction(() => document.querySelector('#camera-preview')?.srcObject?.active);
  await page.evaluate(() => { window.testCameraTrack = document.querySelector('#camera-preview').srcObject.getVideoTracks()[0]; });
  await page.getByRole('button', { name: 'Close photos' }).click();
  assert.equal(await page.evaluate(() => window.testCameraTrack.readyState), 'ended', 'camera must turn off when photos closes');

  // Simulate permission arriving after the panel closes or the app goes into the background.
  await page.getByRole('button', { name: 'Photos', exact: true }).click();
  await page.evaluate(async () => {
    window.originalGetUserMedia = navigator.mediaDevices.getUserMedia.bind(navigator.mediaDevices);
    window.pendingStream = await window.originalGetUserMedia({ video: true, audio: false });
    navigator.mediaDevices.getUserMedia = () => new Promise(resolve => { window.resolvePendingCamera = () => resolve(window.pendingStream); });
  });
  await page.getByRole('button', { name: 'Open camera' }).click();
  await page.getByText('Opening camera…').waitFor();
  await page.getByRole('button', { name: 'Close photos' }).click();
  await page.evaluate(() => window.resolvePendingCamera());
  await page.waitForFunction(() => window.pendingStream.getVideoTracks()[0].readyState === 'ended');
  await page.getByRole('button', { name: 'Photos', exact: true }).click();
  await page.evaluate(async () => { window.pendingStream = await window.originalGetUserMedia({ video: true, audio: false }); });
  await page.getByRole('button', { name: 'Open camera' }).click();
  await page.getByText('Opening camera…').waitFor();
  await page.evaluate(() => {
    Object.defineProperty(document, 'hidden', { value: true, configurable: true });
    document.dispatchEvent(new Event('visibilitychange'));
    delete document.hidden;
    window.resolvePendingCamera();
  });
  await page.waitForFunction(() => window.pendingStream.getVideoTracks()[0].readyState === 'ended');
  assert.equal(await page.locator('#photo-status').innerText(), '', 'a cancelled background camera request must not remain in a loading state');
  assert.equal(await page.locator('#open-camera').isEnabled(), true);
  await page.evaluate(() => { navigator.mediaDevices.getUserMedia = window.originalGetUserMedia; });
  await page.getByRole('button', { name: 'Close photos' }).click();
  await page.setViewportSize({ width: 390, height: 844 });
  await page.reload();
  await page.getByRole('button', { name: 'Photos', exact: true }).click();
  await page.waitForFunction(() => document.querySelector('.photo img')?.naturalWidth > 0);
  const capturedColours = await page.locator('.photo img').evaluate(image => {
    const canvas = document.createElement('canvas');
    canvas.width = 64;
    canvas.height = 64;
    const context = canvas.getContext('2d');
    context.drawImage(image, 0, 0, canvas.width, canvas.height);
    const pixels = context.getImageData(0, 0, canvas.width, canvas.height).data;
    const colours = new Set();
    for (let i = 0; i < pixels.length; i += 4) colours.add(`${pixels[i]},${pixels[i + 1]},${pixels[i + 2]}`);
    return colours.size;
  });
  assert.ok(capturedColours > 3, 'camera capture must decode to a non-blank image');
  const [cameraDownload] = await Promise.all([page.waitForEvent('download'), page.getByRole('link', { name: 'Download' }).click()]);
  const captured = await fs.readFile(await cameraDownload.path());
  assert.equal(captured[0], 0xff, 'camera capture must contain JPEG bytes');
  assert.equal(captured[1], 0xd8, 'camera capture must contain JPEG bytes');
  assert.ok(captured.length > 1000, 'camera capture must contain image data');
  await page.getByRole('button', { name: /Delete photo/ }).click();
  await page.getByText('No photos yet. Take or add one to keep it here.').waitFor();
  await page.evaluate(() => { navigator.mediaDevices.getUserMedia = async () => { throw new DOMException('Denied', 'NotAllowedError'); }; });
  await page.getByRole('button', { name: 'Open camera' }).click();
  await page.getByText('Camera permission was denied. Use the image chooser below or enable camera access.').waitFor();
  assert.equal(await page.locator('#camera').isEnabled(), true);
  assert.equal(await page.locator('#camera').getAttribute('capture'), 'environment');
  await page.locator('#camera').setInputFiles({ name: 'broken.png', mimeType: 'image/png', buffer: Buffer.from('not an image') });
  await page.getByText('That image could not be opened. Please try another photo.').waitFor();
  assert.equal(await page.locator('.photo').count(), 0);
  const fixture = path.resolve(__dirname, '..', 'icon-192.png');
  const original = await fs.readFile(fixture);
  await page.locator('#camera').setInputFiles(fixture);
  await page.getByText('Photo saved on this device.').waitFor();
  await page.locator('.photo img').waitFor();
  await page.reload();
  await page.getByRole('button', { name: 'Photos', exact: true }).click();
  const image = page.locator('.photo img');
  await image.waitFor();
  await page.waitForFunction(() => document.querySelector('.photo img')?.naturalWidth === 192);
  const [download] = await Promise.all([page.waitForEvent('download'), page.getByRole('link', { name: 'Download' }).click()]);
  const downloaded = await fs.readFile(await download.path());
  assert.deepEqual(downloaded, original, 'the saved and downloaded PNG must match the selected image byte for byte');
  await page.setViewportSize({ width: 320, height: 568 });
  const card = await page.locator('.photo').boundingBox();
  const remove = await page.getByRole('button', { name: /Delete photo/ }).boundingBox();
  assert.ok(remove.x >= card.x && remove.x + remove.width <= card.x + card.width, 'photo controls must fit inside their card');
  await page.getByRole('button', { name: /Delete photo/ }).click();
  await page.getByText('No photos yet. Take or add one to keep it here.').waitFor();
  await page.reload();
  await page.getByRole('button', { name: 'Photos', exact: true }).click();
  await page.getByText('No photos yet. Take or add one to keep it here.').waitFor();
  assert.deepEqual(errors, []);

  await page.getByRole('button', { name: 'Close photos' }).click();
  await page.evaluate(() => navigator.serviceWorker.ready);
  await context.setOffline(true);
  await load();
  assert.match(await page.evaluate(() => localStorage.getItem('stick-the-charge-saved')), /DR:4\(1\)/);
  await browser.close();
  console.log('Browser checks passed: charge save/reload, camera capture/reload/download, image import/delete, and offline launch.');
})().catch(async error => { console.error(error); if (browser) await browser.close(); process.exit(1); });
