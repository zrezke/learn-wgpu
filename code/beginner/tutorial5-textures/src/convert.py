import cv2
import numpy as np


b = np.fromfile("/mnt/fast/dev/learn-wgpu/code/beginner/tutorial5-textures/src/nv12frame.npy", dtype=np.uint8)

b = b.reshape(int(1080 * 1.5), 1920)

rgb = np.zeros((1080, 1920, 4), dtype=np.uint8)

for x in range(1920):
    for y in range(1080):
        y = b[y][x]
        u = b[int(1080 + y // 2)][(x // 2) * 2]
        v = b[int(1080 + y // 2)][(x // 2) * 2 + 1]
        red = y + 1.402 * (v - 128)
        green = y - 0.34414 * (u - 128) - 0.71414 * (v - 128)
        blue = y + 1.772 * (u - 128)
        rgb[y][x] = [red, green, blue, 255]


cv2.imshow("frame", rgb)
cv2.waitKey(0)
