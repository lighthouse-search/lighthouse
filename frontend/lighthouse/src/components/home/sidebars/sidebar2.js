import ProfilePic from "@/components/user/profile_pic";
import "./css/sidebar2.css";
import SidebarButton1 from "./sidebar-buttons/sidebarbutton1";
import UserCard1 from "@/components/user/user_cards/user_card1";
import Search_Input1 from "@/components/internal_components/search/input/input1";
import { useEffect, useRef, useState } from "react";
import Logo from "@/components/internal_components/logo/logo";
import Dropdown from "@/components/miscellaneous/dropdown";
import { credentials_object } from "@/global";
import { useRouter } from "next/router";
import { Lighthouse } from "@oracularhades/lighthouse";
import Link from "next/link";
import LoadingSpinner from "@/components/miscellaneous/loadingspinner";

export default function Sidebar2(page_props) {
    const router = useRouter();
    const should_run = useRef(true);
    const [user, set_user] = useState(null);
    const [notices, set_notices] = useState(null);
    const [metadata, set_metadata] = useState(null);

    const [mini_search, set_mini_search] = useState(false);

    async function run() {
        let user = await Lighthouse(credentials_object(router)).account.me();
        user.data.profile_pic = `/profile-pictures/${Math.floor(Math.random() * 5)}.png`;
        set_user(user.data);
    }

    async function metadata_get() {
        const metadata = await Lighthouse({ ... credentials_object(router), fetch_properties: { cache: "reload" } }).metadata.urls();
        set_metadata(metadata.data);
    }

    useEffect(() => {
        if (typeof window != "undefined" && !window.location.pathname.startsWith("/search") && !window.location.pathname.startsWith("/query")) {
            set_mini_search(true);
        }

        if (should_run.current != true) { return; }
        should_run.current = false;

        metadata_get();
        run();
    });

    const notices_ul = (notices ? notices : []).map((data) => {
        return (
            <div className="notice outline secondary_element">
                <p>Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.</p>
            </div>
        );
    });

    let guard_url = null;
    if (metadata && metadata.guard) {
        let url = new URL(metadata.guard);
        url.pathname = url.pathname+"/frontend/login";
        url.searchParams.set("redirect", window.location.href);
        guard_url = url.href;
    }

    return (
        <div className="sidebar2 column">
            <div className={`sidebar2_row ${page_props.className}`}>
                <div className="left">
                    <Logo/>
                    {/* <SidebarButton1 alias="Search" href="/search" icon="/emojis/1f453_glasses_3d.png"/> */}
                    {/* <SidebarButton1 alias="E-mail" href="/email" icon="/emojis/1f4e7_email_3d.png"/>
                    <SidebarButton1 alias="Calendar" href="/email" icon="/emojis/1f4c5_calendar_3d.png"/>
                    <SidebarButton1 alias="Your Stuff" href="/your-stuff" icon="/emojis/1f5c3_cardfilebox_3d.png"/> */}
                    {/* <SidebarButton1 alias="Calendar" href="/calendar" icon="/emojis/1f4c5_calendar_3d.png"/> */}
                    {/* <SidebarButton1 alias="Documents" href="/documents" icon="/emojis/1f4d3_notebook_3d.png"/> */}
                    {/* <SidebarButton1 alias="Storage" href="/storage" icon="/emojis/1f4c1_filefolder_3d.png"/> */}
                    {/* <SidebarButton1 alias="Maps" href="/map" icon="/emojis/1f30e_earthglobeamericas_3d.png"/> */}
                </div>

                {credentials_object() != null && <div className="right">
                    <Link href="/admin">Admin</Link>

                    {/* {user && <Dropdown icon={<ProfilePic src={user.profile_pic}/>}>
                        <UserCard1 user={{ ...user }}/>
                        <button>Logout</button>
                    </Dropdown>} */}
                    <ProfilePic href="/settings" check={user} src={user ? user.profile_pic : null}/>

                    {/* <SidebarButton1 href="/settings" icon="/icons/cogwheel-outline-white.svg"/> */}
                    {/* {mini_search == true && <Search_Input1/>} */}
                </div>}

                {metadata && credentials_object() == null && <div className="right">
                    <Link href={guard_url} className="login">Login</Link>
                </div>}
            </div>

            <div className="notices column row_gap_4 scrollY">
                {notices_ul}
            </div>
        </div>
    )
}