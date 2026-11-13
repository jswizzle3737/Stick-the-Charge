import { copyFile, mkdir } from 'node:fs/promises';

const files = ['index.html', 'styles.css', 'app.mjs', 'photos.mjs', 'sw.js', 'manifest.webmanifest', 'icon.svg', 'icon-192.png', 'icon-512.png', 'pkg/stick_the_charge.js', 'pkg/stick_the_charge_bg.wasm'];
for (const file of files) {
  const destination = new URL(`../dist/${file}`, import.meta.url);
  await mkdir(new URL('.', destination), { recursive: true });
  await copyFile(new URL(`../${file}`, import.meta.url), destination);
}
console.log(`Prepared ${files.length} mobile app files in dist/.`);
