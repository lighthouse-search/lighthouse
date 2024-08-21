import ProfilePic from "@/components/user/profile_pic";
import "./css/sidebar2.css";
import SidebarButton1 from "./sidebar-buttons/sidebarbutton1";
import UserCard1 from "@/components/user/user_cards/user_card1";

export default function Sidebar2(props) {
    return (
        <div className="sidebar2">
            <ProfilePic hover={<UserCard1 user={{ name: "Example user", email: "user@example.com" }}>
                <button>Logout</button>
            </UserCard1>}/>
            <SidebarButton1 alias="Search" href="/search" icon="/emojis/1f453_glasses_3d.png"/>
            <SidebarButton1 alias="Maps" href="/map" icon="/emojis/1f30e_earthglobeamericas_3d.png"/>
            <SidebarButton1 alias="Documents" href="/documents" icon="/emojis/1f4d3_notebook_3d.png"/>
            <SidebarButton1 alias="Storage" href="/storage" icon="/emojis/1f4c1_filefolder_3d.png"/>
        </div>
    )
}