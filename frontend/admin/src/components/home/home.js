import Base from "../base";
import Sidebar2 from "./sidebars/sidebar2";
import './../global.css';
import "./css/home.css"
import Link from "next/link";
import { array_string_every_item } from "../../global";

export default function Home1(props) {
    // let full_background = props.full_background ? props.full_background : false;
    let full_background = false;

    const headers = (props.header ? array_string_every_item(props.header, "/") : []).map((header) => {
        return (
            <Link href={`/${header.toLowerCase()}`}>
                <h3 className="hover_underline greyText">{header}</h3>
            </Link>
        );
    });

    return (
        <Base className={`home1 ${full_background == true ? "search_home_base" : ''}`}>
            <Sidebar2 className={`${full_background == false ? "search_home_base" : ''}`}/>
            <div className={"home1_children "+props.className} style={props.style}>
                <div className="row space_between">
                    <div className="row column_gap_6">
                        {headers}
                    </div>
                    <div className="row">
                        {props.header_children}
                    </div>
                </div>
                {props.children}
            </div>
        </Base>
    )
}