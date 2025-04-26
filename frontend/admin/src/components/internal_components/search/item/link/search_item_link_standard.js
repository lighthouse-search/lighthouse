import './search_item_link_standard.css';

export default function Search_item_link_standard(props) {
    const data = props.data;
    let link = data.url;
    
    return (
        <div className="search_item_link_standard">
            <div className="search_item_link_standard_link_information">
                <img className='search_item_link_standard_link_information_img' src="https://bsky.app/static/favicon-32x32.png"/>
                <p className='search_item_link_standard_link_information_url'>{link}</p>
            </div>
            <a href={link} className='search_item_link_standard_title hover_underline one_line'>{data.content.title}</a>
            <p className='search_item_link_standard_description'>{data.content.text}</p>
        </div>
    )
}