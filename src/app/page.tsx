'use client';

import { useState } from 'react';
import { invoke } from '@tauri-apps/api/core';

export default function Home() {
  const [userInput, setUserInput] = useState('');
  const [chatLog, setChatLog] = useState<
    { role: 'user' | 'assistant'; content: string }[]
  >([]);

  const handleSend = async () => {
    const prompt = userInput;
    setUserInput('');
    setChatLog([...chatLog, { role: 'user', content: prompt }]);
    try {
      const response = await invoke<{ text: string }>('infer_from_model', {
        prompt,
      });
      console.log(response);
      setChatLog((prev) => [
        ...prev,
        { role: 'assistant', content: response.text },
      ]);
    } catch (error) {
      console.error(error);
    }
  };

  return (
    <div>
      <div>
        {chatLog.map((msg, i) => (
          <div key={i}>
            <strong>{msg.role}:</strong> {msg.content}
          </div>
        ))}
      </div>
      <input value={userInput} onChange={(e) => setUserInput(e.target.value)} />
      <button onClick={handleSend}>Send</button>
    </div>
  );
}
