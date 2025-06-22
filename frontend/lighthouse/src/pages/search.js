import "./css/search.css";
import Logo from "@/components/internal_components/logo/logo";
import Home1 from "@/components/home/home";
import Search_Input1 from "@/components/internal_components/search/input/input1";
import { useEffect } from "react";
import Link from "next/link";

export default function Search() {
    useEffect(() => {
        document.title = "Lighthouse";
    });
    return (
        <Home1 className="search_container" full_background={true}>
            {/* <img className="search_bar_background" src="https://images.unsplash.com/photo-1451187580459-43490279c0fa?q=80&w=2944&auto=format&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D"/> */}
            <div className="search_bar_container">
                <Logo/>
                <Search_Input1 autoFocus={true}/>
                {/* <div className="row column_gap_10 legal">
                    <Link href="/tos" className="greyText">Terms</Link>
                    <Link href="/privacy" className="greyText">Privacy</Link>
                </div> */}
                {/* <select>
                    <option>Public</option>
                    <option>Internal</option>
                    <option>Restricted</option>
                    <option>Confidential</option>
                    <option>Highly Confidential</option>
                </select> */}
            </div>
        </Home1>
    )
}