import "./css/settings.css";
import "@/components/global.css";
import Home1 from "@/components/home/home";
import Search_item_link_standard from "@/components/internal_components/search/item/link/search_item_link_standard";
import { useEffect } from "react";

export default function Settings() {
    useEffect(() => {
        document.title = "Settings";
    });
    const results = [0, 1, 2, 3, 4, 5, 6, 7];
    const results_ul = results.map((data) => {
        return (
            <Search_item_link_standard/>
        )
    });

    return (
        <Home1 className="query_search_container">
            <h1>Settings</h1>
        </Home1>
    )
}