import Link from 'next/link';
import './css/email_row1.css';
import ProfilePic from "@/components/user/profile_pic";

export default function Email_row1(props) {
    return (
        <Link href="#" className="email_row1">
            <ProfilePic name="Microsoft"/>
            <div className='metadata'>
                <p>John Doe</p>
                {/* <p>Office365Alerts@microsoft.com</p> */}
                <p>•</p>
                <p>Informational-severity alert: User requested to release a quarantined message</p>
            </div>
        </Link>
    )
}