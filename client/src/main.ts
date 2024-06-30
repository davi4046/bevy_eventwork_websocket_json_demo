import './style.css';

const sendMessageButton = document.getElementById('send-message-button')!;

const socket = new WebSocket('ws://localhost:8081');

socket.addEventListener('message', async (event) => {
    const blob = event.data as Blob;
    console.log('Received message:', await blob.text());
    // Server will say hello when we join
});

sendMessageButton.addEventListener('click', (_) => {
    const data = (() => {
        const obj = {
            message: 'hello',
        };
        const string = JSON.stringify(obj);
        const encoder = new TextEncoder();
        const byteArray = encoder.encode(string);
        return Array.from(byteArray);
    })();

    const message = {
        kind: 'balls_of_steel:ChatMessage',
        data: data,
    };

    console.log('Sending message:', JSON.stringify(message));

    socket.send(JSON.stringify(message));
});
