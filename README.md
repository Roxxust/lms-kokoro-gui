# kokoro-onnx needs to be [downloaded](https://huggingface.co/onnx-community/Kokoro-82M-ONNX) <br/>
from the hugging face repo. works with most versions of kokoro with the model.onnx download. (unsure about quants)  create & place in /onnx<br/>

### this is not a great/good implementation or "proper" this is a "make it up as you go wip custom implementation"<br/>
<img width="1456" height="1053" alt="image" src="https://github.com/user-attachments/assets/ef1bb317-a386-4b47-9cc0-b7f7a326bfcb" />

#### [simpletranscribe-rs](https://crates.io/crates/simple_transcribe_rs) automatically downloads a whisper model defined by tiny, base, small, medium or large in the code and paths to: /models<br/> 
tiny is fastest but can be off, small takes a little bit but is more accurate.. something's not allowing %100 cpu usage on STT, plan to change it for an onnx STT system at some point?<br/>
use ` to toggle STT<br/>

#### voice sources:<br/>
[kokoro onnx v1](https://huggingface.co/onnx-community/Kokoro-82M-v1.0-ONNX/tree/main/voices)<br/> 
[kokoro onnx v0.9](https://huggingface.co/onnx-community/Kokoro-82M-ONNX/tree/main/voices)<br/>

[cmudict source](http://www.speech.cs.cmu.edu/cgi-bin/cmudict)<br/><br/>
### **needs libclang**.<br/>
 for powershell with base msvs2022 installed with libclang pkg:<br/>
 $env:LIBCLANG_PATH = "C:\Program Files\Microsoft Visual Studio\2022\Community\VC\Tools\Llvm\x64\lib"<br/>

## features:<br/>
stt<br/>
tts<br/>
bson memory storage for context(needs to be expanded to support loading, saving etc. currently dies when app is closed but .bin retains previous message history)<br/>
settings for models<br/>
loading/unloading models<br/>
image uploading<br/>
re-sizable chat input<br/>
codeblocks<br/>
reasoning<br/>
message streaming<br/>
20+ voices<br/>

## lm studio:<br/>
start the server, load the model you want.<br/>
if no model is defined in settings it'll use the first loaded one in lm studio.<br/>

### instructions:<br/>
download the code/files or clone it into it's own folder.<br/>
download the onnx model and place it in the source/repo/kokoro/onnx (or w.e you call it)<br/>
compile the code with libclang<br/>
run the .exe or in debug<br/>
final folder structure should be:<br/>
kokoro/src<br/>
kokoro/onnx<br/>
kokoro/voices<br/>
kokoro/models<br/>
cargo.toml<br/>
etc.<br/>
 
#### License
kokoro-onnx: MIT<br/>
kokoro model: Apache 2.0<br/>
