interface RecognitionResult {
  song_id: string;
  title: string;
  artist: string;
  confidence: number;
}

document.addEventListener('DOMContentLoaded', () => {
    const recordButton = document.getElementById('recordButton') as HTMLButtonElement;
    const recordingStatus = document.getElementById('recordingStatus') as HTMLDivElement;
    const resultsDiv = document.getElementById('results') as HTMLDivElement;
    const songTitle = document.getElementById('songTitle') as HTMLSpanElement;
    const songArtist = document.getElementById('songArtist') as HTMLSpanElement;
    const confidence = document.getElementById('confidence') as HTMLSpanElement;
    const staffNotation = document.getElementById('staffNotation') as HTMLDivElement;
    
    let mediaRecorder: MediaRecorder | null = null;
    let audioChunks: Blob[] = [];
    let isRecording = false;
    
    // Request microphone access
    async function setupAudio(): Promise<boolean> {
        try {
            const stream = await navigator.mediaDevices.getUserMedia({ audio: true });
            mediaRecorder = new MediaRecorder(stream);
            
            mediaRecorder.addEventListener('dataavailable', (event: BlobEvent) => {
                audioChunks.push(event.data);
            });
            
            mediaRecorder.addEventListener('stop', async () => {
                const audioBlob = new Blob(audioChunks, { type: 'audio/wav' });
                await recognizeSong(audioBlob);
                audioChunks = [];
            });
            
            return true;
        } catch (error) {
            console.error('Error accessing microphone:', error);
            recordingStatus.textContent = 'Error: Microphone access denied';
            return false;
        }
    }
    
    // Handle record button click
    recordButton.addEventListener('click', async () => {
        if (isRecording) {
            // Stop recording
            mediaRecorder?.stop();
            recordButton.textContent = 'Record Audio';
            recordButton.classList.remove('recording');
            recordingStatus.textContent = 'Processing audio...';
            isRecording = false;
        } else {
            // Start recording
            if (!mediaRecorder && !(await setupAudio())) {
                return;
            }
            
            resultsDiv.classList.add('hidden');
            audioChunks = [];
            mediaRecorder?.start();
            recordButton.textContent = 'Stop Recording';
            recordButton.classList.add('recording');
            recordingStatus.textContent = 'Recording...';
            isRecording = true;
        }
    });
    
    // Send audio to server for recognition
    async function recognizeSong(audioBlob: Blob): Promise<void> {
        try {
            // Convert blob to base64
            const base64Audio = await blobToBase64(audioBlob);
            
            // Send to server
            const response = await fetch('/api/recognize', {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify({ data: base64Audio }),
            });
            
            if (!response.ok) {
                throw new Error('Recognition failed');
            }
            
            const result: RecognitionResult = await response.json();
            displayResults(result);
        } catch (error) {
            console.error('Error recognizing song:', error);
            recordingStatus.textContent = 'Error: Recognition failed';
        }
    }
    
    // Helper function to convert Blob to base64
    function blobToBase64(blob: Blob): Promise<string> {
        return new Promise((resolve, reject) => {
            const reader = new FileReader();
            reader.readAsDataURL(blob);
            reader.onloadend = () => {
                const base64data = reader.result as string;
                // Remove the "data:audio/wav;base64," part
                resolve(base64data.split(',')[1]);
            };
            reader.onerror = reject;
        });
    }
    
    // Display recognition results
    async function displayResults(result: RecognitionResult): Promise<void> {
        songTitle.textContent = result.title;
        songArtist.textContent = result.artist;
        confidence.textContent = Math.round(result.confidence * 100).toString();
        
        // Fetch staff notation
        try {
            const response = await fetch(`/api/staff/${result.song_id}`);
            if (!response.ok) {
                throw new Error('Failed to fetch staff notation');
            }
            
            const notationData = await response.json();
            
            // In a real app, you would render the staff notation here
            // This could use a library like VexFlow or integrate with Magenta's visualization
            staffNotation.textContent = "Staff notation would be rendered here";
            
            // Show results
            resultsDiv.classList.remove('hidden');
            recordingStatus.textContent = '';
        } catch (error) {
            console.error('Error fetching staff notation:', error);
            recordingStatus.textContent = 'Error: Could not load staff notation';
        }
    }
}); 