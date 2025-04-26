import Link from 'next/link';
import './css/no_results.css';

export default function No_results(props) {
    // const Second_part = ((props) => {
    //     if (!props.tip) {
    //         return;
    //     }

    //     let tip = props.tip;
    //     if (typeof props.tip == "string") {
    //         tip = <a href={props.tip_href} target="_blank" rel="noreferrer">{props.tip}</a>
    //     }

    //     return (
    //         <div>
    //             -
    //             {tip}
    //         </div>
    //     )
    // });

    return (
        <p className="greyText">{props.custom_header ? props.custom_header : "No results"} {props.tip && `- `}{props.tip && <Link href={props.tip_href ? props.tip_href : ''} target="_blank" rel="noreferrer">{props.tip}</Link>}</p>
    )
}