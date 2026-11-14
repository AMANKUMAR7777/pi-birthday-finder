#!/usr/bin/env python3
"""
Pi Cosmic Observatory - Modern Desktop GUI (Python / PyWebView)
Searches and streams decimal digits of Pi across a 200M digit index.
"""

import os
import sys
import json
import time
import requests
import webview

SCRIPT_DIR = os.path.dirname(os.path.abspath(__file__))
GUI_DIR = os.path.join(SCRIPT_DIR, "gui")
PI_10K_FILE = os.path.join(GUI_DIR, "pi_10k.txt")

# Load local 10k digits cache if available
LOCAL_PI_DIGITS = ""
if os.path.exists(PI_10K_FILE):
    with open(PI_10K_FILE, "r") as f:
        LOCAL_PI_DIGITS = f.read().strip()

class PiApi:
    def __init__(self, window=None):
        self.window = window

    def search_pi(self, query):
        query = str(query).strip()
        if not query:
            return {"success": False, "error": "Query string is empty"}

        # 1. Check local offline cache first
        if LOCAL_PI_DIGITS:
            pos = LOCAL_PI_DIGITS.find(query)
            if pos != -1:
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
                    "source": "Local Fast Cache (10k Digits)",
                }

        # 2. Query 200 Million online index
        try:
            resp = requests.post(
                "https://www.angio.net/newpi/piquery",
                data={"q": query},
                headers={"Content-Type": "application/x-www-form-urlencoded"},
                timeout=8,
            )
            data = resp.json()
            if data.get("r") and len(data["r"]) > 0:
                item = data["r"][0]
                if item.get("status") == "notfound" or not item.get("p"):
                    return {"success": False, "error": f"Sequence '{query}' not found in first 200 million digits."}

                return {
                    "success": True,
                    "position": item["p"],
                    "before": item.get("db", "05369361000249723618"),
                    "matched": query,
                    "after": item.get("da", "59385964469130601700"),
                    "source": "Pi 200M Deep Continuum Index",
                }
        except Exception as e:
            return {"success": False, "error": f"Network search error: {str(e)}"}

        return {"success": False, "error": f"Sequence '{query}' not found."}


def main():
    api = PiApi()
    index_html_path = os.path.join(GUI_DIR, "index.html")

    window = webview.create_window(
        title="Pi Cosmic Observatory - Date of Birth Finder",
        url=f"file://{index_html_path}",
        width=880,
        height=720,
        min_size=(700, 560),
        js_api=api,
        background_color="#090b11",
    )
    api.window = window

    webview.start(debug=False)

if __name__ == "__main__":
    main()
