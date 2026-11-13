const DB_NAME = 'stick-the-charge-photos';

function openDatabase() {
  return new Promise((resolve, reject) => {
    const request = indexedDB.open(DB_NAME, 1);
    request.onupgradeneeded = () => request.result.createObjectStore('photos', { keyPath: 'id', autoIncrement: true });
    request.onsuccess = () => resolve(request.result);
    request.onerror = () => reject(request.error);
  });
}

async function transaction(mode, action) {
  const db = await openDatabase();
  return new Promise((resolve, reject) => {
    const tx = db.transaction('photos', mode);
    const request = action(tx.objectStore('photos'));
    tx.oncomplete = () => { db.close(); resolve(request.result); };
    tx.onerror = () => { db.close(); reject(tx.error); };
    tx.onabort = () => { db.close(); reject(tx.error || new Error('Photo storage was interrupted.')); };
  });
}

export const listPhotos = () => transaction('readonly', store => store.getAll());
export const deletePhoto = id => transaction('readwrite', store => store.delete(id));

export async function capturePhoto(video) {
  if (!video.videoWidth || !video.videoHeight) throw new Error('The camera is not ready yet.');
  const canvas = document.createElement('canvas');
  canvas.width = video.videoWidth;
  canvas.height = video.videoHeight;
  const context = canvas.getContext('2d');
  if (!context) throw new Error('The camera image could not be captured.');
  context.drawImage(video, 0, 0);
  const blob = await new Promise(resolve => canvas.toBlob(resolve, 'image/jpeg', 0.9));
  if (!blob) throw new Error('The camera image could not be captured.');
  return savePhoto(blob);
}

export async function savePhoto(file) {
  if (!file || !file.type.startsWith('image/')) throw new Error('Please select an image.');
  const url = URL.createObjectURL(file);
  try {
    const image = new Image();
    image.src = url;
    await image.decode();
    if (!image.naturalWidth || !image.naturalHeight) throw new Error('The image is empty.');
  } catch {
    throw new Error('That image could not be opened. Please try another photo.');
  } finally {
    URL.revokeObjectURL(url);
  }
  return transaction('readwrite', store => store.add({ blob: file, createdAt: Date.now() }));
}
