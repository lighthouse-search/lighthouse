import './search_item_link_standard.css';

export default function Search_item_link_standard(props) {
    let link = "https://bsky.app/profile/jay.bsky.team/post/3juflvnb3d62u";
    
    return (
        <div className="search_item_link_standard">
            <div className="search_item_link_standard_link_information">
                <img className='search_item_link_standard_link_information_img' src="https://bsky.app/static/favicon-32x32.png"/>
                <p className='search_item_link_standard_link_information_url'>{link}</p>
            </div>
            <a href={link} className='search_item_link_standard_title hover_underline one_line'>Jay 🦋: "Guys, please don't let "skeets" stick The experiment in decentralized naming decisions has resulted in the worst possible term How about skoots? Can we bring back skoots? Bluesky OGs, help me out here, remember skoots?" — Bluesky</a>
            <p className='search_item_link_standard_description'>Guys, please don't let "skeets" stick

The experiment in decentralized naming decisions has resulted in the worst possible term

How about skoots? Can we bring back skoots? Bluesky OGs, help me out here, remember skoots?</p>
        </div>
    )
}