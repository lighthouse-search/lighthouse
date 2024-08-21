import "../../styles/query.css";
import "@/components/global.css";
import Logo from "@/components/internal_components/logo/logo";
import Home1 from "@/components/home/home";
import Search_item_link_standard from "@/components/internal_components/search/item/link/search_item_link_standard";
import { search_builder } from "@/library/common/search";
import Link from "next/link";

export default function Home() {
    const results = [0, 1, 2, 3, 4, 5, 6, 7];
    const results_ul = results.map((data) => {
        return (
            <Search_item_link_standard/>
        )
    });

    return (
        <div style={{ width: "100%", height: "100vh", overflow: "hidden", alignItems: "center", justifyContent: "center" }}>
            <Home1 className="query_search_container">
                <div className="query_search_bar_container">
                    <Logo/>
                    <input className="query_search_bar_input" placeholder="Search ACME | websites, documents, emails, files, code"/>
                </div>

                <div className="query_search_results_outer">
                    <div className="query_search_results">
                        <p className="do_you_mean greyText">Do you mean <Link href={search_builder("skeet")} className="hover_underline">skeet</Link>, <Link href={search_builder("poasting")} className="hover_underline">poasting</Link>?</p>
                        <div className="query_search_item_results">
                            {results_ul}
                            {results.length > 0 && <p className="reached_the_end greyText">You've reached the end.</p>}
                        </div>
                    </div>
                </div>
            </Home1>
        </div>
    )
}