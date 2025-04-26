import ToolTip from '../miscellaneous/tooltip';
import './css/profile_pics.css';

export default function Profile_pics(props) {
    const profile_pics = props.data.map((data) => {
        return (
            <ToolTip text={data.name}><img className='pfp' src={data.profile_pic}/></ToolTip>
        )
    });

    return (
        <div className='profile_pics'>
            {profile_pics}
        </div>
    )
}