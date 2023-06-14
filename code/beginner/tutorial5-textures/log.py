import depthai_viewer as viewer
import numpy as np
import time

viewer.init("Depthai Viewer")
viewer.connect()
frame = np.fromfile("frame.nv12", dtype=np.uint8).reshape(int(1080 * 1.5), 1920)
while True:
    viewer.log_encoded_image("frame", frame, 1920, 1080, viewer.ImageEncoding.NV12)
    time.sleep(0.1)
