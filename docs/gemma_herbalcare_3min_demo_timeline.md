# Gemma HerbalCare 3-Minute Demo Timeline

Use `scripts/demo_video.py` for screen automation and
`docs/gemma_herbalcare_3min_demo.srt` for subtitles or voice timing.

## Recording Flow

1. Start OBS recording.
2. Run:

```powershell
python scripts\demo_video.py
```

3. The script plays `C:\Downloads\3339-168930436.mp4` for about 5 seconds.
4. It opens `https://herbalcare.voidforge.pro/`.
5. It demos the home page, herb library, safety page, consult form, accessibility area, result panel, read-aloud control, and three demo cases.
6. Stop OBS and trim the first seconds in CapCut if needed.

## Voiceover Notes

- Speak slowly. The judges need to understand the product in one pass.
- Emphasize that this is not a diagnosis app.
- Emphasize that camera and microphone are phase 2, while read aloud works now.
- Emphasize three levels of guidance:
  - Diarrhea: ORS and safe water first, herbs supportive only.
  - Food plan: practical resilience, food crops, and chickens where realistic.
  - Malaria: urgent testing and proven medicine, no herbal substitution.

## Local Setup

Install Playwright once:

```powershell
pip install playwright
python -m playwright install chromium
```

Optional custom URL:

```powershell
python scripts\demo_video.py --url http://127.0.0.1:5174/
```

Optional skip intro:

```powershell
python scripts\demo_video.py --intro ""
```
