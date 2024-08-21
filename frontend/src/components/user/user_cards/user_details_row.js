import ProfilePic from '../profile_pic';
import './css/user_details_row.css';
import './../../global.css';

export default function UserDetailsRow(props) {
    return (
        <div className={`user_details_row ${props.className}`}>
            <ProfilePic/>
            <div className='user_details_row_user_details'>
                {props.user.name && <p className='user_details_row_user_details_name'>{props.user.name}</p>}
                {props.user.email && <p className='user_details_row_user_details_subtext greyText'>{props.user.email}</p>}
            </div>
        </div>
    )
}