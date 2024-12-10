'use client';

import { useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import * as dialog from '@tauri-apps/plugin-dialog';

export default function Home() {
  const [userInput, setUserInput] = useState('');
  const [chatLog, setChatLog] = useState<
    { role: 'user' | 'assistant'; content: string }[]
  >([]);

  const [dirPath, setDirPath] = useState<string | null>(null);

  const selectDir = async () => {
    const result = await dialog.open({
      directory: true,
    });
    console.log(result);
    if (result) {
      setDirPath(result);
    }
  };

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
    <main>
      <header className={'flex h-72 w-full items-center gap-x-16 p-24'}>
        <h1 className={'text-24 font-bold italic'}>Chat with AI</h1>
        <button onClick={selectDir} className={'rounded-8 border-1 p-8'}>
          Select Directory
        </button>
        {dirPath && (
          <span className={'text-blue-500 underline'}>{dirPath}</span>
        )}
      </header>
      <div>
        {chatLog.map((msg, i) => (
          <div key={i}>
            <strong>{msg.role}:</strong> {msg.content}
          </div>
        ))}
      </div>
      <input value={userInput} onChange={(e) => setUserInput(e.target.value)} />
      <button onClick={handleSend}>Send</button>
    </main>
  );
}
