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
import Button_with_icon from '@/components/button/image/button_with_icon';

export default function Sidebar2(page_props) {
    const router = useRouter();
    const should_run = useRef(true);
    const [user, set_user] = useState(null);
    const [notices, set_notices] = useState(null);

    const [mini_search, set_mini_search] = useState(false);

    async function run() {
        let user = await Lighthouse(await credentials_object(router)).account.me();
        user.data.profile_pic = `/profile-pictures/${Math.floor(Math.random() * 5)}.png`;
        set_user(user.data);
    }
    useEffect(() => {
        if (typeof window != "undefined" && !window.location.pathname.startsWith("/search") && !window.location.pathname.startsWith("/query")) {
            set_mini_search(true);
        }

        if (should_run.current != true) { return; }
        should_run.current = false;
        run();
    });

    const Sidebar_dropdown = ((props) => {
        const [visible, set_visible] = useState(true);
        return (
            <div className="sidebar_dropdown column secondary_element">
                <Button_with_icon icon={props.icon} onClick={() => { set_visible(!visible); }}>{props.alias}</Button_with_icon>
                {visible == true && <div className="items column">
                    {props.children}
                </div>}
            </div>
        )
    })

    return (
        <div className="sidebar2 column space_between">
            <div className="left row_gap_6">
                <Logo/>
                <div className="column row_gap_8 scrollY">
                    {/* <SidebarButton1 alias="Search" href="/search" icon="/emojis/1f453_glasses_3d.png"/> */}
                    <Sidebar_dropdown alias="Accounts" href="/account" icon="/emojis/1f4e7_email_3d.png">
                        <Button_with_icon href="/account/list">List</Button_with_icon>
                    </Sidebar_dropdown>

                    <Sidebar_dropdown alias="Index" icon="/emojis/1f4d6_openbook_3d.png">
                        <Button_with_icon href="/index/search">Search</Button_with_icon>
                        <Button_with_icon href="/index/add-url">Add URLs</Button_with_icon>
                        <Button_with_icon href="/index/crawl-status">Crawl status</Button_with_icon>
                    </Sidebar_dropdown>

                    <Sidebar_dropdown alias="Moderation" icon="/emojis/1f6e1_shield_3d.png">
                        <Button_with_icon href="/moderation/filter">Filter</Button_with_icon>
                        <Button_with_icon href="/moderation/appeal">Appeal</Button_with_icon>
                    </Sidebar_dropdown>

                    <Sidebar_dropdown alias="Analytics" icon="/emojis/1f4ca_barchart_3d.png">
                        <Button_with_icon href="/analytics/overview">Overview</Button_with_icon>
                        <Button_with_icon href="/analytics/popular-search">Popular search</Button_with_icon>
                        <Button_with_icon href="/analytics/login">Login</Button_with_icon>
                        <Button_with_icon href="/analytics/subscriber">Subscriber</Button_with_icon>
                        <Button_with_icon href="/analytics/email">Email</Button_with_icon>

                        {/* We only show vague information, like bandwidth used, as all this data will be end-to-end encrypted. */}
                        <Button_with_icon href="/analytics/calendar">Calendar</Button_with_icon>
                        <Button_with_icon href="/analytics/drive">Drive</Button_with_icon>
                    </Sidebar_dropdown>

                    <Sidebar_dropdown alias="Audit" icon="/emojis/270f_pencil_3d.png">
                        <Button_with_icon href="/audit/all">All</Button_with_icon>
                        <Button_with_icon href="/audit/accounts">Accounts</Button_with_icon>
                        <Button_with_icon href="/audit/index">Index</Button_with_icon>
                        <Button_with_icon href="/audit/filter">Moderation</Button_with_icon>
                    </Sidebar_dropdown>

                    {/* These should just be tied to config. */}
                    {/* <Sidebar_dropdown alias="Customisation" icon="/emojis/1f4e7_email_3d.png">
                    </Sidebar_dropdown>

                    <Sidebar_dropdown alias="Availability" icon="/emojis/1f4e7_email_3d.png">
                    </Sidebar_dropdown> */}
                </div>
            </div>

            <div className="right row column_gap_8">
                {user && <div className="row column_gap_6">
                    <ProfilePic href="/settings" check={user} src={user ? user.profile_pic : null}/>
                    <p className="font_size_14">{user.name}</p>
                </div>}

                {/* {user && <Dropdown icon={<ProfilePic src={user.profile_pic}/>}>
                    <UserCard1 user={{ ...user }}/>
                    <button>Logout</button>
                </Dropdown>} */}

                {/* <SidebarButton1 href="/settings" icon="/icons/cogwheel-outline-white.svg"/> */}
                {/* {mini_search == true && <Search_Input1/>} */}
            </div>
        </div>
    )
}