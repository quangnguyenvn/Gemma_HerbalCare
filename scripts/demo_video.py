r"""
Automated 3-minute OBS demo driver for Gemma HerbalCare.

Usage:
  python scripts/demo_video.py

Optional:
  python scripts/demo_video.py --url https://herbalcare.voidforge.pro/ --intro "D:\git\Gemma HerbalCare — Safety-first Herbal Knowledge Navigator\assets\3339-168930436.mp4"

Before running:
  pip install playwright
  python -m playwright install chromium

Start OBS recording first, then run this script. It opens a maximized headed
browser, plays the intro clip for about 5 seconds, then navigates the public web app.
The matching subtitle file is:
  docs/gemma_herbalcare_3min_demo.srt
"""

from __future__ import annotations

import argparse
import html
import sys
import tempfile
import time
import webbrowser
from pathlib import Path

try:
    from playwright.sync_api import Page, TimeoutError as PlaywrightTimeoutError, sync_playwright
except ModuleNotFoundError:
    print("Playwright is not installed. Run: pip install playwright && python -m playwright install chromium")
    sys.exit(1)


DEFAULT_URL = "https://herbalcare.voidforge.pro/"
DEFAULT_INTRO = r"D:\git\Gemma HerbalCare — Safety-first Herbal Knowledge Navigator\assets\3339-168930436.mp4"
INTRO_SECONDS = 5.0


def pause(seconds: float) -> None:
    time.sleep(seconds)


def caption(text: str) -> None:
    print(f"[caption] {text}", flush=True)


def make_intro_page(video_path: Path) -> Path:
    video_uri = video_path.resolve().as_uri()
    markup = f"""<!doctype html>
<html>
<head>
  <meta charset="utf-8" />
  <title>Gemma HerbalCare intro</title>
  <style>
    html, body {{
      margin: 0;
      width: 100%;
      height: 100%;
      background: #080806;
      overflow: hidden;
    }}
    video {{
      width: 100vw;
      height: 100vh;
      object-fit: cover;
      background: #080806;
    }}
    .label {{
      position: fixed;
      left: 32px;
      bottom: 28px;
      color: #fff8df;
      font: 700 26px Georgia, serif;
      text-shadow: 0 2px 12px rgba(0,0,0,.55);
    }}
  </style>
</head>
<body>
  <video id="intro" src="{html.escape(video_uri)}" autoplay muted playsinline preload="auto"></video>
  <div class="label">Gemma HerbalCare</div>
  <script>
    const video = document.getElementById('intro');
    video.currentTime = 0;
    video.muted = true;
    video.play().catch(() => {{}});
  </script>
</body>
</html>"""
    temp_dir = Path(tempfile.gettempdir()) / "gemma_herbalcare_demo"
    temp_dir.mkdir(parents=True, exist_ok=True)
    intro_html = temp_dir / "intro.html"
    intro_html.write_text(markup, encoding="utf-8")
    return intro_html


def make_fallback_intro_page(reason: str) -> Path:
    markup = f"""<!doctype html>
<html>
<head>
  <meta charset="utf-8" />
  <title>Gemma HerbalCare intro fallback</title>
  <style>
    html, body {{
      margin: 0;
      width: 100%;
      height: 100%;
      background: #0d140b;
      color: #fff8df;
      overflow: hidden;
      font-family: Georgia, "Times New Roman", serif;
    }}
    body {{
      display: grid;
      place-items: center;
    }}
    main {{
      width: min(980px, 78vw);
    }}
    p {{
      color: #e8d9af;
      font-size: 30px;
      line-height: 1.35;
      margin: 0 0 18px;
    }}
    h1 {{
      color: #fff8df;
      font-size: 86px;
      line-height: .95;
      margin: 0 0 24px;
    }}
    .note {{
      color: #8ab66f;
      font-size: 22px;
    }}
  </style>
</head>
<body>
  <main>
    <p>Local-first safety guidance for low-resource communities</p>
    <h1>Gemma HerbalCare</h1>
    <p class="note">{html.escape(reason)}</p>
  </main>
</body>
</html>"""
    temp_dir = Path(tempfile.gettempdir()) / "gemma_herbalcare_demo"
    temp_dir.mkdir(parents=True, exist_ok=True)
    intro_html = temp_dir / "intro_fallback.html"
    intro_html.write_text(markup, encoding="utf-8")
    return intro_html


def request_fullscreen(page: Page) -> None:
    try:
        page.keyboard.press("F11")
        pause(0.8)
    except Exception:
        pass


def play_intro(page: Page, intro: Path | None, seconds: float = INTRO_SECONDS) -> None:
    caption("Intro: local AI can run close to people and help in the field.")
    if intro and intro.exists():
        intro_page = make_intro_page(intro)
        goto(page, intro_page.as_uri())
        try:
            page.locator("#intro").wait_for(state="visible", timeout=5_000)
            page.evaluate(
                """() => {
                  const video = document.querySelector('#intro');
                  video.currentTime = 0;
                  video.muted = true;
                  return video.play().catch(() => null);
                }"""
            )
        except PlaywrightTimeoutError:
            print("[warn] Intro video element was not ready; showing fallback title card.", flush=True)
            goto(page, make_fallback_intro_page("Intro video could not start in the browser.").as_uri())
    else:
        reason = "Intro video missing, so this fallback title card is shown for the same 5-second opening."
        if intro:
            print(f"[warn] Intro video not found: {intro}. Showing fallback intro card.", flush=True)
        goto(page, make_fallback_intro_page(reason).as_uri())
    pause(seconds)


def goto(page: Page, url: str) -> None:
    page.goto(url, wait_until="domcontentloaded", timeout=45_000)
    pause(1.0)


def smooth_scroll(page: Page, y: int, seconds: float = 1.4) -> None:
    page.evaluate(
        """([targetY, duration]) => {
            const startY = window.scrollY;
            const distance = targetY - startY;
            const start = performance.now();
            function step(now) {
              const t = Math.min((now - start) / duration, 1);
              const eased = t < 0.5 ? 2 * t * t : -1 + (4 - 2 * t) * t;
              window.scrollTo(0, startY + distance * eased);
              if (t < 1) requestAnimationFrame(step);
            }
            requestAnimationFrame(step);
        }""",
        [y, seconds * 1000],
    )
    pause(seconds + 0.3)


def safe_click(page: Page, role: str, name: str, timeout: int = 8_000) -> bool:
    try:
        page.get_by_role(role, name=name).click(timeout=timeout)
        return True
    except PlaywrightTimeoutError:
        return False


def safe_fill_label(page: Page, label: str, value: str) -> None:
    field = page.get_by_label(label, exact=True)
    field.fill(value, timeout=8_000)
    pause(0.25)


def set_checkbox_by_label(page: Page, label_text: str, checked: bool) -> None:
    locator = page.get_by_label(label_text, exact=False)
    if checked:
        locator.check(timeout=8_000)
    else:
        locator.uncheck(timeout=8_000)
    pause(0.25)


def choose_age(page: Page, value: str) -> None:
    try:
        page.locator("select").select_option(value, timeout=8_000)
    except PlaywrightTimeoutError:
        page.locator("label", has_text="Age group").locator("select").select_option(value, timeout=8_000)
    pause(0.25)


def submit_and_show_result(page: Page, wait_seconds: float = 8.0) -> None:
    page.get_by_role("button", name="Run safety-first consult").click()
    try:
        page.get_by_text("Gemma response", exact=True).wait_for(timeout=25_000)
    except PlaywrightTimeoutError:
        pause(wait_seconds)
    pause(1.0)
    page.evaluate(
        """() => {
          const panel = document.querySelector('.result-panel');
          if (panel) panel.scrollIntoView({ behavior: 'smooth', block: 'start' });
        }"""
    )
    pause(2.0)


def click_read_aloud(page: Page) -> None:
    try:
        page.get_by_role("button", name="Read response aloud").click(timeout=4_000)
        pause(2.5)
        page.get_by_role("button", name="Stop reading response aloud").click(timeout=4_000)
    except PlaywrightTimeoutError:
        pass


def show_home(page: Page, base_url: str) -> None:
    caption("A safety-first local health assistant for low-resource communities.")
    goto(page, base_url)
    pause(4.0)
    smooth_scroll(page, 430, 1.4)
    pause(4.5)
    smooth_scroll(page, 0, 1.2)
    pause(2.5)


def show_library(page: Page, base_url: str) -> None:
    caption("The herb library uses curated records, images, safety notes, and local availability.")
    goto(page, base_url.rstrip("/") + "/herbs")
    pause(5.0)
    smooth_scroll(page, 520, 1.8)
    pause(5.0)
    smooth_scroll(page, 0, 1.2)
    pause(2.0)


def show_safety(page: Page, base_url: str) -> None:
    caption("The safety page explains the boundary: triage first, no fake cures.")
    goto(page, base_url.rstrip("/") + "/about")
    pause(4.0)
    smooth_scroll(page, 580, 1.5)
    pause(5.0)
    smooth_scroll(page, 0, 1.0)
    pause(1.5)


def show_consult_form(page: Page, base_url: str) -> None:
    caption("The consult form captures context before Gemma writes anything.")
    goto(page, base_url.rstrip("/") + "/consult")
    pause(2.0)
    safe_fill_label(page, "Country", "India")
    safe_fill_label(page, "Region", "Bihar")
    safe_fill_label(page, "City", "Gaya")
    safe_fill_label(
        page,
        "Symptoms or known concern",
        "child diarrhea after unsafe water, tired, thirsty, no blood in stool",
    )
    choose_age(page, "child")
    safe_fill_label(page, "Duration days", "2")
    set_checkbox_by_label(page, "Pregnant", False)
    set_checkbox_by_label(
        page,
        "Clinic, pharmacy, or community health worker reachable within 24 hours",
        False,
    )
    safe_fill_label(page, "Known diseases", "none known")
    safe_fill_label(page, "Current medicines", "none")
    safe_fill_label(page, "Allergies", "none known")
    pause(1.0)
    smooth_scroll(page, 660, 1.4)
    caption("Accessibility matters: camera and microphone are phase 2, read aloud works now.")
    pause(7.0)
    smooth_scroll(page, 930, 1.2)
    submit_and_show_result(page)
    pause(4.0)
    click_read_aloud(page)


def run_case(page: Page, title: str, caption_text: str, result_hold: float) -> None:
    caption(caption_text)
    page.evaluate("window.scrollTo(0, 0)")
    pause(0.8)
    page.get_by_role("button", name=title).click(timeout=10_000)
    pause(1.2)
    submit_and_show_result(page)
    smooth_scroll(page, 720, 1.4)
    pause(result_hold)
    click_read_aloud(page)


def show_demo_cases(page: Page) -> None:
    run_case(
        page,
        "Child diarrhea after unsafe water",
        "Case one: diarrhea prioritizes ORS and safe water before any herb.",
        12.0,
    )
    run_case(
        page,
        "Long-term food source plan",
        "Case two: long-term resilience gives practical food steps, not just herbs.",
        12.0,
    )
    run_case(
        page,
        "Fever and suspected malaria",
        "Case three: suspected malaria pushes testing and proven care urgently.",
        13.0,
    )


def run_demo(url: str, intro: Path | None, headed: bool) -> None:
    with sync_playwright() as p:
        browser = p.chromium.launch(
            headless=not headed,
            args=[
                "--start-maximized",
                "--window-position=0,0",
                "--window-size=1920,1080",
                "--autoplay-policy=no-user-gesture-required",
                "--disable-infobars",
            ],
        )
        context = browser.new_context(no_viewport=True)
        page = context.new_page()

        if headed:
            request_fullscreen(page)

        play_intro(page, intro)

        show_home(page, url)
        show_library(page, url)
        show_safety(page, url)
        show_consult_form(page, url)
        show_demo_cases(page)

        caption("End: a local-first safety layer for real communities.")
        pause(3.0)
        context.close()
        browser.close()


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description="Run the Gemma HerbalCare 3-minute demo.")
    parser.add_argument("--url", default=DEFAULT_URL, help="Public or local app URL.")
    parser.add_argument("--intro", default=DEFAULT_INTRO, help="Intro MP4 path. Use empty string to skip.")
    parser.add_argument("--headless", action="store_true", help="Run without visible browser.")
    parser.add_argument(
        "--open-captions",
        action="store_true",
        help="Open the SRT file in the default app before starting.",
    )
    return parser.parse_args()


def main() -> None:
    args = parse_args()
    caption_file = Path(__file__).resolve().parents[1] / "docs" / "gemma_herbalcare_3min_demo.srt"
    if args.open_captions and caption_file.exists():
        webbrowser.open(caption_file.as_uri())
    intro = Path(args.intro) if args.intro else None
    run_demo(args.url, intro, headed=not args.headless)


if __name__ == "__main__":
    main()
