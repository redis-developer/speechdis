from transformers import Wav2Vec2Tokenizer, Wav2Vec2ForCTC
from datasets import load_dataset
import torch
import os
import shutil
import soundfile as sf
MODEL_NAME = "facebook/wav2vec2-base-960h"
MODEL_FOLDER = f"data/models/asr/{MODEL_NAME.replace('/','_')}"
OUTPUT_PATH = f"{MODEL_FOLDER}/model.onnx"


if os.path.exists(MODEL_FOLDER):
    shutil.rmtree(MODEL_FOLDER)


os.makedirs(MODEL_FOLDER)


tokenizer = Wav2Vec2Tokenizer.from_pretrained(MODEL_NAME)
model = Wav2Vec2ForCTC.from_pretrained(MODEL_NAME)
model = model.eval()

def map_to_array(batch):
    speech, _ = sf.read(batch["file"])
    batch["speech"] = speech
    return batch


ds = load_dataset("patrickvonplaten/librispeech_asr_dummy", "clean", split="validation")
ds = ds.map(map_to_array)


dummy_input = tokenizer(ds["speech"][:2], return_tensors="pt",
padding="longest").input_values


torch.onnx.export(model, dummy_input, OUTPUT_PATH, verbose=True,
input_names=["input"], output_names=["output"],
dynamic_axes={'input': {1: 'sequence'}})


tokenizer.save_pretrained(MODEL_FOLDER)