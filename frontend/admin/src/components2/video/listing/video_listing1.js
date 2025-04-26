import Link from 'next/link';
import './css/video_listing1.css';

export default function Video_listing1(props) {
    let href = "";
    if (props.href) {
        href = props.href;
    }

    return (
        <Link href={href} className='Video_listing1'>
            <button onClick={props.onClick}>
                {props.thumbnail && <img className='Video_listing1_thumbnail' src={props.thumbnail}/>}
                <div className='Video_listing1_content'>
                    <p className='Video_listing1_content_header'>{props.title}</p>
                    <p className='Video_listing1_content_subtext greyText'>{props.subtext}</p>
                </div>
            </button>
        </Link>
    )
}