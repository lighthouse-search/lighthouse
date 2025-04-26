import Link from 'next/link';
import './logo.css';

export default function Logo(props) {
    return (
        <Link href="/home" className="logo_div no_a">
            <h1 className="logo_title hover_underline">Admin</h1>
        </Link>
    )
}