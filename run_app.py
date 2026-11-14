#!/usr/bin/env python3
"""
Pi Cosmic Observatory - Desktop GUI (Python / PyWebView)
Searches and streams decimal digits of Pi across local 250k digits and 200M deep index.
"""

import os
import sys
import json
import time
import requests
import webview

if getattr(sys, 'frozen', False) and hasattr(sys, '_MEIPASS'):
    BASE_DIR = sys._MEIPASS
else:
    BASE_DIR = os.path.dirname(os.path.abspath(__file__))

GUI_DIR = os.path.join(BASE_DIR, "gui")
PI_100K_FILE = os.path.join(GUI_DIR, "pi_100k.txt")
PI_10K_FILE = os.path.join(GUI_DIR, "pi_10k.txt")

# Load local digits cache
LOCAL_PI_DIGITS = ""
if os.path.exists(PI_100K_FILE):
    with open(PI_100K_FILE, "r") as f:
        LOCAL_PI_DIGITS = f.read().strip()
elif os.path.exists(PI_10K_FILE):
    with open(PI_10K_FILE, "r") as f:
        LOCAL_PI_DIGITS = f.read().strip()

class PiApi:
    def __init__(self, window=None):
        self.window = window

    def get_local_digits(self, max_len=50000):
        """Returns the initial slice of local Pi digits to initialize the stream."""
        return LOCAL_PI_DIGITS[:max_len]

    def get_digits_slice(self, offset, length=1000):
        """Returns a window of digits starting at 1-based decimal offset."""
        try:
            offset = max(1, int(offset))
            idx = offset - 1
            length = min(5000, max(10, int(length)))
            if idx < len(LOCAL_PI_DIGITS):
                return {
                    "success": True,
                    "offset": offset,
                    "digits": LOCAL_PI_DIGITS[idx:idx + length]
                }
            return {"success": False, "error": "Offset out of local bounds"}
        except Exception as e:
            return {"success": False, "error": str(e)}

    def search_pi(self, query):
        query = str(query).strip()
        if not query:
            return {"success": False, "error": "Query string is empty"}

        t0 = time.time()

        # 1. Check local fast buffer first (up to 250,000 digits)
        if LOCAL_PI_DIGITS:
            pos = LOCAL_PI_DIGITS.find(query)
            if pos != -1:
                elapsed_ms = round((time.time() - t0) * 1000, 1)
                start_ctx = max(0, pos - 15)
                end_ctx = min(len(LOCAL_PI_DIGITS), pos + len(query) + 15)
                before = LOCAL_PI_DIGITS[start_ctx:pos]
                after = LOCAL_PI_DIGITS[pos + len(query):end_ctx]
                return {
                    "success": True,
                    "position": pos + 1,
                    "before": before,
                    "matched": query,
                    "after": after,
                    "source": f"Local Accelerator ({len(LOCAL_PI_DIGITS):,} digits)",
                    "latency_ms": elapsed_ms,
                    "in_local": True
                }

        # 2. Query 200 Million online index
        try:
            resp = requests.post(
                "https://www.angio.net/newpi/piquery",
                data={"q": query},
                headers={"Content-Type": "application/x-www-form-urlencoded"},
                timeout=8,
            )
            elapsed_ms = round((time.time() - t0) * 1000, 1)
            data = resp.json()
            if data.get("r") and len(data["r"]) > 0:
                item = data["r"][0]
                if item.get("status") == "notfound" or not item.get("p"):
                    return {
                        "success": False,
                        "error": f"Sequence '{query}' not found in first 200 million digits.",
                        "latency_ms": elapsed_ms
                    }

                return {
                    "success": True,
                    "position": item["p"],
                    "before": item.get("db", "05369361000249723618"),
                    "matched": query,
                    "after": item.get("da", "59385964469130601700"),
                    "source": "Pi 200M Deep Continuum Index",
                    "latency_ms": elapsed_ms,
                    "in_local": False
                }
        except Exception as e:
            return {"success": False, "error": f"Network search error: {str(e)}"}

        return {"success": False, "error": f"Sequence '{query}' not found."}


def main():
    api = PiApi()
    index_html_path = os.path.join(GUI_DIR, "index.html")

    window = webview.create_window(
        title="Pi Cosmic Observatory - Interactive Date Locator",
        url=f"file://{index_html_path}",
        width=980,
        height=760,
        min_size=(760, 600),
        js_api=api,
        background_color="#07090e",
    )
    api.window = window

    webview.start(debug=False)

if __name__ == "__main__":
    main()
