import React, {useState} from "react";
import "./WaveForm.css"

function WaveForm(props) {

    const specsRange = {
        wav_spec: {
            number_of_channels: [1, 2],
            sample_rate: [8000, 11025, 22050, 44100],
            bits_per_sample: [8, 16]
        },
        frequency: [0, 65536],
        duration: [1, 3600],
        volume: [0, 1]
    }

    // wav_spec: {
    //     number_of_channels: integer,
    //         sample_rate: integer,
    //         bits_per_sample: integer
    // },
    // frequency: Array[int],
    // duration: float or int,
    // volume: float
    //

    const [ nrChannels, setNrChannels ] = useState(2);
    const [ sampleRate, setSampleRate ] = useState(44100);
    const [ bitsPerSample, setBitsPerSample ] = useState(8);
    const [ frequency, setFrequency ] = useState([440]);
    const [ duration, setDuration ] = useState(30);
    const [ volume, setVolume ] = useState(1);

    const [ valid, setValid ] = useState()


    function addFrequency() {
        setFrequency([...frequency, 440])
    }


    function handleSubmit(event) {
        event.preventDefault();
        const specs = {
            wav_spec: {
                number_of_channels: nrChannels,
                sample_rate: sampleRate,
                bits_per_sample: bitsPerSample
            },
            wav_data: {
                frequencies: frequency,
                duration: duration,
                volume: volume
            }
        }
        if (valid) {
            props.handleSubmit(specs)
        }
    }

    function updateFrequency(val, index) {
        let newArr = [...frequency];
        newArr[index] = val;
        setFrequency(newArr);
    }

    return (
    <form onSubmit={handleSubmit} className="WaveForm">
        <label>
            Nr. of Channels:
            <select value={nrChannels} onChange={(event => {setNrChannels(parseInt(event.target.value)); })}>
                { specsRange.wav_spec.number_of_channels.map((opt) => <option value={opt}>{opt}</option> )}
            </select>
        </label>
        <label>
            Sample Rate:

            <select value={sampleRate} onChange={(event => {setSampleRate(parseInt(event.target.value)); })}>
                { specsRange.wav_spec.sample_rate.map((opt) => <option value={opt}>{opt}</option> )}
            </select>
        </label>
        <label>
            Bits per Sample:
            <select value={bitsPerSample} onChange={(event => {setBitsPerSample(parseInt(event.target.value)); })}>
                { specsRange.wav_spec.bits_per_sample.map((opt) => <option value={opt}>{opt}</option> )}
            </select>
        </label>
        <label id="frequencies">
            Frequencies:
            { frequency.map((val, index) =>
                <input type="number" value={val} key={"input-" + index} onChange={(event => {
                    const val = parseInt(event.target.value);
                    updateFrequency(val, index);
                    if (val < specsRange.frequency[0] && val > specsRange.frequency[1]) {
                        setValid(false);
                    } else {
                        setValid(true);
                    }
                })}
                />
            )}
            <button id="addButton" onClick={(e) => addFrequency()}>+</button>
            {valid?'':<span id="error">value not in the allowed range</span>}
        </label>
        <label>
            Duration:
            <input type="number" min={specsRange.duration[0]} max={specsRange.duration[1]} step="1" id="duration" value={duration} onChange={(event => setDuration(parseInt(event.target.value)))} />
        </label>
        <label>
            Volume:
            <input type="range" min={specsRange.volume[0]} max={specsRange.volume[1]} step="0.05" id="volume" value={volume} onChange={(event => setVolume(parseFloat(event.target.value)))} />
        </label>
    <input type="submit" value="Send" id="sendButton"/>
    </form>
    )
}

export default WaveForm;
