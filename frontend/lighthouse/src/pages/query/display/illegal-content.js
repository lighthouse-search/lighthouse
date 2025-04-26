import Home1 from "@/components/home/home";
import Link from "next/link";

export default function Illegal_content(page_props) {
    return (
        <Home1 className="query_search_container">
            <div className="row_gap_10 settings">
                <h1>Content blocked</h1>
                <div className="column row_gap_8">
                    <h2>Why</h2>
                    <p>Your query may be associated with content flagged as CSAM (Child Sexual Abuse Material).</p>
                    <p><b>Deliberate attempts to access CSAM or related material may result in you committing a criminal offence.</b></p>
                </div>
                
                <div className="column row_gap_8">
                    <h2>Get Help</h2>
                    <p>Anonymous and confidential support for concerning thoughts or behaviour:</p>
                    <ul>
                        <li>
                            <p>Safe to talk</p>
                            <Link href={"https://safetotalk.nz/get-help-for-concerning-thoughts-or-behaviour/"}>https://safetotalk.nz/get-help-for-concerning-thoughts-or-behaviour/</Link>
                        </li>
                    </ul>
                </div>

                <div className="column row_gap_8">
                    <h2>Appeal</h2>
                    <p>Blocklists are curated from a wide-range of sources. Algorithms make mistakes. If you believe this content to be unjustifiably blocked, you can anonymously appeal.</p>
                    <button>Anonymously appeal this decision</button>
                </div>

                <div className="column row_gap_8">
                    <h2>Additional information</h2>
                    <ul>
                        <li>
                            <Link href={"https://www.dia.govt.nz/Preventing-Online-Child-Sexual-Exploitation-Digital-Child-Exploitation-Filtering-System"}>Digital Child Exploitation Filtering System (DCEFS)</Link>
                        </li>
                    </ul>
                </div>

                <p className="font_size_12">This page is based on the New Zealand government's <Link className="underline" href="https://www.dia.govt.nz/digital-child-exploitation-Code-of-Practice-2024#Landing">Digital Child Exploitation Filtering System Code of Practice - October 2024</Link></p>
            </div>
        </Home1>
    )
}