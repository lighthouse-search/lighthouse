import "./css/settings.css";
import "@/components/global.css";
import Home1 from "@/components/home/home";
import Email_row1 from "@/components/internal_components/email/item/rows/email_row1";
import Search_item_link_standard from "@/components/internal_components/search/item/link/search_item_link_standard";
import { useEffect, useState } from "react";

export default function Email() {
    useEffect(() => {
        document.title = "Email";
    });
    const results = [0, 1, 2, 3, 4, 5, 6, 7];
    const results_ul = results.map((data) => {
        return (
            <Search_item_link_standard/>
        )
    });

    return (
        <Home1 className="query_search_container">
            <h2>Email</h2>
            <div>
                <Email_row1/>
                <Email_row1/>
                <Email_row1/>
                <Email_row1/>
                <Email_row1/>
                <Email_row1/>
                <Email_row1/>
                <p>You've reached the end.</p>
            </div>
        </Home1>
    )
}