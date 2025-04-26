import '@/components/global.css';
import "../../styles/global.css";
import { Roboto } from 'next/font/google';
import Head from 'next/head';

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
        </div>
    )
}