import "./css/input_with_header.css";
import './../global.css';

export default function Input_with_header(props) {
    return (
        <div className={`input_with_header ${props.className}`}>
            <p className="input_with_header_headertxt">{props.header}</p>
            <div className="input_with_header_content">
                {!props.children && <input placeholder={props.placeholder} value={props.value} onChange={props.onChange} style={props.style} type={props.type} className={`input_with_header_input ${props.className}`} onKeyPress={e => e.key === 'Enter' && props.onKeyPress}/>}
                {props.children && props.children}
            </div>
        </div>
    )
}