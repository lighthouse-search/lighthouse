import "./css/search.css";
import Home1 from "@/components/home/home";
import { useEffect, useRef, useState } from "react";
import Search_Input1 from "../../components/internal_components/search/input/input1";
import { Lighthouse } from "@oracularhades/lighthouse";
import { credentials_object } from "@/global";
import { useRouter } from "next/router";
import Button_with_icon from "@/components/button/image/button_with_icon";
import LoadingSpinner from "../../components/miscellaneous/loadingspinner";
import Link from "next/link";
import No_results from "@/components/tip/no_results";
import Loading from "@/components/navigating/in-progress/loading";

export default function Search_list() {
    const router = useRouter();
    const should_run = useRef(true);

    const [loading, set_loading] = useState(false);
    const [urls, set_urls] = useState([]);

    useEffect(() => {
        if (should_run.current != true) { return; }
        should_run.current = false;
        
        job_list();
    });

    async function job_list() {
        set_loading(true);
        const jobs = await Lighthouse(credentials_object(router)).admin.index.job.list();
        set_urls(jobs.data);
        set_loading(false);
    }

    async function cancel_job(id) {
        await Lighthouse(credentials_object(router)).admin.index.job.update({
            actions: [
                { action: "delete", id: id }
            ] 
        });
    }

    // const urls = [
    //     { status: "waiting", urls: ["https://bsky.app/profile/selenalarson.bsky.social"] }
    // ]

    const urls_ul = urls.map((job) => {
        const urls_ul = job.urls.map((url_string) => {
            return (
                <p className="font_size_14">{url_string}</p>
            )
        });

        return (
            <div className="row space_between">
                <div className="row column_gap_8">
                    {job.status == "waiting" && <LoadingSpinner style={{ width: 15, height: 15  }}/>}
                    <div className="column column_gap_8">
                        <p className="greyText font_size_14">{job.status}</p>
                        {urls_ul}
                    </div>
                </div>

                <div className="row column_gap_8">
                    <Link href="#" onClick={() => { cancel_job(job.id) }}>Cancel</Link>
                    <Link href="#">View</Link>
                </div>
            </div>
        )
    });

    if (loading == true) {
        return (
            <Home1 header={["Index", "Crawl status"]} header_children={<Button_with_icon href="/index/add-url">Add URLs</Button_with_icon>} full_background={true} className="home_padding row_gap_8">
                <Loading/>
            </Home1>
        )
    }

    return (
        <Home1 header={["Index", "Crawl status"]} header_children={<Button_with_icon href="/index/add-url">Add URLs</Button_with_icon>} full_background={true} className="home_padding row_gap_8">
            <Search_Input1/>
            {urls_ul.length >= 0 && <div className="row_gap_8 column scrollY">
                {urls_ul}
            </div>}
            {urls_ul.length == 0 && <No_results/>}
        </Home1>
    )
}