import './css/button_with_icon.css';

export default function Button_with_icon(props) {
    return (
        <button {...props} className={`button_with_icon ${props.className}`}>
            {props.icon && typeof props.icon == "string" && <img className={props.icon_classname} src={props.icon} alt={props.img_alt}/>}
            {props.children}
        </button>
    )
}