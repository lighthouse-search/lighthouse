import Link from 'next/link';
import './css/user_card_row.css';
import ProfilePic from '../profile_pic';

export default function User_card_row(props) {
    const Holder = ((props) => {
        return (
            <div className='User_card_row_holder_div'>
                <p className='User_card_row_holder_header greyText'>{props.header}</p>
                <p className='User_card_row_holder_value'>{props.value}</p>
            </div>
        )
    });

    let values = Object.keys(props.data);

    const Holders = values.map((key) => {
        return (
            <Holder header={key} value={props.data[key]}/>
        )
    });

    let href = "";
    if (props.href) {
        href = props.href;
    }

    return (
        <Link href={href} onClick={props.onClick} className='User_card_row'>
            <button>
                <div className='User_card_row_holder_div_content'>
                    {Holders}
                </div>
                <div className='User_card_row_holder_div_actions'>
                    {/* <button>HI</button> */}
                </div>
            </button>
        </Link>
    )
}