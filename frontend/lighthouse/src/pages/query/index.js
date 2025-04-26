import "./css/query.css";
import "@/components/global.css";
import "@/../styles/global.css";
import Home1 from "@/components/home/home";
import Search_item_link_standard from "@/components/internal_components/search/item/link/search_item_link_standard";
import { search_builder } from "@/library/common/search";
import Link from "next/link";
import Search_Input1 from "@/components/internal_components/search/input/input1";
import Button_with_icon from "@/components/button/image/button_with_icon";
import Bluesky_user_details_row from "@/components/internal_components/user_cards/bluesky_user_details_row";
import { hydrate_users } from "@/library/common/bluesky";
import { useEffect, useRef, useState } from "react";
import { Lighthouse } from "@oracularhades/lighthouse";
import { credentials_object } from "@/global";
import { useRouter } from "next/router";
import Loading from "@/components/navigating/in-progress/loading";
import Illegal_content from "./display/illegal-content";
import Backdrop_content from "@/components2/rows/backdrop/backdrop_content";

export default function Query() {
    const router = useRouter();
    const should_run = useRef(true);
    const [results, set_results] = useState(null);
    const [users, set_users] = useState([]);
    const [error, set_error] = useState(null);
    const [is_content_blocked, set_is_content_blocked] = useState(false);

    async function get_user() {
        const users_v = await hydrate_users(["jcsalterego.bsky.social"]);
        // alert(JSON.stringify(users_v[0]));
        set_users(users_v)
    }

    async function content_blocked() {
        set_is_content_blocked(true);
    }

    async function run(query) {
        set_results(null);
        try {
            // await content_blocked();
            // return;
            const query_data = await Lighthouse(await credentials_object(router)).query.list(query);
            set_results(query_data.data);
            set_error(null);
        } catch (error) {
            set_error(error);
        }
    }

    function get_query() {
        const params = new URLSearchParams(window.location.search);
        const query = params.get("query");

        return query;
    }

    useEffect(() => {
        const query = get_query();
        if (should_run.current == query) { return; }
        should_run.current = query;

        document.title = `"${query}" | Lighthouse`;

        run(query);
        get_user();
    });

    const Search_base = ((props) => {
        return (
            <Home1 className="query_search_container">
                <div className="search_row row align_items_unset column_gap_10">
                    <div className="query_search_results_outer row_gap_6">
                        <div className="query_search_bar_container">
                            {/* <Logo/> */}
                            <Search_Input1 on_search={(value) => { run(value); }} className="query_search_bar_input"/>
                            <Button_with_icon icon="/icons/filter1.svg"/>
                        </div>

                        <div className="query_search_results row_gap_6">
                            {props.children}
                        </div>
                    </div>

                    <div className="column row_gap_6">
                        {props.right}
                    </div>
                </div>
            </Home1>
        )
    });

    if (error) {
        return (
            <Search_base>
                <div className="row_gap_2">
                    <h2>Something went wrong</h2>
                    <p><span className="greyText">{error.message}</span></p>
                </div>
            </Search_base>
        )
    }
    
    if (is_content_blocked == true) {
        return (
            <Illegal_content/>
        )
    }

    if (results == null) {
        return (
            <Search_base>
                <Loading/>
            </Search_base>
        )
    }

    if (results.length == 0) {
        return (
            <Search_base>
                <div className="row_gap_2">
                    <h2>404</h2>
                    <p>Sorry, we couldn't find any results.</p>

                    {/* <p>We searched under the bed,<br/>
 we searched under the chair,<br/>
we looked all around, but<br/>
it's just not there.</p><br/> */}
                </div>
            </Search_base>
        )
    }

    const results_ul = results.map((data) => {
        return (
            <Search_item_link_standard data={data}/>
        )
    });

    const Bluesky_profile_row = ((props) => {
        <div>
            
        </div>
    });

    const Right = ((props) => {
        return (
            <div className="column row_gap_8">
                <div className="query_profile column outline">
                    <div className="images row align_items_unset column_gap_4 scrollX">
                        <img src="https://upload.wikimedia.org/wikipedia/commons/thumb/3/32/Pulitzer2018-portraits-kendrick-lamar.jpg/1024px-Pulitzer2018-portraits-kendrick-lamar.jpg"/>
                        <img src="https://imgs.search.brave.com/86p6yc23N7UWsof17XgH_ecFV0bpYOw7dZX6wH0mbxY/rs:fit:860:0:0:0/g:ce/aHR0cHM6Ly9zMS50/aWNrZXRtLm5ldC9k/YW0vYS85ZjMvOWIy/OGU2M2EtNDY3Yi00/YmNjLWI4NmUtMDk5/MDM3Mjc4OWYzXzE2/OTUxNDFfUkVUSU5B/X1BPUlRSQUlUXzNf/Mi5qcGc"/>
                        <img src="https://upload.wikimedia.org/wikipedia/commons/thumb/3/32/Pulitzer2018-portraits-kendrick-lamar.jpg/1024px-Pulitzer2018-portraits-kendrick-lamar.jpg"/>
                        <img src="https://upload.wikimedia.org/wikipedia/commons/thumb/3/32/Pulitzer2018-portraits-kendrick-lamar.jpg/1024px-Pulitzer2018-portraits-kendrick-lamar.jpg"/>
                    </div>
                    <div className="information column row_gap_6">
                        <div className="column row_gap_6">
                            <h2>Kendrick Lamar</h2>
                            <p className="description font_size_14">Kendrick Lamar Duckworth is an American rapper. Regarded as one of the most influential hip-hop artists of his generation, and one of the greatest rappers of all time, he was awarded the 2018 Pulitzer Prize for Music, becoming the first musician outside of the classical and jazz genres to receive the honor. -- <Link href="https://example.com">Wikipedia</Link></p>
                        </div>
                        
                        <p className="font_size_14"><b>Born</b>	Kendrick Lamar Duckworth June 17, 1987 (age 37) Compton, California, U.S.</p>
                    </div>
                </div>

                <Link href="#" className="font_size_12 greyText">Search transparency</Link>
            </div>
        )
    });

    return (
        <Search_base right={<Right/>}>
            {/* <p className="do_you_mean greyText">Do you mean <Link href={search_builder("skeet")} className="hover_underline">skeet</Link>, <Link href={search_builder("poasting")} className="hover_underline">poasting</Link>?</p> */}
            <p className="do_you_mean greyText">Suggested: <Link href={search_builder("beans")} className="hover_underline">beans</Link></p>
            {/* <p className="do_you_mean greyText">Took: </p> */}
            <div className="column row_gap_6">
                <Backdrop_content header="AI Answer" description={<p>Generated by ChatGPT. Answers may be incorect.</p>}>
                    <div className="typewriter column row_gap_4">
                        <p>In the U.S., fall (autumn) 2025 begins on Monday, September 22, 2025, and ends on Sunday, December 21, 2025, based on the astronomical seasons.

However, meteorological fall (used for weather and climate purposes) runs from September 1 to November 30 each year.</p>
                        <div className="row column_gap_4">
                            <input className="width_100" placeholder="Start typing..." />
                            <Button_with_icon icon="/icons/file-regular.svg"/>
                            <Button_with_icon style={{ padding: 4 }} icon_style={{ width: 28, height: 28 }} icon="/icons/send-outline.svg"/>
                        </div>
                    </div>
                </Backdrop_content>
                {/* <div className="outline">
                    <div className="row column_gap_4">
                        <img src="https://bsky.app/static/favicon-32x32.png"/>
                        <p>Bluesky</p>
                    </div>
                    {users[0] != null && <Bluesky_user_details_row user={users[0]} features={["following", "followers"]}/>}
                    <div>
                        <p>labelers aren't enough i need cringe meters</p>
                    </div>
                    <div>
                    <p>sure boss, i'll look into it. after all, we are the jim henson foundation and historical society. just gonna take a hot sip of coffee and see what this bluesky thing is all about</p>
                    </div>
                </div> */}
                {results_ul}
                {/* {results.length > 0 && <p className="reached_the_end greyText">You've reached the end.</p>} */}
            </div>
        </Search_base>
    )
}