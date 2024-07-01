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

    let blob = new Blob([JSON.stringify(message)], {

    });

    const encoder = new TextEncoder();
    const byteArray = encoder.encode(JSON.stringify(message));
    let array = Array.from(byteArray)
    let bytesArray = new Uint8Array(array);
    console.log('Sending message:', blob);

    socket.send(bytesArray);
});

function toUTF8Array(str: String) {
    var utf8 = [];
    for (var i = 0; i < str.length; i++) {
        var charcode = str.charCodeAt(i);
        if (charcode < 0x80) utf8.push(charcode);
        else if (charcode < 0x800) {
            utf8.push(0xc0 | (charcode >> 6), 0x80 | (charcode & 0x3f));
        } else if (charcode < 0xd800 || charcode >= 0xe000) {
            utf8.push(
                0xe0 | (charcode >> 12),
                0x80 | ((charcode >> 6) & 0x3f),
                0x80 | (charcode & 0x3f)
            );
        }
        // surrogate pair
        else {
            i++;
            // UTF-16 encodes 0x10000-0x10FFFF by
            // subtracting 0x10000 and splitting the
            // 20 bits of 0x0-0xFFFFF into two halves
            charcode =
                0x10000 +
                (((charcode & 0x3ff) << 10) | (str.charCodeAt(i) & 0x3ff));
            utf8.push(
                0xf0 | (charcode >> 18),
                0x80 | ((charcode >> 12) & 0x3f),
                0x80 | ((charcode >> 6) & 0x3f),
                0x80 | (charcode & 0x3f)
            );
        }
    }
    return utf8;
}
