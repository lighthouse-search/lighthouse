import ProfilePic from '../profile_pic';
import './css/bluesky_user_details_row.css';
import './../../global.css';
import '@/components/global.css';
import { get_ago, number_to_month } from '@/library/common/date';
import { commas_to_number } from '@/library/common/formatting';
import Button_with_icon from '@/components/button/image/button_with_icon';
import Link from 'next/link';
import ToolTip from '@/components/miscellaneous/tooltip';

export default function Bluesky_bluesky_user_details_row(props) {
    // 1.0 follow count less than following
    // <1.0 follow count more than following
    // 0.0 = bad, 1.0 (and above) is good.

    // let ratio = Math.log10(props.user.followersCount + 1) / Math.log10(props.user.followsCount + 1);
    // let ratio_warning = ratio <= 0.87 && props.user.followsCount > 1000;

    let ratio = props.user.followersCount - props.user.followsCount;
    let ratio_warning = ratio <= -8000 && props.user.followsCount > 1000;

    // let ratio_warning = ratio <= 0.80 && props.user.followsCount > 10000;
    // let ratio = props.user.followersCount / props.user.followsCount;
    // let ratio = Math.log10(6 + 1) / Math.log10(8000 + 1);
    // let ratio_floor = 100-Math.floor(ratio * 100);
    // if (ratio_floor > 100) { ratio_floor = 100; }

    const Stat = ((stat_props) => {
        if (props.features && !props.features.includes(stat_props.type)) { return; }
        return (
            <p className={`${stat_props.className} ${stat_props.warning && stat_props.warning == true ? 'warning_text' : 'greyText'}`}>{stat_props.value} <b>{stat_props.type}</b></p>
        )
    });

    let joined_date = new Date(props.user.createdAt);
    const joined = get_ago(joined_date);
    
    let url = new URL("https://bsky.app");
    url.pathname = `/profile/${props.user.did}`;

    // 0.444999111

    function feature_enabled(type) {
        return !props.features || props.features.includes("joined");
    }
    
    return (
        <div href={url.href != null ? url.href : "#"} target='_blank' rel='noreferrer' className={`bluesky_user_details_row ${props.className} hover`}>
            <ProfilePic src={props.user.avatar}/>
            <div className='details'>
                {/* {props.user.displayName && <p className='bluesky_user_details_row_user_details_name'>{props.user.displayName}</p>} */}
                {props.user.handle && <Link href={url.href != null ? url.href : "#"} target='_blank' rel='noreferrer' className='bluesky_user_details_row_user_details_name'>{props.user.handle} • <span className='greyText'>{props.user.displayName}</span></Link>}
                <div className='user_stats'>
                    {/* <Stat className="subtext" value={commas_to_number(NaN)} type="reports against user"/> */}
                    {/* <p className='greyText'>•</p> */}
                    {feature_enabled("F/F score") && <ToolTip text={ratio}><Stat className="subtext" warning={ratio_warning} value={ratio} type={`F/F score`}/></ToolTip>}
                    <Stat className="subtext" value={commas_to_number(props.user.followersCount)} type="followers"/>
                    <Stat className="subtext" value={commas_to_number(props.user.followsCount)} type="following"/>
                    {/* <p className='greyText'>•</p> */}
                    <Stat className="subtext" value={commas_to_number(props.user.postsCount)} type="posts"/>
                    <Stat className="subtext" value={commas_to_number(props.user.associated.lists)} type="lists"/>
                    <Stat className="subtext" value={commas_to_number(props.user.associated.feedgens)} type="feeds"/>
                    <Stat className="subtext" value={commas_to_number(props.user.associated.starterPacks)} type="starter-packs"/>
                </div>
                {feature_enabled("joined") && <p className='subtext greyText'>Joined: <span style={{ color: joined.raw.days <= 7 && joined.raw.months == 0 ? "#fc5378" : null }}>{joined.string} ago</span> ({joined_date.getDate()} {number_to_month(joined_date.getMonth())} {joined_date.getFullYear()})</p>}
                {feature_enabled("description") && props.user.description && <p className='bio greyText'>{props.user.description}</p>}
                
                {/* <div className='user_actions'>
                    <Button_with_icon icon="/icons/gavel.svg" className="outline hover">Suspend</Button_with_icon>
                    <Button_with_icon icon="/icons/message-outline.svg" className="outline hover">Message</Button_with_icon>
                    <Button_with_icon icon="/icons/flag-outline.svg" className="outline hover">Report</Button_with_icon>
                    <Button_with_icon icon="/icons/label-outline.svg" className="outline hover">Label</Button_with_icon>
                    <Button_with_icon icon="/icons/lists-outline.svg" className="outline hover">Lists</Button_with_icon>
                </div> */}
            </div>
        </div>
    )
}