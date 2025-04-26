import HoverFrame from "../miscellaneous/hover_frame";
import "./css/profile_pic.css";
import './../global.css';
import { useEffect, useRef, useState } from "react";

export default function ProfilePic(props) {
    const should_run = useRef(true);
    const [name_letters, set_name_letters] = useState(null);

    useEffect(() => {
        let name_letters_draft = null;
        if (props.name) {
            const str = props.name;
            let words = str.split(' ');
            let first_letter = words[0].split('')[0];
            let last_letter = words.length > 1 ? words[words.length - 1].split('')[0] : '';
            name_letters_draft = `${first_letter}${last_letter}`;
        }

        if (should_run.current == name_letters_draft) { return; }
        should_run.current = name_letters_draft

        set_name_letters(name_letters_draft);
    });

    return (
        <HoverFrame hover={props.hover}><button style={props.style} className={`profile_pic ${props.className}`}>
            <img src={props.src ? props.src : "/profile-pictures/2.png"} alt="Your profile picture."/>
            {props.name && name_letters && !props.src && `${name_letters}`}
        </button></HoverFrame>
    )
}