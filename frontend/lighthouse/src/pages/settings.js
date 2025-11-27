import "./css/settings.css";
import "@/components/global.css";
import Home1 from "@/components/home/home";
import Search_item_link_standard from "@/components/internal_components/search/item/link/search_item_link_standard";
import { useEffect, useRef, useState } from "react";
import { credentials_object } from "@/global";
import { useRouter } from "next/router";
import { Lighthouse } from "@oracularhades/lighthouse";
import ProfilePic from "@/components/user/profile_pic";
import Loading from "@/components/navigating/in-progress/loading";
import Backdrop_content from "@/components/rows/backdrop/backdrop_content";
import Input_with_header from "@/components/input/input_with_header";
import Link from "next/link";
import Switch_with_text from "@/components/switches/frames/rows/switch_with_text";

export default function Settings() {
    const router = useRouter();
    const should_run = useRef(true);
    const [user, set_user] = useState(null);

    async function run() {
        let user = await Lighthouse(credentials_object(router)).account.me();
        user.data.profile_pic = `/profile-pictures/${Math.floor(Math.random() * 5)}.png`;
        set_user(user.data);
    }
    useEffect(() => {
        if (should_run.current != true) { return; }
        should_run.current = false;

        run();
    });

    useEffect(() => {
        document.title = "Settings";
    });

    if (user == null) {
        return (
            <Home1 className="query_search_container">
                <div className="settings">
                    <Loading/>
                </div>
            </Home1>
        )
    }

    return (
        <Home1 className="padding_16">
            {/* <h1>Settings</h1> */}
            <div className="settings column row_gap_8">
                <div className="row column_gap_10">
                    <ProfilePic src={null} check={true} style={{ width: 60, height: 60 }}/>
                    <div className="column">
                        <h2>{user.name}</h2>
                        <p className="greyText">{user.email}</p>
                    </div>
                </div>

                <Backdrop_content header="Account" className="column row_gap_8">
                    <Input_with_header header="Name" placeholder="e.g. John Doe"/>
                    <Input_with_header header="Email">
                        <div className="row column_gap_8">
                            <p>{user.email}</p>
                            <Link href="#" className="font_size_14 hover_underline">Change</Link>
                        </div>
                    </Input_with_header>
                </Backdrop_content>

                <Backdrop_content header="Search" className="column row_gap_8">
                    <Switch_with_text header="Lighthouse Safe Search" description="This server supports searching content across AT-Protocol enabled websites (e.g. Bluesky). When enabled, ATProto accounts and posts will have enhanced embeds in search results.">
                        <select>
                            <option>Yes</option>
                            <option>No</option>
                        </select>
                    </Switch_with_text>

                    <Switch_with_text header="Include AI generated answers" description="This server supports LLM (AI) generated results from ChatGPT, Bing, Claude, HuggingFace or other LLMs.">
                        <select>
                            <option>Yes</option>
                            <option>Ask</option>
                            <option>No</option>
                        </select>
                    </Switch_with_text>

                    <Switch_with_text header="Include AT-Protocol (Bluesky) results" description="This server supports searching content across AT-Protocol enabled websites (e.g. Bluesky). When enabled, ATProto accounts and posts will have enhanced embeds in search results.">
                        <select>
                            <option>Yes</option>
                            <option>No</option>
                        </select>
                    </Switch_with_text>
                </Backdrop_content>

                <Backdrop_content header="Privacy" className="column row_gap_8">
                    <Switch_with_text header="Location-based results" description="When enabled, IP-Address geolocation data will be used to return relevant results for your location.">
                        <select>
                            <option>Yes</option>
                            <option>Ask</option>
                            <option value="not_saved">No</option>
                        </select>
                    </Switch_with_text>
                </Backdrop_content>

                <div className="column">
                    <p className="greyText font_size_12">This server is powered by <Link href="https://example.com">Lighthouse</Link>. Made in Aotearoa New Zealand.</p>
                    <p className="greyText font_size_12 row column_gap_4">
                        This server is not operated or affiliated with the Lighthouse project. Server:
                        <div className="row column_gap_8">
                            <Link href="/terms">Terms of service</Link>
                            <Link href="/privacy">Privacy policy</Link>
                        </div>
                    </p>
                </div>
            </div>
        </Home1>
    )
}