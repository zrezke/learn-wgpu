void mvcvCvtColorRGBtoNV12(u8** inR, u8** inG, u8** inB, u8** yOut, u8** uvOut, u32 width, u32 k)
{
	u32 i;

	u8* r = inR[0];
	u8* g = inG[0];
	u8* b = inB[0];
	u8* yo = yOut[0];
	u8* uvo = uvOut[0];

	int y,u1, u2, v1, v2, um, vm;
	u32 uv_idx = 0;

	for (i = 0; i < width; i+=2)
    {
        y = 0.299f * r[i] + 0.587f * g[i] + 0.114f * b[i];
        yo[i] = (u8) (y > 255 ? 255 : y < 0 ? 0 : y);

		if (k % 2 == 0) {
           	u1 = (int)(((float)b[i] - y) * 0.564f) + 128;
        	v1 = (int)(((float)r[i] - y) * 0.713f) + 128;
        }
	//-------------------------------------------------------

		y = 0.299f * r[i+1] + 0.587f * g[i+1] + 0.114f * b[i+1];
		yo[i + 1] = (u8) (y > 255 ? 255 : y < 0 ? 0 : y);

		if (k % 2 == 0) {
			u2 = (int)(((float)b[i+1] - y) * 0.564f) + 128;
			v2 = (int)(((float)r[i+1] - y) * 0.713f) + 128;

		um = (u1 + u2)/2;
        vm = (v1 + v2)/2;
        uvo[uv_idx] = (u8) (um > 255 ? 255 : um < 0 ? 0 : um);
		uvo[uv_idx + 1] = (u8) (vm > 255 ? 255 : vm < 0 ? 0 : vm);
		uv_idx = uv_idx + 2;
		}

 	}
}
