import "./css/query.css";
import "@/components/global.css";
import Home1 from "@/components/home/home";
import Search_item_link_standard from "@/components/internal_components/search/item/link/search_item_link_standard";
import { search_builder } from "@/library/common/search";
import Link from "next/link";
import Search_Input1 from "@/components/internal_components/search/input/input1";
import Button_with_icon from "@/components/button/image/button_with_icon";
import Bluesky_user_details_row from "@/components/user/user_cards/bluesky_user_details_row";
import { hydrate_users } from "@/library/common/bluesky";
import { useEffect, useRef, useState } from "react";

export default function Query() {
    const should_run = useRef(true);
    const [users, set_users] = useState([]);

    async function get_user() {
        const users_v = await hydrate_users(["jcsalterego.bsky.social"]);
        // alert(JSON.stringify(users_v[0]));
        set_users(users_v)
    }
    useEffect(() => {
        if (should_run.current != true) { return; }
        should_run.current = false;
        
        const params = new URLSearchParams(window.location.search);
        const query = params.get("query");

        document.title = `"${query}" | Lighthouse`;

        get_user();
    });
    const results = [0, 1, 2, 3, 4, 5, 6, 7];
    const results_ul = results.map((data) => {
        return (
            <Search_item_link_standard/>
        )
    });

    const Bluesky_profile_row = ((props) => {
        <div>
            
        </div>
    })

    return (
        <Home1 className="query_search_container">
            <div className="query_search_bar_container">
                {/* <Logo/> */}
                <Search_Input1 className="query_search_bar_input"/>
                <Button_with_icon icon="/icons/filter1.svg"/>
            </div>

            <div className="query_search_results_outer">
                <div className="query_search_results">
                    {/* <p className="do_you_mean greyText">Do you mean <Link href={search_builder("skeet")} className="hover_underline">skeet</Link>, <Link href={search_builder("poasting")} className="hover_underline">poasting</Link>?</p> */}
                    <p className="do_you_mean greyText">Common queries: <Link href={search_builder("beans")} className="hover_underline">beans</Link></p>
                    <div className="query_search_item_results">
                        <div className="outline">
                            <img src="https://bsky.app/static/favicon-32x32.png"/>
                            <p>Bluesky</p>
                            {users[0] != null && <Bluesky_user_details_row user={users[0]} features={["following", "followers"]}/>}
                            <div>
                                <p>labelers aren't enough i need cringe meters</p>
                            </div>
                            <div>
                            <p>sure boss, i'll look into it. after all, we are the jim henson foundation and historical society. just gonna take a hot sip of coffee and see what this bluesky thing is all about</p>
                            </div>
                        </div>
                        {results_ul}
                        {results.length > 0 && <p className="reached_the_end greyText">You've reached the end.</p>}
                    </div>
                </div>
            </div>
        </Home1>
    )
}