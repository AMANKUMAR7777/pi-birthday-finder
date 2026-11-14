#!/usr/bin/env python3
"""
Pi Cosmic Observatory - Native CustomTkinter GUI (Alternative Native Python UI)
"""

import os
import sys
import threading
import requests
import customtkinter as ctk

SCRIPT_DIR = os.path.dirname(os.path.abspath(__file__))
PI_10K_FILE = os.path.join(SCRIPT_DIR, "gui", "pi_10k.txt")

LOCAL_PI_DIGITS = ""
if os.path.exists(PI_10K_FILE):
    with open(PI_10K_FILE, "r") as f:
        LOCAL_PI_DIGITS = f.read().strip()

ctk.set_appearance_mode("dark")
ctk.set_default_color_theme("blue")

class PiNativeApp(ctk.CTk):
    def __init__(self):
        super().__init__()

        self.title("Pi Cosmic Observatory - Native Python GUI")
        self.geometry("860x700")
        self.minsize(700, 560)
        self.configure(fg_color="#090b11")

        self.format_var = ctk.StringVar(value="DDMMYYYY")
        self.last_result = None

        self._build_ui()

    def _build_ui(self):
        # Header
        header = ctk.CTkFrame(self, fg_color="#0d101a", corner_radius=10, border_width=1, border_color="#1f273d")
        header.pack(fill="x", padx=16, pady=12)

        icon_lbl = ctk.CTkLabel(header, text=" π ", font=ctk.CTkFont(size=24, weight="bold"), fg_color="#1e1b4b", text_color="#a5b4fc", corner_radius=8)
        icon_lbl.pack(side="left", padx=12, pady=10)

        title_box = ctk.CTkFrame(header, fg_color="transparent")
        title_box.pack(side="left", padx=6, pady=8)
        ctk.CTkLabel(title_box, text="PI COSMIC OBSERVATORY", font=ctk.CTkFont(size=16, weight="bold"), text_color="#f8fafc").pack(anchor="w")
        ctk.CTkLabel(title_box, text="Date Coordinates in the Infinite Transcendental Stream", font=ctk.CTkFont(size=11), text_color="#94a3b8").pack(anchor="w")

        # Tabs
        self.tabview = ctk.CTkTabview(self, fg_color="#121624", border_width=1, border_color="#1f273d", segmented_button_selected_color="#4f46e5")
        self.tabview.pack(fill="both", expand=True, padx=16, pady=(0, 16))

        self.tab_search = self.tabview.add("Explorer")
        self.tab_stream = self.tabview.add("Live Streamer")
        self.tab_cert = self.tabview.add("Certificate")

        self._build_search_tab()
        self._build_stream_tab()
        self._build_cert_tab()

    def _build_search_tab(self):
        frame = self.tab_search

        # Input Card
        input_card = ctk.CTkFrame(frame, fg_color="#161b2c", border_width=1, border_color="#232d44", corner_radius=10)
        input_card.pack(fill="x", padx=12, pady=10)

        ctk.CTkLabel(input_card, text="TARGET CALENDAR DATE", font=ctk.CTkFont(size=12, weight="bold"), text_color="#818cf8").pack(anchor="w", padx=16, pady=(12, 6))

        row1 = ctk.CTkFrame(input_card, fg_color="transparent")
        row1.pack(fill="x", padx=16, pady=4)

        ctk.CTkLabel(row1, text="Day:", text_color="#94a3b8").pack(side="left", padx=(0, 4))
        self.entry_day = ctk.CTkEntry(row1, width=60, font=ctk.CTkFont(family="monospace", size=14))
        self.entry_day.insert(0, "14")
        self.entry_day.pack(side="left", padx=(0, 14))

        ctk.CTkLabel(row1, text="Month:", text_color="#94a3b8").pack(side="left", padx=(0, 4))
        self.entry_month = ctk.CTkEntry(row1, width=60, font=ctk.CTkFont(family="monospace", size=14))
        self.entry_month.insert(0, "03")
        self.entry_month.pack(side="left", padx=(0, 14))

        ctk.CTkLabel(row1, text="Year:", text_color="#94a3b8").pack(side="left", padx=(0, 4))
        self.entry_year = ctk.CTkEntry(row1, width=80, font=ctk.CTkFont(family="monospace", size=14))
        self.entry_year.insert(0, "1879")
        self.entry_year.pack(side="left", padx=(0, 14))

        # Format
        ctk.CTkLabel(row1, text="Format:", text_color="#94a3b8").pack(side="left", padx=(10, 4))
        self.seg_format = ctk.CTkSegmentedButton(row1, values=["DDMMYYYY", "MMDDYYYY", "YYYYMMDD", "DDMMYY", "DDMM"], variable=self.format_var, command=self._update_target)
        self.seg_format.pack(side="left")

        # Presets
        presets_row = ctk.CTkFrame(input_card, fg_color="transparent")
        presets_row.pack(fill="x", padx=16, pady=8)
        ctk.CTkLabel(presets_row, text="Milestones:", text_color="#64748b", font=ctk.CTkFont(size=11)).pack(side="left", padx=(0, 8))

        ctk.CTkButton(presets_row, text="Einstein (Pi Day 1879)", width=120, height=24, fg_color="#1e2338", text_color="#c7d2fe", command=lambda: self._set_preset(14, 3, 1879)).pack(side="left", padx=4)
        ctk.CTkButton(presets_row, text="Moon Landing (1969)", width=120, height=24, fg_color="#1e2338", text_color="#c7d2fe", command=lambda: self._set_preset(20, 7, 1969)).pack(side="left", padx=4)
        ctk.CTkButton(presets_row, text="Web Invented (1989)", width=120, height=24, fg_color="#1e2338", text_color="#c7d2fe", command=lambda: self._set_preset(12, 3, 1989)).pack(side="left", padx=4)

        # Action bar
        act_bar = ctk.CTkFrame(input_card, fg_color="#0a0d16", corner_radius=8, border_width=1, border_color="#1f273d")
        act_bar.pack(fill="x", padx=16, pady=(6, 16))

        ctk.CTkLabel(act_bar, text="Target Query: ", text_color="#94a3b8").pack(side="left", padx=(12, 4), pady=10)
        self.lbl_target = ctk.CTkLabel(act_bar, text="14031879", font=ctk.CTkFont(family="monospace", size=20, weight="bold"), text_color="#fbbf24")
        self.lbl_target.pack(side="left", pady=10)

        self.btn_search = ctk.CTkButton(act_bar, text="SCAN PI STREAM", font=ctk.CTkFont(size=13, weight="bold"), fg_color="#4f46e5", hover_color="#4338ca", command=self._start_search)
        self.btn_search.pack(side="right", padx=12, pady=8)

        # Results Frame
        self.res_card = ctk.CTkFrame(frame, fg_color="#0c171e", border_width=1.5, border_color="#10b981", corner_radius=10)
        self.res_card.pack(fill="both", expand=True, padx=12, pady=(10, 10))

        ctk.CTkLabel(self.res_card, text="COSMIC COORDINATES RESOLVED", font=ctk.CTkFont(size=13, weight="bold"), text_color="#34d399").pack(anchor="w", padx=16, pady=(12, 4))
        self.lbl_pos = ctk.CTkLabel(self.res_card, text="Position: #11,885,022", font=ctk.CTkFont(family="monospace", size=26, weight="bold"), text_color="#fbbf24")
        self.lbl_pos.pack(anchor="w", padx=16, pady=4)

        self.lbl_tape = ctk.CTkLabel(self.res_card, text="...984510382947 [ 14031879 ] 492048591823...", font=ctk.CTkFont(family="monospace", size=15), text_color="#94a3b8")
        self.lbl_tape.pack(anchor="w", padx=16, pady=8)

    def _build_stream_tab(self):
        f = self.tab_stream
        ctk.CTkLabel(f, text="LIVE DIGIT STREAM TELEMETRY", font=ctk.CTkFont(size=13, weight="bold"), text_color="#818cf8").pack(anchor="w", padx=12, pady=(10, 4))

        self.stream_box = ctk.CTkTextbox(f, font=ctk.CTkFont(family="monospace", size=14), fg_color="#06080e", text_color="#a5b4fc", border_width=1, border_color="#1f273d")
        self.stream_box.pack(fill="both", expand=True, padx=12, pady=10)

        if LOCAL_PI_DIGITS:
            self.stream_box.insert("1.0", "3." + LOCAL_PI_DIGITS[:4000])

    def _build_cert_tab(self):
        f = self.tab_cert
        self.cert_box = ctk.CTkTextbox(f, font=ctk.CTkFont(family="monospace", size=12), fg_color="#06080e", text_color="#fef08a", border_width=1.5, border_color="#fbbf24")
        self.cert_box.pack(fill="both", expand=True, padx=12, pady=12)
        self._update_cert("14031879", "11,885,022")

    def _set_preset(self, d, m, y):
        self.entry_day.delete(0, "end"); self.entry_day.insert(0, str(d))
        self.entry_month.delete(0, "end"); self.entry_month.insert(0, str(m))
        self.entry_year.delete(0, "end"); self.entry_year.insert(0, str(y))
        self._update_target()

    def _get_target(self):
        d = str(int(self.entry_day.get() or 1)).zfill(2)
        m = str(int(self.entry_month.get() or 1)).zfill(2)
        y = str(int(self.entry_year.get() or 2000)).zfill(4)
        fmt = self.format_var.get()
        if fmt == "DDMMYYYY": return d + m + y
        if fmt == "MMDDYYYY": return m + d + y
        if fmt == "YYYYMMDD": return y + m + d
        if fmt == "DDMMYY": return d + m + y[-2:]
        return d + m

    def _update_target(self, *args):
        self.lbl_target.configure(text=self._get_target())

    def _update_cert(self, target, pos):
        text = f"""
+=============================================================================+
|                      CERTIFICATE OF EXISTENCE IN PI                         |
+=============================================================================+
|                                                                             |
|   Target Date / Coordinates:                                                |
|   ★  {target}  ★                                                       |
|                                                                             |
|   Cosmic Transcendental Decimal Offset Position:                            |
|   👉  #{pos}                                                      |
|                                                                             |
|   Mathematical Note:                                                        |
|   Because π is transcendental and conjectured to be normal in base 10,       |
|   every finite permutation of numbers exists infinitely across eternity.    |
|                                                                             |
+=============================================================================+
"""
        self.cert_box.delete("1.0", "end")
        self.cert_box.insert("1.0", text.strip())

    def _start_search(self):
        target = self._get_target()
        self.btn_search.configure(text="SCANNING...", state="disabled")

        def worker():
            try:
                # 1. Local check
                if LOCAL_PI_DIGITS:
                    pos = LOCAL_PI_DIGITS.find(target)
                    if pos != -1:
                        self.after(0, lambda: self._on_found(target, pos + 1, "Local 10k Cache", LOCAL_PI_DIGITS[max(0, pos-12):pos], LOCAL_PI_DIGITS[pos+len(target):pos+len(target)+12]))
                        return

                # 2. Online 200M check
                r = requests.post("https://www.angio.net/newpi/piquery", data={"q": target}, timeout=8).json()
                if r.get("r") and r["r"][0].get("p"):
                    it = r["r"][0]
                    self.after(0, lambda: self._on_found(target, it["p"], "200M Index", it.get("db", "..."), it.get("da", "...")))
                else:
                    self.after(0, lambda: self._on_not_found())
            except Exception as e:
                self.after(0, lambda: self._on_not_found(str(e)))

        threading.Thread(target=worker, daemon=True).start()

    def _on_found(self, target, pos, src, before, after):
        self.btn_search.configure(text="SCAN PI STREAM", state="normal")
        self.lbl_pos.configure(text=f"Position: #{pos:,}")
        self.lbl_tape.configure(text=f"...{before} [ {target} ] {after}...")
        self._update_cert(target, f"{pos:,}")

    def _on_not_found(self, err=None):
        self.btn_search.configure(text="SCAN PI STREAM", state="normal")
        self.lbl_pos.configure(text="Not found in range")
        self.lbl_tape.configure(text=err or "Try a shorter date format (e.g. DDMM)")

def main():
    app = PiNativeApp()
    app.mainloop()

if __name__ == "__main__":
    main()
