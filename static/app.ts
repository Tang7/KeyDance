// Define interfaces for API responses
interface MusicRecognitionResult {
    title: string;
    artist: string;
    confidence: number;
    song_id: string;
}

interface StaffNotation {
    title: string;
    notation: string;
}

// DOM elements
let mediaRecorder: MediaRecorder | null = null;
let audioChunks: Blob[] = [];

document.addEventListener('DOMContentLoaded', function() {
    const recordButton = document.getElementById('recordButton') as HTMLButtonElement;
    const stopButton = document.getElementById('stopButton') as HTMLButtonElement;
    const statusDiv = document.getElementById('status') as HTMLDivElement;
    const resultDiv = document.getElementById('result') as HTMLDivElement;
    const titleSpan = document.getElementById('title') as HTMLSpanElement;
    const artistSpan = document.getElementById('artist') as HTMLSpanElement;
    const confidenceSpan = document.getElementById('confidence') as HTMLSpanElement;
    const confidenceBar = document.getElementById('confidenceBar') as HTMLDivElement;
    
    // Get the staff notation div
    const staffNotationDiv = document.getElementById('staffNotation');
    if (!staffNotationDiv) {
        console.error('Staff notation div not found in the HTML');
    }
    
    // Request microphone access
    async function setupRecording(): Promise<boolean> {
        try {
            const stream = await navigator.mediaDevices.getUserMedia({ audio: true });
            mediaRecorder = new MediaRecorder(stream);

            mediaRecorder.ondataavailable = (event) => {
                audioChunks.push(event.data);
            };

            mediaRecorder.onstop = async () => {
                statusDiv.textContent = 'Processing audio...';
                statusDiv.className = 'processing';

                const audioBlob = new Blob(audioChunks, { type: 'audio/wav' });
                const reader = new FileReader();

                reader.onloadend = async () => {
                    const base64Audio = (reader.result as string).split(',')[1];
                    await recognizeAudio(base64Audio);
                };

                reader.readAsDataURL(audioBlob);
            };

            return true;
        } catch (err) {
            console.error('Error accessing microphone:', err);
            statusDiv.textContent = `Error: ${err instanceof Error ? err.message : 'Unknown error'}`;
            statusDiv.className = 'error';
            return false;
        }
    }

    // Send audio to server for recognition
    async function recognizeAudio(base64Audio: string): Promise<void> {
        try {
            const response = await fetch('/api/recognize', {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json'
                },
                body: JSON.stringify({ data: base64Audio })
            });
            
            if (!response.ok) {
                const errorText = await response.text();
                throw new Error(`Server returned ${response.status}: ${errorText}`);
            }
            
            const result: MusicRecognitionResult = await response.json();
            
            // Display the result
            titleSpan.textContent = result.title;
            artistSpan.textContent = result.artist;
            
            const confidencePercent = (result.confidence * 100).toFixed(2);
            confidenceSpan.textContent = confidencePercent;
            confidenceBar.style.width = `${result.confidence * 100}%`;
            
            resultDiv.style.display = 'block';
            
            statusDiv.textContent = 'Recognition successful!';
            statusDiv.className = 'success';


            // Fetch staff notation for the song, TODO: improve this once we have a proper API
            if (result.song_id) {
                await fetchStaffNotation(result.song_id);
            } else {
                const generatedId = `${result.title.toLowerCase().replace(/ /g, "-")}-${result.artist.toLowerCase().replace(/ /g, "-")}`;
                await fetchStaffNotation(generatedId);
            }
        } catch (err) {
            console.error('Recognition error:', err);
            statusDiv.textContent = `Recognition failed: ${err instanceof Error ? err.message : 'Unknown error'}`;
            statusDiv.className = 'error';
            resultDiv.style.display = 'none';
            if (staffNotationDiv) {
                staffNotationDiv.style.display = 'none';
            }
        }
    }
    
    // Fetch staff notation for a song
    async function fetchStaffNotation(songId: string): Promise<void> {
        try {
            console.log(`Fetching staff notation for song ID: ${songId}`);
            
            // Get the staff notation div
            const staffNotationDiv = document.getElementById('staffNotation');
            if (!staffNotationDiv) {
                console.error('Staff notation div not found');
                return;
            }
            
            // Make the API call
            const response = await fetch(`/api/staff/${songId}`);
            
            if (!response.ok) {
                throw new Error(`Failed to fetch staff notation: ${response.status}`);
            }
            
            const notation: StaffNotation = await response.json();
            console.log('Received staff notation:', notation);
            
            // Display the notation
            const notationContent = document.getElementById('notationContent');
            if (notationContent) {
                console.log('Setting notation content to:', notation.notation);
                notationContent.textContent = notation.notation;
                
                // Make sure the div is visible
                staffNotationDiv.style.display = 'block';
            } else {
                console.error('notationContent element not found');
            }
        } catch (err) {
            console.error('Error fetching staff notation:', err);
            const staffNotationDiv = document.getElementById('staffNotation');
            if (staffNotationDiv) {
                staffNotationDiv.style.display = 'none';
            }
        }
    }

    // Handle record button click
    recordButton.addEventListener('click', async function() {
        if (!mediaRecorder) {
            if (!(await setupRecording())) {
                return;
            }
        }
        
        audioChunks = [];
        if (mediaRecorder) {
            mediaRecorder.start();
        }
        
        recordButton.disabled = true;
        stopButton.disabled = false;
        statusDiv.textContent = 'Recording... (capture at least 5-10 seconds of music)';
        statusDiv.className = 'recording';
        resultDiv.style.display = 'none';
        if (staffNotationDiv) {
            staffNotationDiv.style.display = 'none';
        }
    });
    
    // Handle stop button click
    stopButton.addEventListener('click', function() {
        if (mediaRecorder && mediaRecorder.state === 'recording') {
            mediaRecorder.stop();
            recordButton.disabled = false;
            stopButton.disabled = true;
        }
    });
}); 