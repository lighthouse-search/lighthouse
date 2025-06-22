import "./css/account.css";
import Home1 from "@/components/home/home";
import { useEffect, useRef, useState } from "react";
import { Lighthouse } from "@oracularhades/lighthouse";
import { credentials_object } from "@/global";
import { useRouter } from "next/router";
import Button_with_icon from "@/components/button/image/button_with_icon";
import Loading from "@/components/navigating/in-progress/loading";
import ProfilePic from "@/components/user/profile_pic";
import Dialog_Frame from "@/components/dialogs/dialog_frame";
import Input_with_header from "@/components/input/input_with_header";

export default function Account_list() {
    const router = useRouter();
    const should_run = useRef(true);

    const [accounts, set_accounts] = useState([]);

    useEffect(() => {
        if (should_run.current != true) { return; }
        should_run.current = false;
        
        get_account();
    });

    async function get_account() {
       const accounts = await Lighthouse(credentials_object(router)).account.list();
       set_accounts(accounts.data);
    }

    if (accounts.length == 0) {
        return (
            <Home1 header={["Accounts", "[loading]"]} full_background={true} className="home_padding row_gap_8">
                <Loading/>
            </Home1>
        )
    }

    return (
        <Home1 header={["Accounts", `"${accounts[0].name}"`]} header_children={<Button_with_icon>Suspend</Button_with_icon>} full_background={true} className="account home_padding row_gap_8">
            <Dialog_Frame id="user_will_be_notified" header="Sensitive information - Notify user" className="row_gap_4">
                <p className="font_size_14">If you continue, we'll notify <a className="greyText">{accounts[0].email}</a>. Before continuing, seek legal advice, you are bound to privacy laws and user agreements. Only access this data if you believe an account is breaking the law or spam, do not violate this user's privacy.</p>
                <button onClick={() => { document.getElementById("multifactor_dialog").showModal(); }}>Continue (Notify user)</button>
            </Dialog_Frame>

            <Dialog_Frame id="multifactor_dialog" header="Let's verify it's you" className="row_gap_4">
                <p className="greyText font_size_12">You're performing a sensitive action. Let's make sure it's really you.</p>
                <Input_with_header header="2FA code"/>
            </Dialog_Frame>

            <div className="row column_gap_8">
                <ProfilePic check={accounts[0]} src={accounts[0]?.profile_pic} className="profile_picture"/>
                <div className="column">
                    <h2>{accounts[0].name}</h2>
                    <p className="greyText">{accounts[0].email}</p>
                </div>
            </div>

            <div className="row column_gap_4">
                <button>Information</button>
                <button>Audit Log</button>
                <button>Subscription</button>
                <button>Drive</button>
                <button>Email</button>
                <button>Data export</button>
            </div>

            {/* <button onClick={() => { document.getElementById("user_will_be_notified").showModal(); }}>Test dialog</button> */}

            {/*
                Subscription
                Drive (resource usage)
                Login history
                2FA information.
                (User) Audit log && (Admin) Audit log
                Data download/export (law enforcement)
                Ability to inspect emails / See email count / see who they're emailing. Need to be extremely careful with this, send notifications.
            */}
        </Home1>
    )
}