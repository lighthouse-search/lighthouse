import { useEffect } from "react";
import "./css/map.css";
import Home1 from "@/components/home/home";

export default function Map() {
    useEffect(() => {
        document.title = "Map";
    });
    return (
        <Home1 className="map_container">
            <iframe className="map_iframe" src="https://earth.nullschool.net/#current/wind/surface/level/orthographic=-186.08,-38.34,819"/>
        </Home1>
    )
}