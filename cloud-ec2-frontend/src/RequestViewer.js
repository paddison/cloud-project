import React, {useState} from "react";

function RequestViewer(props) {

    return (
        <span id="status">{ props.status }</span>
    )
}

export default RequestViewer