import Home1 from "@/components/home/home";
import { useEffect, useRef, useState } from "react";
import { useRouter } from "next/router";
import Button_with_icon from "@/components/button/image/button_with_icon";

export default function Account_list() {
    const router = useRouter();
    const should_run = useRef(true);

    return (
        <Home1 header={["Home"]} full_background={true} className="account home_padding row_gap_8">
        </Home1>
    )
}