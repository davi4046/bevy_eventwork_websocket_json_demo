import './style.css';

const sendMessageButton = document.getElementById('send-message-button')!;

const socket = new WebSocket('ws://localhost:8081');

socket.addEventListener('message', async (event) => {
    const blob = event.data as Blob;
    console.log('Received message:', await blob.text());
    // Server will say hello when we join
});

sendMessageButton.addEventListener('click', async (_) => {
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

    const encoder = new TextEncoder();
    let message_string = JSON.stringify(message);
    const byteArray = encoder.encode(message_string);
    let array = Array.from(byteArray);
    let bytesArray = new Uint8Array(array);
    socket.send(bytesArray);
});
