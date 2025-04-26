import ToolTip from '@/components/miscellaneous/tooltip';
import './css/sidebarbutton1.css';
import Link from 'next/link'

export default function SidebarButton1(props) {
    return (
        <Link href={props.href} className="Sidebarbutton row hover_underline">
            {props.icon && <img src={props.icon}/>}
            {props.alias && props.alias}
            {props.children && props.children}
        </Link>
    )
}