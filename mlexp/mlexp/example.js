import * as tf from '@tensorflow/tfjs-node';
import fs from 'fs';
import * as ort from "onnxruntime";
import path from 'path';
export class ASR
{
private _session?: ort.InferenceSession
private tokToId: { [key: string]: number }
private idToTok: { [key: number]: string } = {}


constructor(private modelPath: string)
{
this.tokToId = JSON.parse(fs.readFileSync(path.join(this.modelPath, 'vocab.json'), 'utf-8'))
for (const [key, value] of Object.entries(this.tokToId)) { this.idToTok[value] = key }
}


async load() { this._session = await ort.InferenceSession.create(path.join(this.modelPath, 'model.onnx')) }


decodeIds(ids: number[]): string
{
return ids.slice(0, Math.floor(ids.length / 2)).map(i => this.idToTok[i]).filter(t => t !== '').join("_")
}


async asr(vector: number[]): Promise
{
const input_ids = new ort.Tensor("float32", vector.concat(vector), [2, vector.length])
const results = await this._session!.run({ input: input_ids });
const outTens = tf.tensor(results.output.data as Float32Array).reshape(results.output.dims as number[])
const ids = tf.argMax(outTens, -1)
return this.decodeIds(Array.from(ids.dataSync()))
}
}


const test = async () =>
{
const asr = new ASR(path.join(__dirname, '../models/asr/facebook_wav2vec2-base-960h'))
const data = JSON.parse(fs.readFileSync(path.join(__dirname, '../models/asr/test-mic.json'), 'utf-8'))
await asr.load()
const res = await asr.asr(data.data)
console.log(res)
}
test().then(() => { console.log("Done") })


// Gives : I|||THHINK|||IIT'SS|||A|||BEEAAUTIFUULL|||DAY|||TO|DAY|||