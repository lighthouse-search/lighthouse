import './css/switcher.css';
import expand_more from '../assets/expand_more.svg';
import Image from 'next/image';
import HoverFrame from './hover_frame';
import './../global.css';

export default function Switcher(props) {
    const side = props.side ? props.side : "left";

    return (
        <HoverFrame className="switcher_hoverframe" hover={props.children}><button className='switcher hover'>
            {side == "left" && <Image src={expand_more}/>}
            <h2 className='layout_topbar_header'>{props.header}</h2>
            {side == "right" && <Image src={expand_more}/>}
        </button></HoverFrame>
    )
}