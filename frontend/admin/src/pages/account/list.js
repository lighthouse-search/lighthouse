import "./css/list.css";
import Home1 from "@/components/home/home";
import { useEffect, useRef, useState } from "react";
import Search_Input1 from "../../components/internal_components/search/input/input1";
import { Lighthouse } from "@oracularhades/lighthouse";
import { credentials_object } from "@/global";
import { useRouter } from "next/router";
import UserCard1 from "@/components/user/user_cards/user_card1";
import Button_with_icon from "@/components/button/image/button_with_icon";

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

    const accounts_ul = accounts.map((account) => {
        return (
            <UserCard1 user={account} href={`/account/${account.id}`} className="hover"/>
        )
    });

    return (
        <Home1 header={["Accounts"]} header_children={<Button_with_icon>Create</Button_with_icon>} full_background={true} className="home_padding row_gap_8">
            <Search_Input1/>
            <div className="column scrollY">
                {accounts_ul}
            </div>
        </Home1>
    )
}