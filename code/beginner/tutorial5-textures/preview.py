import depthai as dai
import cv2

pipeline = dai.Pipeline()
color = pipeline.createColorCamera()
color.setPreviewSize(300, 300)
color.setInterleaved(False)
color.setFps(40)

xlink = pipeline.createXLinkOut()
xlink.setStreamName("preview")
color.preview.link(xlink.input)

with dai.Device(pipeline) as device:
    q = device.getOutputQueue(name="preview", maxSize=4, blocking=False)
    while True:
        data = q.get()
        cv2.imshow("preview", data.getCvFrame())
        if cv2.waitKey(1) == ord('q'):
            break

