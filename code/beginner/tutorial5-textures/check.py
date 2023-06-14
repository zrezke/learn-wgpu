import numpy as np
import cv2

frame = np.fromfile("nv12frame.yuv", dtype=np.uint8).reshape(int(1080 * 1.5), 2304)[:, :1920]
frame = frame.copy() # To make it c contiguous

print(frame.shape)
open("cmp.nv12", "wb").write(frame)


cv2.imshow("frame", cv2.cvtColor(frame, cv2.COLOR_YUV2BGR_NV12))
if cv2.waitKey(0) == 'q':
    exit(0)
