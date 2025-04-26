import HoverFrame from "../miscellaneous/hover_frame";
import "./css/profile_pic.css";
import './../global.css';
import Link from "next/link";
import LoadingSpinner from "../miscellaneous/loadingspinner";

export default function ProfilePic(props) {
    const loading = (props.check == null ? true : false); // if props.loading is null, loading=true.
    const Content = (() => {
        return (
            <button {...props} style={props.style} className={`profile_pic ${props.className}`}>
                {loading == false && <img src={(props.src != null && props.src != "/default-pfp.png") ? props.src : `/profile-pictures/0.png`} alt="Your profile picture."/>}
                {loading == true && <LoadingSpinner style={{ width: 15, height: 15 }}/>}
                {/* <div className="online_indicator"/> */}
            </button>
        )
    });
    if (props.href) {
        return (
            <Link href={props.href}>
                <Content/>
            </Link>
        )
    }
    return (
        <Content/>
    )
}