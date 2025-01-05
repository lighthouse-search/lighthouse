import Link from 'next/link';
import './logo.css';

export default function Logo(props) {
    return (
        <Link href="/search" className="logo_div no_a">
            <h1 className="logo_title hover_underline">Lighthouse</h1>
        </Link>
    )
}