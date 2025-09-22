# kokoro-onnx needs to be [downloaded](https://huggingface.co/onnx-community/Kokoro-82M-ONNX) <br/>
from the hugging face repo. works with most versions of kokoro with the model.onnx download.  create & place in /onnx<br/>

### this is not a great/good implementation or "proper" this is a "make it up as you go wip custom implementation"<br/>

#### [simpletranscribe-rs](https://crates.io/crates/simple_transcribe_rs) automatically downloads a whisper model defined by tiny,base, small, medium, large in /models<br/> 
tiny is fastest but can be off, small takes a little bit but is more accurate.. something's not allowing %100 cpu usage on STT, plan to change it for an onnx STT system at some point?

#### voice sources:<br/>
[kokoro onnx v1](https://huggingface.co/onnx-community/Kokoro-82M-v1.0-ONNX/tree/main/voices)<br/> 
[kokoro onnx v0.9](https://huggingface.co/onnx-community/Kokoro-82M-ONNX/tree/main/voices)<br/>

[cmudict source](http://www.speech.cs.cmu.edu/cgi-bin/cmudict)<br/><br/>
### **needs libclang**.<br/>
 for powershell with base msvs2022 installed with libclang pkg:<br/>
 $env:LIBCLANG_PATH = "C:\Program Files\Microsoft Visual Studio\2022\Community\VC\Tools\Llvm\x64\lib"<br/>
