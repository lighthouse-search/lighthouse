import './search_item_link_standard.css';

export default function Search_item_link_standard(props) {
    const data = props.data;
    let url = data.url;

    let title = data.title;
    if (!title && data.url) {
        try {
            title = new URL(data.url).host
        } catch (error) {
            console.error(error);
        }
    }
    
    return (
        <div className="search_item_link_standard">
            <div className="search_item_link_standard_link_information">
                {/* <img className='search_item_link_standard_link_information_img' src="https://bsky.app/static/favicon-32x32.png"/> */}
                <p className='search_item_link_standard_link_information_url'>{url}</p>
            </div>
            <a href={url} className='search_item_link_standard_title hover_underline one_line'>{title}</a>
            <p className='search_item_link_standard_description'>{data.text}</p>
        </div>
    )
}