import logo from './logo.svg';
import './App.css';
import WaveForm from "./WaveForm";
import {useState} from "react";
import RequestViewer from "./RequestViewer";
import { Buffer } from "buffer";


function App() {

    const [ reqState, setReqState ] = useState("loading...")

    const { REACT_APP_FIRST_REQ_URL } = process.env;
    const { REACT_APP_SECOND_REQ_URL } = process.env;

    async function handleSubmit(specs) {
        setIsSubmit(true);
        console.log("specs: " + JSON.stringify(specs))

        const data = await fetch(REACT_APP_FIRST_REQ_URL, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify(specs)
        })

        const response = await data.json();
        console.log(data.status)
        console.log("received data: "+ JSON.stringify(response), response.request_id);


        if (data.status === 200) {
            setReqState("loading...")
            // todo: send second request to manager lambda
            const waveId = response.id;
            const reqId = response.request_id;
            let inProgress = true;

            while(inProgress) {
                const res = await fetch(`${REACT_APP_SECOND_REQ_URL}?file_id=${waveId}&request_id=${reqId}`, {
                    method: 'GET',
                    headers: {
                        'Content-Type': 'application/json'
                    },
                    queryStringParameters: {
                        "file_id": waveId,
                        "request_id": reqId
                    }
                })
                const dataLink = await res.json();
                console.log(JSON.stringify(dataLink))

                if (dataLink.body?.status === "in_progress") {
                    setReqState("loading...")
                    await new Promise(r => setTimeout(r, 2000));
                } else if (dataLink.headers && dataLink.headers["Content-Type"] === "text/plain") {
                    setReqState("ready to download");
                    console.log(dataLink.body)
                    const buffer = Buffer.from(dataLink.body, 'base64');
                    console.log(buffer);
                    const file = new File([buffer], waveId, {type: "audio/wav"});
                    console.log(file)
                    setDownloadUrl(window.URL.createObjectURL(file));
                    setDownloadButtonEnabled(true);
                    inProgress = false;
                } else {
                    setReqState("invalid request");
                    inProgress = false;
                }
            }
        } else {
            setReqState(data.statusText)
        }
    }

    const [ isSubmit, setIsSubmit ] = useState(false);
    const [ bucketUrl, setBucketUrl ] = useState();
    const [ downloadUrl, setDownloadUrl ] = useState()

    const [ downloadButtonEnabled, setDownloadButtonEnabled ] = useState(false);

      return (
        <div className="App">
          <span id="logo">WaveBuilder</span>
            { isSubmit?<RequestViewer status={reqState}/>:<WaveForm handleSubmit={handleSubmit}/> }
            { downloadButtonEnabled?<a href={downloadUrl} download onClick={() => {
                setDownloadButtonEnabled(false);
            }}><button id="downloadEnabled">Download</button></a>:<button id="downloadDisabled">Download</button> }
        </div>
      );
}

export default App;