import "./css/dropdown.css";
import './../global.css';
import React, { useEffect, useRef, useState } from "react";

export default function Dropdown(props) {
    const [show, set_show] = useState(false);
    const [marginRight, set_marginRight] = useState(null);
    const [id, set_id] = useState(Math.random()+new Date().getTime());
    
    function check_parent(element) {
        // if (!element) {
        //     console.log("skipping check_parent, element is null...");
        //     return;
        // }
        if (element.id == id) {
            return true;
        }
        
        if (element.parentElement) {
            return check_parent(element.parentElement);
        } else {
            return false;
        }
    }
    function click_outside(event) {
        if (check_parent(event.target.parentElement) == false) {
            set_show(false);
            document.removeEventListener('click', click_outside);
        }
    }

    function isDivOffscreen(div) {
        const rect = div.getBoundingClientRect();
        const viewportWidth = window.innerWidth || document.documentElement.clientWidth;
        const viewportHeight = window.innerHeight || document.documentElement.clientHeight;
      
        return (
            rect.top < 0 ||                   // Top of the div is above the viewport
            rect.left < 0 ||                  // Left of the div is to the left of the viewport
            rect.bottom > viewportHeight ||   // Bottom of the div is below the viewport
            rect.right > viewportWidth        // Right of the div is to the right of the viewport
        );
    }

    useEffect(() => {
        if (document.getElementById(id) && document.getElementById(id+"-content")) {
            if (isDivOffscreen(document.getElementById(id+"-content")) == true) {
                set_marginRight(-document.getElementById(id+"-content").offsetWidth + document.getElementById(id).offsetWidth);
            }
        }
    });

    async function show_content() {
        if (show == false) {
            // document.addEventListener('click', click_outside);
        }
        set_show(!show);
    }

    return (
        <div key={props.tkey} className={`dropdown ${props.className}`} id={id}>
            {React.cloneElement(props.icon, { onClick: (() => { show_content() }), className: 'dropbtn' })}
            {show && <div id={id+"-content"} style={{ marginRight: marginRight, pointerEvents: "none" }} className="dropdown-content outline">{props.children}</div>}
        </div>
    )
}