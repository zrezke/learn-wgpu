import depthai as dai
import numpy as np
import time
import sys
import cv2

if len(sys.argv) > 1 and sys.argv[1] == "d":
    width = 1920
    height = 1080

    arr = np.repeat(np.array([0], dtype=np.uint8), width * height).reshape(height, width)
    uv = np.tile(np.tile(np.array([128, 12], dtype=np.uint8), width // 2), height // 4)
    uv2 = np.tile(np.tile(np.array([10, 15], dtype=np.uint8), width // 2), height // 4)
    uv = np.append(uv, uv2)
    arr = np.append(arr, uv)

    print(arr.reshape(int(height * 1.5), width))
    print(arr.shape)
    open("frame.nv12", "wb").write(arr.tobytes())
else:
    p = dai.Pipeline()

    c = p.createColorCamera()
    c.setResolution(dai.ColorCameraProperties.SensorResolution.THE_1080_P)
    c.setInterleaved(False)

    xlink = p.createXLinkOut()
    xlink.setStreamName("video")
    c.video.link(xlink.input)

    start = time.time()
    with dai.Device(p) as device:
        q = device.getOutputQueue("video", maxSize=1, blocking=False)
        while True:
            frame = q.get()

            cv2.imshow("frame", cv2.cvtColor(frame.getData().reshape((int(1080 * 1.5), 1920)), cv2.COLOR_YUV2BGR_NV12))
            if cv2.waitKey(1) == ord('q'):
                exit(0)
            if start + 5 < time.time():
                print("Writing frame...")
                open("frame.nv12", "wb").write(frame.getData())
                exit()
