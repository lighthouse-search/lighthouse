import "./css/input1.css";
import { useEffect, useRef, useState } from "react";
import { useRouter } from "next/router";
import { search_builder } from "@/library/common/search";

export default function Search_Input1(props) {
    const router = useRouter();

    const should_run = useRef(true);
    const [query, set_query] = useState(null);

    useEffect(() => {
        if (typeof window == "undefined") { return; }

        const params = new URLSearchParams(window.location.search);
        const query = params.get("query");
        
        if (should_run.current == query) { return; }
        should_run.current = query;

        if (props.autoFocus == true) {
            document.getElementById("lighthouse_search_bar").focus();
        }

        set_query(query);
    });

    async function search(query) {
        router.push({
            pathname: "/query",
            query: { query: query }
        }, undefined, { shallow: (window.location.pathname == "/query" ? true : false) }); // Use shallow to avoid full reload;
    }

    async function input_key_down(e) {
        if (e.key == "Enter") {
            e.preventDefault();
            search(query);
            if (props.on_search) {
                props.on_search(query);
            }
        }
    }

    // let placeholder = "Search ACME CO | websites, documents, emails, files";
    let placeholder = "Search";

    return (
        <input {...props.className} id="lighthouse_search_bar" className={`search_bar_input ${props.className}`} placeholder={placeholder} value={query} onChange={(e) => { set_query(e.target.value); }} onKeyDown={input_key_down}/>
    )
}