/**
 * Sends a message to the specified endpoint.
 * @param {Array} messages - The messages to be sent.
 * @returns The response data from the server, or null in case of an error.
 */
export async function sendMessage(messages) {
  const uri = 'http://127.0.0.1:8080/api/sentmessages';
  const oaibody = {
    model: "gpt-3.5-turbo",
    messages:  messages,
  };
  // console.log('[sendMessage] Sending message:', oaibody);
  try {
    const response = await fetch(uri, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',

      },
      body: JSON.stringify(oaibody)
    });

    if (!response.ok) {
      throw new Error(`Network response was not ok! Status: ${response.status}`);
    }

    const data = await response.json();
    return data;
  } catch (err) {
    console.error('[sendMessage] Error sending message:', err);
    return null;
  }
}
