import Link from 'next/link';
import './../../global.css';
import "./css/user_card1.css";
import UserDetailsRow from "./user_details_row";

export default function UserCard1(props) {
    const Content = ((embed_props) => {
        return (
            <div className={`UserCard1 ${embed_props.className}`}>
                <UserDetailsRow user={props.user}/>
                {props.children}
            </div>
        )
    });

    if (props.href) {
        return (
            <Link href={props.href} className={props.className}>
                <Content/>
            </Link>
        )
    }
    return (
        <Content className={props.className}/>
    )
}