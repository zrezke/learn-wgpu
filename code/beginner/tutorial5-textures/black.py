import cv2
import numpy as np

rgb = 0

y = 0
u = 0
v = 0

open("src/black.nv12", "wb").write(np.zeros(3110400, dtype=np.uint8).tobytes())
