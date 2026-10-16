"""Generate a seamless circular liquid-glass button sprite sheet.
Visual reference: dexter-st / breezy-zebra-94 (Uiverse, MIT).
Requires Pillow and NumPy; no browser or network needed at runtime.
"""
from pathlib import Path
import math
import numpy as np
from PIL import Image

ROOT = Path(__file__).resolve().parents[1]
DEST = ROOT / 'app-slint' / 'pets' / 'default'
FRAME_W, FRAME_H, COLS, FRAMES = 192, 208, 6, 48

def mix(rgb, color, amount):
    a = np.clip(amount, 0, 1)[..., None]
    return rgb * (1-a) + np.array(color) * a

def frame(index, width=FRAME_W, height=FRAME_H, supersample=4):
    y, x = np.mgrid[:height*supersample, :width*supersample].astype(float)
    x = ((x + .5) / supersample - width / 2) / (width * 68 / 192)
    y = ((y + .5) / supersample - height / 2) / (height * 68 / 208)
    r = np.hypot(x, y)
    t = index / FRAMES * math.tau

    flow = 2.5*y + 1.2*np.sin(2.2*x + t) + .55*np.sin(3*y - t)
    band = .5 + .5*np.sin(flow*2.4 + .45*np.sin(t))
    rgb = np.zeros((*r.shape, 3)) + [81, 68, 226]
    rgb = mix(rgb, [105, 223, 255], band*.88)
    lavender = np.exp(-((x+.35*np.cos(t))**2/.7 + (y-.3*np.sin(t))**2/.20))
    rgb = mix(rgb, [224, 154, 255], lavender*.85)
    streak = np.exp(-((y + .26 + .25*np.sin(2*x+t))/.19)**2)
    rgb = mix(rgb, [231, 251, 255], streak*.92)
    top = np.exp(-((x+.28)**2/.65 + (y+.64)**2/.12))
    rgb = mix(rgb, [255, 255, 249], top*.96)
    rgb *= (1-.40*np.clip(y, 0, 1)[...,None])
    lens = np.exp(-((x+.27)**2/.20 + (y+.3)**2/.32))
    rgb = mix(rgb, [251, 255, 255], lens*.54)
    glint = np.exp(-((x+.35)**2/.16 + (y+.73)**2/.007))
    rgb = mix(rgb, [255, 255, 255], glint*.92)

    # Smooth coverage mask with extra transparent samples around the edge.
    coverage = np.clip((1-r)*68/1.25+.5, 0, 1)
    alpha = coverage*coverage*(3-2*coverage)
    rgba = np.dstack((np.clip(rgb,0,255), alpha*255)).astype('uint8')
    # Premultiplied resampling prevents pale/black fringes on transparency.
    return (Image.fromarray(rgba).convert('RGBa')
            .resize((width, height), Image.Resampling.LANCZOS).convert('RGBA'))

def main():
    DEST.mkdir(parents=True, exist_ok=True)
    sheet = Image.new('RGBA', (FRAME_W*COLS, FRAME_H*(FRAMES//COLS)))
    for i in range(FRAMES):
        sheet.paste(frame(i), ((i%COLS)*FRAME_W, (i//COLS)*FRAME_H))
    sheet.save(DEST/'spritesheet.png', optimize=True)
    # The launcher is displayed at roughly 50px. Keep a separate 2x texture
    # so the final reduction has enough samples for a clean transparent edge.
    orb_w, orb_h = FRAME_W * 2, FRAME_H * 2
    orb = Image.new('RGBA', (orb_w*COLS, orb_h*(FRAMES//COLS)))
    for i in range(FRAMES):
        orb.paste(frame(i, orb_w, orb_h, 2), ((i%COLS)*orb_w, (i//COLS)*orb_h))
    orb.save(DEST/'orb.png', optimize=True)
    preview = Image.new('RGB', (576,256), '#101117')
    for n, i in enumerate((0,16,32)):
        img = frame(i, orb_w, orb_h, 2).resize((192, 208), Image.Resampling.LANCZOS)
        preview.paste(img, (n*192,24), img)
    preview_dir = ROOT/'app-slint'/'target'
    preview_dir.mkdir(exist_ok=True)
    preview.save(preview_dir/'liquid-orb-preview.png')
    print(f'Generated {FRAMES} frames: {sheet.size}, {DEST}')

if __name__ == '__main__':
    main()
