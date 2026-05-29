import "./css/content-types.css";
import { useState } from "react";

const types = ["All", "Images", "Videos", "News"];

export default function ContentTypes(props) {
    const [selected, set_selected] = useState(props.selected ?? "All");

    function on_select(type) {
        set_selected(type);
        props.on_change?.(type);
    }

    return (
        <div className="content_types">
            {types.map((type) => (
                <button
                    key={type}
                    className={`content_type ${selected == type ? "selected" : ""}`}
                    onClick={() => on_select(type)}
                >
                    {type}
                </button>
            ))}
        </div>
    )
}
