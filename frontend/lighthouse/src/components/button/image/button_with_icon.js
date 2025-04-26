import Link from 'next/link';
import './css/button_with_icon.css';

export default function Button_with_icon(props) {
    const icon_props = { className: props.icon_classname, style: props.icon_style }
    const Content = ((props) => {
        return (
            <button {...props} className={`button_with_icon ${props.className}`}>
                {props.icon && props.side != "right" && typeof props.icon == "string" && <img {...icon_props} src={props.icon}/>}
                {props.children}
                {props.icon && props.side == "right" && typeof props.icon == "string" && <img {...icon_props} className={props.icon_classname} src={props.icon}/>}
            </button>
        )
    });

    if (props.href) {
        return (
            <Link href={props.href}>
                <Content {...props}/>
            </Link>
        )
    } else {
        return <Content {...props}/>
    }
}