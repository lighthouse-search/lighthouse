import "../../styles/search.css";
import Logo from "@/components/internal_components/logo/logo";
import Home1 from "@/components/home/home";
import Search_Input1 from "@/components/internal_components/search/input/input1";

export default function Home() {
    return (
        <div style={{ width: "100%", height: "100vh", overflow: "hidden", alignItems: "center", justifyContent: "center" }}>
            <Home1 className="search_container">
                {/* <img className="search_bar_background" src="https://images.unsplash.com/photo-1451187580459-43490279c0fa?q=80&w=2944&auto=format&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D"/> */}
                <div className="search_bar_container">
                    <Logo/>
                    <Search_Input1/>
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