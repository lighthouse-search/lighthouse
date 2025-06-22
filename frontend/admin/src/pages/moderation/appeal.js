import "./css/appeal.css";
import Home1 from "@/components/home/home";
import { useEffect, useRef, useState } from "react";
import Search_Input1 from "../../components/internal_components/search/input/input1";
import { Lighthouse } from "@oracularhades/lighthouse";
import { credentials_object } from "@/global";
import { useRouter } from "next/router";
import Button_with_icon from "@/components/button/image/button_with_icon";
import LoadingSpinner from "../../components/miscellaneous/loadingspinner";
import Link from "next/link";

export default function Moderation_appeal() {
    const router = useRouter();
    const should_run = useRef(true);

    const [accounts, set_accounts] = useState([]);

    useEffect(() => {
        if (should_run.current != true) { return; }
        should_run.current = false;
        
        get_account();
    });

    async function get_account() {
       const accounts = await Lighthouse(credentials_object(router)).account.list();
       set_accounts(accounts.data);
    }

    const urls = [
        { url: "https://bsky.app", count: 5054 }
    ]

    const urls_ul = urls.map((url) => {
        return (
            <div className="row space_between">
                <div className="column column_gap_8">
                    <h4>{url.url}</h4>
                    <p className="greyText font_size_14">{url.count} urls</p>
                </div>

                <div className="row column_gap_8">
                    <Link href="#">View</Link>
                </div>
            </div>
        )
    });

    return (
        <Home1 header={["Moderation", "Appeal"]} full_background={true} className="home_padding row_gap_8">
            <Search_Input1/>
            <div className="column scrollY">
                {urls_ul}
            </div>
        </Home1>
    )
}