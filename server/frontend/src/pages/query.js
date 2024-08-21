import "../../styles/query.css";
import "@/components/global.css";
import Logo from "@/components/internal_components/logo/logo";
import Home1 from "@/components/home/home";
import Search_item_link_standard from "@/components/internal_components/search/item/link/search_item_link_standard";
import { search_builder } from "@/library/common/search";
import Link from "next/link";
import Search_Input1 from "@/components/internal_components/search/input/input1";

export default function Home() {
    const results = [0, 1, 2, 3, 4, 5, 6, 7];
    const results_ul = results.map((data) => {
        return (
            <Search_item_link_standard/>
        )
    });

    return (
        <Home1 className="query_search_container">
            <div className="query_search_bar_container">
                <Logo/>
                <Search_Input1 className="query_search_bar_input"/>
            </div>

            <div className="query_search_results_outer">
                <div className="query_search_results">
                    {/* <p className="do_you_mean greyText">Do you mean <Link href={search_builder("skeet")} className="hover_underline">skeet</Link>, <Link href={search_builder("poasting")} className="hover_underline">poasting</Link>?</p> */}
                    <p className="do_you_mean greyText">Common queries: <Link href={search_builder("beans")} className="hover_underline">beans</Link></p>
                    <div className="query_search_item_results">
                        {results_ul}
                        {results.length > 0 && <p className="reached_the_end greyText">You've reached the end.</p>}
                    </div>
                </div>
            </div>
        </Home1>
    )
}