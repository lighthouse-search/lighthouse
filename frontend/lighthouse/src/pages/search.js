import "./css/search.css";
import Logo from "@/components/internal_components/logo/logo";
import Home1 from "@/components/home/home";
import Search_Input1 from "@/components/internal_components/search/input/input1";
import Head from "next/head";
import Link from "next/link";

export default function Search() {
    return (
        <Home1 className="search_container text-white" full_background={true}>
            <Head>
                <title>Lighthouse</title>
            </Head>
            <div className="search_bar_container flex flex-col items-center w-full gap-5 -mt-16">
                <Logo/>
                <Search_Input1 autoFocus={true} className="h-[45px]"/>
            </div>

            <div className="image-attribution flex">
                <p>Photo by <Link href="https://unsplash.com/photos/gray-seal-rHmn-CYiMlo" className="underline" target="_blank">Shannon VanDenHeuvel</Link></p>
            </div>
        </Home1>
    )
}