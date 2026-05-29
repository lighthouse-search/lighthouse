"use client";

import { useEffect, useState } from "react";
import Dialog_Frame from "@/components/dialogs/dialog_frame";
import Input_with_header from "@/components/input/input_with_header";
import { DIALOG_ID, open_developer_settings } from "./open_developer_settings";
import "./css/developer_settings_dialog.css";

// The library (`@oracularhades/lighthouse` routing) reads this key at request
// time, so changing it repoints every API call. Unset = use the origin the
// dashboard is served from.
const STORAGE_KEY = "custom_api";

export default function Developer_settings_dialog() {
    const [endpoint, set_endpoint] = useState("");

    useEffect(() => {
        if (typeof localStorage != "undefined") {
            set_endpoint(localStorage.getItem(STORAGE_KEY) || "");
        }

        // Cmd/Ctrl+Shift+E opens developer settings from any screen, so the
        // endpoint stays changeable even when a page fails to load its data.
        function on_keydown(e) {
            if ((e.metaKey || e.ctrlKey) && e.shiftKey && (e.key == "E" || e.key == "e")) {
                e.preventDefault();
                open_developer_settings();
            }
        }
        document.addEventListener("keydown", on_keydown);
        return () => document.removeEventListener("keydown", on_keydown);
    }, []);

    function save() {
        const value = endpoint.trim();
        try {
            if (value == "") {
                localStorage.removeItem(STORAGE_KEY);
            } else {
                // Accept an origin ("http://localhost:4459") or a full base, and
                // store the normalized API base the library expects.
                localStorage.setItem(STORAGE_KEY, new URL("/api/native-v1", value).href);
            }
        } catch (e) {
            alert(`Invalid URL: ${e.message}`);
            return;
        }
        // Reload so already-mounted components refetch against the new endpoint.
        window.location.reload();
    }

    function reset() {
        if (typeof localStorage != "undefined") { localStorage.removeItem(STORAGE_KEY); }
        set_endpoint("");
        window.location.reload();
    }

    return (
        <Dialog_Frame id={DIALOG_ID} header="Developer settings" className="developer_settings_dialog">
            <div className="column row_gap_8">
                <Input_with_header header="Custom API endpoint" placeholder="e.g. http://localhost:4459" value={endpoint} onChange={e => set_endpoint(e.target.value)}/>
                <p className="greyText font_size_12 developer_settings_dialog_hint">Overrides the API origin this dashboard talks to. Leave blank to use this site's origin. (Open anytime with Cmd/Ctrl+Shift+E.)</p>
                <div className="row column_gap_8 developer_settings_dialog_actions">
                    <button className="developer_settings_dialog_save" onClick={save}>Save &amp; reload</button>
                    <button className="developer_settings_dialog_reset" onClick={reset}>Reset</button>
                </div>
            </div>
        </Dialog_Frame>
    )
}
