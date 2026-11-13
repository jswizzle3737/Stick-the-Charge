import init, { WebHandle } from './pkg/stick_the_charge.js';
import { listPhotos, savePhoto, deletePhoto, capturePhoto } from './photos.mjs';

const loading = document.querySelector('#loading');
const dialog = document.querySelector('#photos');
const grid = document.querySelector('#photo-grid');
const status = document.querySelector('#photo-status');
const camera = document.querySelector('#camera');
const openCamera = document.querySelector('#open-camera');
const controls = document.querySelector('#camera-controls');
const preview = document.querySelector('#camera-preview');
const capture = document.querySelector('#capture-photo');
let objectUrls = [];
let stream = null;
let cameraRequest = 0;

function stopCamera() {
  const wasOpening = openCamera.disabled && !stream;
  cameraRequest++;
  if (stream) stream.getTracks().forEach(track => track.stop());
  stream = null;
  preview.srcObject = null;
  controls.hidden = true;
  dialog.classList.remove('camera-active');
  openCamera.hidden = false;
  openCamera.disabled = false;
  if (wasOpening) status.textContent = '';
}

function releaseUrls() {
  for (const url of objectUrls) URL.revokeObjectURL(url);
  objectUrls = [];
}

async function showPhotos() {
  releaseUrls();
  grid.replaceChildren();
  try {
    const photos = await listPhotos();
    if (!photos.length) {
      const empty = document.createElement('p');
      empty.className = 'empty';
      empty.textContent = 'No photos yet. Take or add one to keep it here.';
      grid.append(empty);
      return;
    }
    for (const photo of photos.reverse()) {
      const url = URL.createObjectURL(photo.blob);
      objectUrls.push(url);
      const item = document.createElement('article');
      item.className = 'photo';
      const img = document.createElement('img');
      img.src = url;
      img.alt = `Photo saved ${new Date(photo.createdAt).toLocaleString()}`;
      const actions = document.createElement('div');
      actions.className = 'photo-actions';
      const download = document.createElement('a');
      download.href = url;
      const ext = { 'image/jpeg': 'jpg', 'image/png': 'png', 'image/webp': 'webp', 'image/heic': 'heic' }[photo.blob.type] || 'image';
      download.download = `stick-the-charge-${photo.id}.${ext}`;
      download.textContent = 'Download';
      const remove = document.createElement('button');
      remove.type = 'button';
      remove.textContent = 'Delete';
      remove.setAttribute('aria-label', `Delete photo saved ${new Date(photo.createdAt).toLocaleString()}`);
      remove.addEventListener('click', async () => {
        try { await deletePhoto(photo.id); status.textContent = 'Photo deleted.'; await showPhotos(); }
        catch { status.textContent = 'Could not delete the photo. Please try again.'; }
      });
      actions.append(download, remove);
      item.append(img, actions);
      grid.append(item);
    }
  } catch { status.textContent = 'Photo storage is unavailable in this browser.'; }
}

document.querySelector('#open-photos').addEventListener('click', async () => {
  status.textContent = '';
  dialog.showModal();
  await showPhotos();
});
document.querySelector('#close-photos').addEventListener('click', () => { stopCamera(); dialog.close(); });
dialog.addEventListener('cancel', stopCamera);
dialog.addEventListener('close', () => { stopCamera(); releaseUrls(); });
document.querySelector('#cancel-camera').addEventListener('click', stopCamera);
document.addEventListener('visibilitychange', () => { if (document.hidden) stopCamera(); });
openCamera.addEventListener('click', async () => {
  const request = ++cameraRequest;
  openCamera.disabled = true;
  status.textContent = 'Opening camera…';
  try {
    if (!navigator.mediaDevices?.getUserMedia) throw new Error('Camera unavailable. Use the phone camera or image chooser below.');
    const nextStream = await navigator.mediaDevices.getUserMedia({ video: { facingMode: { ideal: 'environment' } }, audio: false });
    if (request !== cameraRequest || !dialog.open) { nextStream.getTracks().forEach(track => track.stop()); return; }
    stream = nextStream;
    preview.srcObject = stream;
    controls.hidden = false;
    dialog.classList.add('camera-active');
    await preview.play();
    if (request !== cameraRequest) return;
    openCamera.hidden = true;
    status.textContent = '';
  } catch (error) {
    if (request !== cameraRequest) return;
    stopCamera();
    status.textContent = error.name === 'NotAllowedError' ? 'Camera permission was denied. Use the image chooser below or enable camera access.' : (error.message || 'Could not open the camera. Use the image chooser below.');
  }
});
capture.addEventListener('click', async () => {
  capture.disabled = true;
  status.textContent = 'Saving photo…';
  try {
    await capturePhoto(preview);
    stopCamera();
    status.textContent = 'Photo saved on this device.';
    await showPhotos();
    navigator.storage?.persist?.().catch(() => {});
  } catch (error) { status.textContent = error.message || 'Could not save the photo. Please try again.'; }
  finally { capture.disabled = false; }
});
camera.addEventListener('change', async () => {
  const file = camera.files?.[0];
  if (!file) return;
  camera.disabled = true;
  status.textContent = 'Saving photo…';
  try {
    await savePhoto(file);
    status.textContent = 'Photo saved on this device.';
    await showPhotos();
    navigator.storage?.persist?.().catch(() => {});
  } catch (error) { status.textContent = error.message || 'Could not save the photo. Please try again.'; }
  finally { camera.disabled = false; camera.value = ''; }
});

try {
  await init();
  const handle = new WebHandle();
  await handle.start(document.querySelector('#screen'));
  loading.textContent = '';
  if ('serviceWorker' in navigator) navigator.serviceWorker.register('./sw.js').catch(() => {});
} catch (error) {
  loading.textContent = 'Could not open the app. Please reload this page.';
  console.error(error);
}
