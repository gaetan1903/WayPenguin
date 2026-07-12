#!/usr/bin/env python3
"""Generate clean hand-drawn vector Tux poses for WayPenguin activities.

Each activity is a single-pose SVG on a 100x100 viewBox. A shared set of
part-builders keeps anatomy and palette consistent across poses.
"""
import os

BLACK = "#1b1b1b"
BELLY = "#f4f4f4"
ORANGE = "#f7a70b"
ORANGE_D = "#d4870a"
GREEN = "#3aa655"
GREEN_D = "#2f8a45"
SCARF = "#1f7a4d"
BLUE = "#2b6cb0"
BLUE_L = "#5b9bd5"
HALO = "#ffe98a"
WING = "#fbfbfb"
WING_SH = "#dfe4ea"
RED = "#cf1020"
RED_D = "#8f0a15"

# ---- part builders -------------------------------------------------------

def foot(cx, cy=95, rx=10, ry=5, rot=0):
    return (f'<g transform="rotate({rot} {cx} {cy})">'
            f'<ellipse cx="{cx}" cy="{cy}" rx="{rx}" ry="{ry}" '
            f'fill="{ORANGE}" stroke="{ORANGE_D}" stroke-width="1"/></g>')

def eye(cx, cy=33, look=0.0):
    px = cx + look * 2.5
    return (f'<ellipse cx="{cx}" cy="{cy}" rx="6" ry="8" fill="#fff"/>'
            f'<ellipse cx="{px}" cy="{cy+2}" rx="2.6" ry="3.4" fill="{BLACK}"/>')

def beak(cx=50, cy=42):
    return (f'<path d="M{cx} {cy-7} C{cx-6} {cy-7} {cx-10} {cy-2} {cx-10} {cy+2} '
            f'C{cx-10} {cy+5} {cx-5} {cy+7} {cx} {cy+7} '
            f'C{cx+5} {cy+7} {cx+10} {cy+5} {cx+10} {cy+2} '
            f'C{cx+10} {cy-2} {cx+6} {cy-7} {cx} {cy-7} Z" '
            f'fill="{ORANGE}" stroke="{ORANGE_D}" stroke-width="0.8"/>'
            f'<path d="M{cx-10} {cy+2} C{cx-4} {cy+5} {cx+4} {cy+5} {cx+10} {cy+2}" '
            f'fill="none" stroke="{ORANGE_D}" stroke-width="1.1"/>')

BODY = (f'<path d="M50 8 C31 8 21 27 21 52 C21 80 33 94 50 94 '
        f'C67 94 79 80 79 52 C79 27 69 8 50 8 Z" fill="{BLACK}"/>')
BELLY_P = (f'<path d="M50 40 C37 40 30 55 30 72 C30 87 39 92 50 92 '
           f'C61 92 70 87 70 72 C70 55 63 40 50 40 Z" fill="{BELLY}"/>')

def flipper_down(side):
    if side == "L":
        return (f'<path d="M22 44 C12 50 11 74 17 85 C21 89 26 83 26 74 '
                f'C26 62 27 52 29 46 Z" fill="{BLACK}"/>')
    return (f'<path d="M78 44 C88 50 89 74 83 85 C79 89 74 83 74 74 '
            f'C74 62 73 52 71 46 Z" fill="{BLACK}"/>')

def flipper_out(side):
    if side == "L":
        return (f'<path d="M23 45 C8 38 -1 40 1 50 C2 56 9 58 18 55 '
                f'C23 53 25 50 27 47 Z" fill="{BLACK}"/>')
    return (f'<path d="M77 45 C92 38 101 40 99 50 C98 56 91 58 82 55 '
            f'C77 53 75 50 73 47 Z" fill="{BLACK}"/>')

def flipper_up(side):
    if side == "L":
        return (f'<path d="M25 44 C15 28 9 16 14 13 C19 10 24 20 29 33 '
                f'C31 39 29 43 27 46 Z" fill="{BLACK}"/>')
    return (f'<path d="M75 44 C85 28 91 16 86 13 C81 10 76 20 71 33 '
            f'C69 39 71 43 73 46 Z" fill="{BLACK}"/>')

def svg(body, vb="0 0 100 100"):
    return (f'<svg xmlns="http://www.w3.org/2000/svg" viewBox="{vb}">\n'
            f'{body}\n</svg>\n')

# ---- poses ---------------------------------------------------------------

def front(flippers, eyes_look=0.0, extra_front="", extra_back="", feet=None):
    if feet is None:
        feet = foot(37, rot=-8) + foot(63, rot=8)
    fl = "".join(flippers)
    return "\n".join([
        extra_back, feet, BODY, fl, BELLY_P,
        eye(42, look=eyes_look), eye(58, look=eyes_look), beak(),
        extra_front,
    ])

def pose_faller():
    # arms spread, feet splayed, wide eyes
    feet = foot(30, cy=96, rot=-22) + foot(70, cy=96, rot=22)
    return svg(front([flipper_out("L"), flipper_out("R")], feet=feet))

def pose_tumbler():
    # arms out, whole body tilted
    feet = foot(34, cy=95, rot=-14) + foot(66, cy=95, rot=14)
    inner = front([flipper_out("L"), flipper_out("R")], feet=feet)
    return svg(f'<g transform="rotate(-12 50 52)">{inner}</g>')

def pose_floater():
    # propeller beanie + scarf, arms down
    hat = (f'<rect x="30" y="-3" width="40" height="3.2" rx="1.2" fill="{BLUE_L}"/>'
           f'<rect x="47" y="-6" width="6" height="5" fill="{BLUE_D if False else BLUE}"/>'
           f'<path d="M33 14 C33 4 67 4 67 14 Z" fill="{BLUE}"/>'
           f'<rect x="31" y="12" width="38" height="4" rx="2" fill="{BLUE_L}"/>')
    scarf = (f'<path d="M32 38 C42 46 58 46 68 38 L68 44 C58 51 42 51 32 44 Z" fill="{SCARF}"/>'
             f'<path d="M62 44 L70 60 L64 61 L58 46 Z" fill="{SCARF}"/>')
    return svg(front([flipper_down("L"), flipper_down("R")],
                     extra_back=hat, extra_front=scarf))

BLUE_D = "#1e4e7a"

def pose_angel():
    wingL = (f'<path d="M30 40 C12 30 2 34 6 44 C8 50 16 50 22 48 '
             f'C14 52 8 58 12 64 C16 68 26 62 32 54 Z" '
             f'fill="{WING}" stroke="{WING_SH}" stroke-width="0.8"/>')
    wingR = (f'<path d="M70 40 C88 30 98 34 94 44 C92 50 84 50 78 48 '
             f'C86 52 92 58 88 64 C84 68 74 62 68 54 Z" '
             f'fill="{WING}" stroke="{WING_SH}" stroke-width="0.8"/>')
    halo = (f'<ellipse cx="50" cy="2" rx="15" ry="4.5" fill="none" '
            f'stroke="{HALO}" stroke-width="2.4"/>')
    inner = front([flipper_down("L"), flipper_down("R")],
                  extra_back=wingL + wingR, extra_front=halo)
    return svg(f'<g opacity="0.85">{inner}</g>')

def pose_walker():
    # side profile facing left
    body = (f'<ellipse cx="43" cy="94" rx="13" ry="5" fill="{ORANGE}" stroke="{ORANGE_D}" stroke-width="1"/>'
            # back/head silhouette
            f'<path d="M62 40 C62 20 52 10 42 10 C30 10 24 22 24 38 '
            f'C24 44 22 50 22 64 C22 82 30 92 44 92 C58 92 64 80 64 62 '
            f'C64 54 62 48 62 40 Z" fill="{BLACK}"/>'
            # belly (front, left side)
            f'<path d="M28 45 C21 53 21 72 26 82 C30 89 41 89 45 84 '
            f'C46 70 46 56 44 47 C40 42 32 42 28 45 Z" fill="{BELLY}"/>'
            # beak pointing left
            f'<path d="M24 34 C16 34 12 37 12 40 C12 43 16 45 24 44 Z" '
            f'fill="{ORANGE}" stroke="{ORANGE_D}" stroke-width="0.8"/>'
            # eye
            f'<ellipse cx="34" cy="30" rx="5.5" ry="7" fill="#fff"/>'
            f'<ellipse cx="31.5" cy="32" rx="2.4" ry="3.2" fill="{BLACK}"/>'
            # near flipper
            f'<path d="M50 46 C56 52 56 68 52 76 C49 80 45 76 45 68 '
            f'C45 60 46 52 47 47 Z" fill="{BLACK}"/>'
            # back foot
            f'<ellipse cx="56" cy="93" rx="11" ry="4.5" fill="{ORANGE_D}"/>')
    return svg(body)

def pose_climber():
    # front-facing, both flippers reaching up the wall, looking up, on tiptoe
    feet = foot(40, cy=97, rx=8, ry=4, rot=-4) + foot(60, cy=97, rx=8, ry=4, rot=4)
    body = front([flipper_up("L"), flipper_up("R")], eyes_look=0.0, feet=feet)
    return svg(body)

def pose_action0():
    # sitting, reading a green book, wizard hat
    hat = (f'<path d="M50 -6 L64 20 L36 20 Z" fill="#26304a"/>'
           f'<ellipse cx="50" cy="20" rx="17" ry="4" fill="#1a2236"/>'
           f'<circle cx="50" cy="-4" r="2.4" fill="{HALO}"/>')
    book = (f'<path d="M28 64 L50 60 L72 64 L72 90 L50 86 L28 90 Z" fill="{GREEN}"/>'
            f'<path d="M50 60 L50 86" stroke="{GREEN_D}" stroke-width="1.6"/>'
            f'<path d="M32 68 L47 65 M32 74 L47 71 M32 80 L47 77" stroke="#eafaf0" stroke-width="0.8"/>'
            f'<path d="M53 65 L68 68 M53 71 L68 74 M53 77 L68 80" stroke="#eafaf0" stroke-width="0.8"/>')
    body = (foot(38, cy=94, rot=-10) + foot(62, cy=94, rot=10) + BODY + BELLY_P +
            eye(43, cy=34, look=-0.5) + eye(57, cy=34, look=-0.5) + beak(cy=44) +
            # arms holding book edges
            flipper_down("L") + flipper_down("R"))
    return svg(hat + body + book)

def pose_splatted():
    # red splat puddle with a few droplets
    splat = (f'<path d="M50 78 C30 78 14 84 16 90 C18 96 40 96 50 95 '
             f'C60 96 82 96 84 90 C86 84 70 78 50 78 Z" fill="{RED}"/>'
             f'<path d="M50 66 C42 66 36 76 40 86 L60 86 C64 76 58 66 50 66 Z" '
             f'fill="{RED}"/>'
             f'<path d="M50 60 L46 82 L54 82 Z" fill="{RED_D}"/>'
             f'<circle cx="22" cy="80" r="3" fill="{RED}"/>'
             f'<circle cx="80" cy="82" r="2.5" fill="{RED}"/>'
             f'<circle cx="30" cy="92" r="1.8" fill="{RED_D}"/>'
             f'<circle cx="72" cy="92" r="1.8" fill="{RED_D}"/>'
             # tiny stunned eyes
             f'<path d="M42 74 l4 4 m0 -4 l-4 4" stroke="#fff" stroke-width="1.4"/>'
             f'<path d="M54 74 l4 4 m0 -4 l-4 4" stroke="#fff" stroke-width="1.4"/>')
    return svg(splat)

POSES = {
    "walker": pose_walker(),
    "faller": pose_faller(),
    "climber": pose_climber(),
    "tumbler": pose_tumbler(),
    "floater": pose_floater(),
    "action0": pose_action0(),
    "angel": pose_angel(),
    "splatted": pose_splatted(),
}

if __name__ == "__main__":
    out = os.path.join(os.path.dirname(__file__), "out")
    os.makedirs(out, exist_ok=True)
    for name, data in POSES.items():
        with open(os.path.join(out, name + ".svg"), "w") as f:
            f.write(data)
        print("wrote", name)
