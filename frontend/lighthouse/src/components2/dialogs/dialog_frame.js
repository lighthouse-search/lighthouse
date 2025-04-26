import "./css/dialog_frame.css";

export default function Dialog_Frame(props) {
    return (
        <dialog open={props.open} className={`dialog_frame_dialog ${props.className}`} id={props.id}>
            <div className='dialog_frame'>
                <div className="dialog_frame_top">
                    <h1 className="dialog_header">{props.header}</h1>
                    {props.show_close_button != false && <button onClick={() => { document.getElementById(props.id).close(); }} className="button dialog_frame_close"><img src="/icons/close.svg"/></button>}
                </div>
                {props.children}
            </div>
        </dialog>
    )
}