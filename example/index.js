import { Method } from "wasm-dithering";
import * as Comlink from "comlink";

const canvas = document.getElementById("canvas");

const img = document.getElementById("image");
img.addEventListener("change", async () => {
  const file = img.files[0];
  const imgData = await blobToImageData(file);

  if (window.Worker) {
    const d = Comlink.wrap(new Worker(new URL("./worker.js", import.meta.url)));

    const dithered = await d.dither(imgData, Method.FloydSteinberg);
    const ditheredImgData = new ImageData(dithered, imgData.width);

    canvas.width = imgData.width;
    canvas.height = imgData.height;
    canvas.getContext("2d").putImageData(ditheredImgData, 0, 0);
  }
});

function blobToImageData(blob) {
  const url = URL.createObjectURL(blob);
  return imageFileToImageData(url);
}

async function imageFileToImageData(url) {
  const img = document.createElement("img");
  img.src = url;
  await new Promise((resolve, reject) => {
    img.onload = resolve;
    img.onerror = reject;
  });
  return imageToImageData(img);
}

function imageToImageData(img) {
  const cvs = document.createElement("canvas");
  cvs.width = img.naturalWidth;
  cvs.height = img.naturalHeight;
  const ctx = cvs.getContext("2d");
  ctx.drawImage(img, 0, 0);
  return ctx.getImageData(0, 0, cvs.width, cvs.height);
}
