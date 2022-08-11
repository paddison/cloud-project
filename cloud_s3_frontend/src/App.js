import './App.css';
import WaveForm from "./WaveForm";
import {useState} from "react";
import RequestViewer from "./RequestViewer";
import { Buffer } from "buffer";


function App() {

    const [ reqState, setReqState ] = useState("loading...")

    // env variables set in running system by the pipeline
    // lambda API gateway urls
    const { REACT_APP_FIRST_REQ_URL } = process.env;
    const { REACT_APP_SECOND_REQ_URL } = process.env;

    // runs when the Send button is clicked
    async function handleSubmit(specs) {
        setIsSubmit(true);

        // the specification of the wave file properties
        console.log("specs: " + JSON.stringify(specs))

        // send the specs to the first lambda 
        const data = await fetch(REACT_APP_FIRST_REQ_URL, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify(specs)
        })

        // response including a file_id and a request_id
        const response = await data.json();
        console.log(data.status)
        console.log("received data: "+ JSON.stringify(response));

        
        if (data.status === 200) {
            setReqState("loading...")
    
            const waveId = response.id;
            const reqId = response.request_id;
            let inProgress = true;
            let offset_num = 0;
            let bufferArray = [];
            
            // running as long as the file is not complete or there is no error message
            while(inProgress) {

                // inquire the file by adding the given file_id, request_id (for double validation of the sender) and the offset number of the downloading process
                const res = await fetch(`${REACT_APP_SECOND_REQ_URL}?file_id=${waveId}&offset_num=${offset_num.toString()}&request_id=${reqId}`, {
                    method: 'GET',
                    headers: {
                        'Content-Type': 'application/json'
                    },
                })
                const dataFile = await res.json();
                console.log("response: " + JSON.stringify(dataFile))

                // as long as the file is not ready yet the file is inquired every 2 seconds
                if (dataFile.body?.status === "in_progress") {
                    setReqState("loading...")
                    await new Promise(r => setTimeout(r, 2000));
                
                // if the file is ready, a 'text/plain' response is sent
                } else if (dataFile.headers && dataFile.headers["Content-Type"] === "text/plain") {
                    
                    const buffer = Buffer.from(dataFile.body, 'base64');
                    bufferArray.push(buffer);

                    // large files are downloaded in several moves, this is executed if there are still parts left
                    // Frontend is responsible for keeping count
                    if (dataFile.isLast === false) {
                        offset_num++;
                        console.log("making new request to load the whole file, offsetNum = " + offset_num)

                    // all parts get put together and a file and download link get created
                    } else {
                        const file = new File(bufferArray, waveId, {type: "audio/wav"});
                        setReqState("ready to download");
                        setDownloadUrl(window.URL.createObjectURL(file));
                        setDownloadButtonEnabled(true);

                        // break
                        inProgress = false;
                    }

                } else {
                    // in case of a bad request
                    setReqState("invalid request: " + data.status);
                    inProgress = false;
                }
            }
        } else {
            setReqState(data.statusText)
        }
    }

    const [ isSubmit, setIsSubmit ] = useState(false);
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
