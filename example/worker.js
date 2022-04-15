import * as Comlink from "comlink";

const ditherer = {
  async dither(imgData, method) {
    const { dither } = await import("wasm-dithering");
    const dithered = dither(
      new Uint8Array(imgData.data.buffer),
      imgData.width,
      method
    );
    return Comlink.transfer(dithered, [dithered.buffer]);
  },
};

Comlink.expose(ditherer);
