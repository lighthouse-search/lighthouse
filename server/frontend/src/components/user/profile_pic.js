import HoverFrame from "../miscellaneous/hover_frame";
import "./css/profile_pic.css";
import './../global.css';

export default function ProfilePic(props) {
    return (
        <HoverFrame hover={props.hover}><button style={props.style} className={`profile_pic ${props.className}`}>
            <img src="https://cdn.discordapp.com/icons/943359784621596672/4da51359712021ed2317310be0980122.webp?size=240"/>
            {/* <div className="online_indicator"/> */}
        </button></HoverFrame>
    )
}