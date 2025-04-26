import ProfilePic from "@/components/user/profile_pic";
import "./css/sidebar2.css";
import SidebarButton1 from "./sidebar-buttons/sidebarbutton1";
import UserCard1 from "@/components/user/user_cards/user_card1";
import Search_Input1 from "@/components/internal_components/search/input/input1";
import { useEffect, useState } from "react";
import Logo from "@/components/internal_components/logo/logo";

export default function Sidebar2(props) {
    const [mini_search, set_mini_search] = useState(false);

    useEffect(() => {
        if (typeof window != "undefined" && !window.location.pathname.startsWith("/search") && !window.location.pathname.startsWith("/query")) {
            set_mini_search(true);
        }
    });

    return (
        <div className={`sidebar2 ${props.className}`}>
            <div className="left">
                <Logo/>
                {/* <SidebarButton1 alias="Search" href="/search" icon="/emojis/1f453_glasses_3d.png"/> */}
                <SidebarButton1 alias="E-mail" href="/email" icon="/emojis/1f4e7_email_3d.png"/>
                <SidebarButton1 alias="Your Stuff" href="/your-stuff" icon="/emojis/1f5c3_cardfilebox_3d.png"/>
                {/* <SidebarButton1 alias="Calendar" href="/calendar" icon="/emojis/1f4c5_calendar_3d.png"/> */}
                {/* <SidebarButton1 alias="Documents" href="/documents" icon="/emojis/1f4d3_notebook_3d.png"/> */}
                {/* <SidebarButton1 alias="Storage" href="/storage" icon="/emojis/1f4c1_filefolder_3d.png"/> */}
                <SidebarButton1 alias="Maps" href="/map" icon="/emojis/1f30e_earthglobeamericas_3d.png"/>
            </div>

            <div className="right">
                <ProfilePic hover={<UserCard1 user={{ name: "Example user", email: "user@example.com" }}>
                    <button>Logout</button>
                </UserCard1>}/>
                <SidebarButton1 href="/settings" icon="/icons/cogwheel-outline-white.svg"/>
                {mini_search == true && <Search_Input1/>}
            </div>
        </div>
    )
}