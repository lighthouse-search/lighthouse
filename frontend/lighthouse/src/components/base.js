import '@/components/global.css';
import "../../styles/global.css";
import { Roboto } from 'next/font/google';
import Head from 'next/head';
import Link from 'next/link';
import Developer_settings_dialog from '@/components/settings/developer_settings_dialog';

const roboto = Roboto({
    subsets: ['latin'],
    weight: ['400', '700']
})

export default function Base(props) {
    return (
        <div className={`${props.className} ${roboto.className}`} style={props.style}>
            <Head>
                <link
                    rel="search"
                    type="application/opensearchdescription+xml"
                    href="/opensearch.xml"
                    title="Lighthouse"
                />
            </Head>
            {props.children}

            {/* Mounted globally so it's reachable on any screen, including
                when a page fails to load (e.g. a wrong/unset API endpoint). */}
            <Developer_settings_dialog/>
        </div>
    )
}