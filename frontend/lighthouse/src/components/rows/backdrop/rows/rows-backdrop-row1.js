import '@/../styles/global.css';
import "@/components/global.css";
import './css/rows-backdrop-row1.css';
import Link from 'next/link';

export default function Rows_backdrop_row1(props) {
    const className = `Rows_backdrop_row1 secondary_element outline ${props.className}`;

    const Content = (() => {
        return (
            <button {...props} onClick={props.onClick} disabled={props.disabled} className={className}>
                <div className='Rows_backdrop_row1_left'>
                    {props.icon && typeof props.icon == "string" && <img className='Rows_backdrop_row1_left_icon' src={props.icon}/>}
                    {props.icon && typeof props.icon != "string" && props.icon}
                    <div className='Rows_backdrop_row1_left_content'>
                        {typeof props.header == "string" && <p className='Rows_backdrop_row1_left_content_header'>{props.header}</p>}
                        {typeof props.header != "string" && props.header}

                        {typeof props.subchildren == "string" && <p className='Rows_backdrop_row1_left_content_subtext greyText'>{props.subchildren}</p>}
                        {typeof props.subchildren != "string" && props.subchildren}
                    </div>
                </div>
                <div className='Rows_backdrop_row1_right'>
                    {props.right}
                    {/* <button>
                        <img src="/icons/pencil_over_line.svg"/>
                    </button>
                    <button>
                        <img src="/icons/trash.svg"/>
                    </button> */}
                </div>
            </button>
        )
    });

    if (props.href) {
        return (
            <Link href={props.href}>
                <Content/>
            </Link>
        )
    } else {
        return (
            <Content/>
        )
    }
}