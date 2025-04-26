import './css/input_row_header.css';

export default function Input_row_header(props) {
    return (
        <div className="input_row_header">
            <div className="input_row_header_topbar">
                <div className='input_row_header_header'>
                    {props.header}
                </div>
                {props.children && <div className='input_row_header_children'>{props.children}</div>}
            </div>
            {/* <p className="input_row_header_description greyText">{props.description}</p> */}
        </div>
    )
}