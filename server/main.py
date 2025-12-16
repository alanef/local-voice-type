import os
import tempfile
from fastapi import FastAPI, File, UploadFile, HTTPException, Depends
from fastapi.security import HTTPBearer, HTTPAuthorizationCredentials
from faster_whisper import WhisperModel

app = FastAPI(title="Voice Type API", version="1.0.0")
security = HTTPBearer()

# Load Whisper model on startup
model = None

@app.on_event("startup")
async def load_model():
    global model
    model_size = os.getenv("WHISPER_MODEL", "small")
    print(f"Loading Whisper model: {model_size}")
    model = WhisperModel(model_size, device="cpu", compute_type="int8")
    print("Model loaded!")

def verify_token(credentials: HTTPAuthorizationCredentials = Depends(security)):
    """Verify the API token from Authorization header."""
    expected_token = os.getenv("API_TOKEN", "changeme")
    if credentials.credentials != expected_token:
        raise HTTPException(status_code=401, detail="Invalid API token")
    return credentials.credentials

@app.get("/v1/health")
async def health():
    """Health check endpoint."""
    return {"status": "ok", "model_loaded": model is not None}

@app.post("/v1/transcribe")
async def transcribe(
    file: UploadFile = File(...),
    language: str = "en",
    token: str = Depends(verify_token)
):
    """
    Transcribe audio file to text.

    Expects: multipart/form-data with audio file and optional language
    Returns: { "text": "transcribed text" }
    """
    if model is None:
        raise HTTPException(status_code=503, detail="Model not loaded")

    # Save uploaded file to temp location
    suffix = os.path.splitext(file.filename)[1] if file.filename else ".wav"
    with tempfile.NamedTemporaryFile(delete=False, suffix=suffix) as tmp:
        content = await file.read()
        tmp.write(content)
        tmp_path = tmp.name

    try:
        # Transcribe with specified language
        segments, _ = model.transcribe(tmp_path, language=language)
        text = " ".join(segment.text for segment in segments).strip()
        return {"text": text}
    except Exception as e:
        raise HTTPException(status_code=500, detail=f"Transcription failed: {str(e)}")
    finally:
        # Clean up temp file
        os.unlink(tmp_path)

if __name__ == "__main__":
    import uvicorn
    uvicorn.run(app, host="0.0.0.0", port=8000)
