import "../../styles/home.css";
import Logo from "@/components/internal_components/logo/logo";
import Home1 from "@/components/home/home";
import { useState } from "react";
import { useRouter } from "next/router";
import { search_builder } from "@/library/common/search";

export default function Home() {
    const router = useRouter();
    const [query, set_query] = useState(null);

    async function search(query) {
        router.push(search_builder(query));
    }

    async function input_key_down(e) {
        if (e.key == "Enter") {
            e.preventDefault();
            search(query);
        }
    }
    
    return (
        <div style={{ width: "100%", height: "100vh", overflow: "hidden", alignItems: "center", justifyContent: "center" }}>
            <Home1 className="search_container">
                {/* <img className="search_bar_background" src="https://images.unsplash.com/photo-1451187580459-43490279c0fa?q=80&w=2944&auto=format&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D"/> */}
                <div className="search_bar_container">
                    <Logo/>
                    <input className="search_bar_input" placeholder="Search MotionFans | websites, documents, emails, files" value={query} onChange={(e) => { set_query(e.target.value); }} onKeyDown={input_key_down}/>
                    {/* <select>
                        <option>Public</option>
                        <option>Internal</option>
                        <option>Restricted</option>
                        <option>Confidential</option>
                        <option>Highly Confidential</option>
                    </select> */}
                </div>
            </Home1>
        </div>
    )
}