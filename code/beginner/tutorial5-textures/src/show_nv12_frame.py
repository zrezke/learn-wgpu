import cv2
import numpy as np


# b = np.fromfile("/mnt/fast/dev/learn-wgpu/code/beginner/tutorial5-textures/src/nv12frame.npy", dtype=np.uint8)
b = np.fromfile("/mnt/fast/dev/learn-wgpu/code/beginner/tutorial5-textures/frame.nv12", dtype=np.uint8)

b = b.reshape(int(1080 * 1.5), 1920)

# b = np.fromfile("/mnt/fast/dev/learn-wgpu/code/beginner/tutorial5-textures/nv12frame.yuv", dtype=np.uint8)

# b = b.reshape(3732480)
# b = b[:int(1920 * 1080 * 1.5)]

# b = b.reshape(int(1080 * 1.5), 1920)


ys = b[:1080]

while True:
    # cv2.imshow("frame", ys)
    cv2.imshow("frame", cv2.cvtColor(b, cv2.COLOR_YUV2BGR_NV12))

    if cv2.waitKey(1) == ord('q'):
        exit(0)
