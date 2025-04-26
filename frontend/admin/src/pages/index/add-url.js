import "./css/add-url.css";
import Home1 from "@/components/home/home";
import { useEffect, useRef, useState } from "react";
import Search_Input1 from "../../components/internal_components/search/input/input1";
import { Lighthouse } from "@oracularhades/lighthouse";
import { credentials_object } from "@/global";
import { useRouter } from "next/router";
import UserCard1 from "@/components/user/user_cards/user_card1";
import Button_with_icon from "@/components/button/image/button_with_icon";

export default function Search_AddUrl() {
    const router = useRouter();
    const should_run = useRef(true);
    const [urls_text, set_urls_text] = useState("");

    useEffect(() => {
        if (should_run.current != true) { return; }
        should_run.current = false;
    });

    async function add_urls(url) {
       const urls = await Lighthouse(await credentials_object(router)).admin.index.job.update({
            actions: [
                { action: "create", url: url }
            ] 
        });
    }

    return (
        <Home1 header={["Index", "Add URL"]} full_background={true} className="add_url home_padding row_gap_8">
            <p>Paste URL</p>
            <textarea value={urls_text} onChange={(e) => { set_urls_text(e.target.value); }} className="add_url_input" placeholder={`https://bsky.app/
https://bsky.app/profile/why.bsky.team
https://bsky.app/profile/blowdart.me/post/3ldx57pwvps25
https://cdn.bsky.app/img/feed_fullsize/plain/did:plc:hfgp6pj3akhqxntgqwramlbg/bafkreif2hsc7z6ii3jzvd2e3ytvx3i47qvlskpfu6quvzg6jxihakcpqdy@jpeg`}></textarea>
            <button onClick={() => { add_urls(urls_text.split("\n").filter((value) => { if (value == null || value.trim().length == 0) { return false; } else { return true; } })); }}>Start index</button>
        </Home1>
    )
}