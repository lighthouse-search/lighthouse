import "./css/content_header.css";
import '@/components/global.css';

export default function Content_header(props) {
    return (
        <div className={`content_header ${props.className}`}>
            <p className="content_header_headertxt">{props.header}</p>
            <div className="content_header_content">
                {props.children}
            </div>
        </div>
    )
}