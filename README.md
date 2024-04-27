# Fire

Displays a pixelated fire in a 40x32 (1280 pixels) WS2812 matrix. The matrix is scaled down to 20x16 and runs off an ESP32.

The matrix is limited to 25fps due to LED count.

There's a basic canvas renderer using WASM.

Looks something like this:

https://github.com/DavidVentura/matrix-fire/assets/3650670/b9f5ad1b-9b1f-4d2b-a132-f6941ccc0aff

And something like this (though it looks a lot nicer in real life)

https://github.com/DavidVentura/matrix-fire/assets/3650670/81689af4-8160-4d21-9c5c-98d44354a97a


## Parts

- 5x [32x8 WS2812B led matrix](https://www.aliexpress.com/item/1005001265647648.html)
- Some laser-cut pieces of cardboard to make baffles. Files are at `parts/`.
	- I used a 50x70cm, 3mm thick piece of cardboard.
- ESP32
- 5v power supply appropriate for your chosen brigthness; I bought a 10A supply.


## Other

I made the baffles 2.5cm deep, which looks nice, but may be too deep, so the brightness has to be increased (to ~100 in my case).
Probably 1.5cm would be better, but I've not tested it.
