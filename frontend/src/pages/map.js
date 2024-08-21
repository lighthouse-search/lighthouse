import "../../styles/map.css";
import Home1 from "@/components/home/home";

export default function Home() {
    return (
        <Home1 className="map_container">
            <iframe className="map_iframe" src="https://earth.nullschool.net/#current/wind/surface/level/orthographic=-186.08,-38.34,819"/>
        </Home1>
    )
}