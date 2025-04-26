import Base from "../base";
import Sidebar2 from "./sidebars/sidebar2";
import './../global.css';
import "./css/home.css"

export default function Home1(props) {
    let full_background = props.full_background ? props.full_background : false;
    return (
        <Base className={`home1 ${full_background == true ? "search_home_base" : ''}`}>
            <Sidebar2 className={`${full_background == false ? "search_home_base" : ''}`}/>
            <div className="home1_children">
                <div className={props.className} style={props.style}>
                    {props.children}
                </div>
            </div>
        </Base>
    )
}