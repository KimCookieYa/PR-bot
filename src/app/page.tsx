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
  const [branchName, setBranchName] = useState<string>('');
  const [review, setReview] = useState<string | null>(null);

  const selectDir = async () => {
    const result = await dialog.open({
      directory: true,
      multiple: false,
      title: 'Select your Git repository folder',
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

  const generate_code_review = async () => {
    if (!dirPath) {
      alert('Please select a directory first');
      return;
    }

    try {
      const res = await invoke<{
        error: number;
        review: string;
        success: boolean;
      }>('generate_code_review', {
        params: {
          dirPath,
          outputFile: 'diff.txt',
          branchName,
        },
      });
      console.log(res);
      setReview(res.review);
    } catch (error) {
      console.error(error);
    }
  };

  return (
    <main className={'flex min-h-screen w-full flex-col gap-y-24'}>
      <header className={'flex h-72 w-full items-center gap-x-16 p-24'}>
        <h1 className={'text-24 font-bold italic'}>Chat with AI</h1>
        <button onClick={selectDir} className={'rounded-8 border-1 p-8'}>
          Select Directory
        </button>
        {dirPath && (
          <span className={'text-blue-500 underline'}>{dirPath}</span>
        )}
      </header>
      <section id={'chat-section'}>
        <div>
          {chatLog.map((msg, i) => (
            <div key={i}>
              <strong>{msg.role}:</strong> {msg.content}
            </div>
          ))}
        </div>
        <input
          value={userInput}
          onChange={(e) => setUserInput(e.target.value)}
        />
        <button onClick={handleSend}>Send</button>
      </section>
      <section>
        <div className={'flex items-center gap-x-16'}>
          <h2 className={'text-18 font-semibold'}>코드변경사항</h2>
          <input
            value={branchName}
            onChange={(e) => setBranchName(e.target.value)}
            placeholder={'확인할 브랜치'}
          />
          <button
            onClick={generate_code_review}
            className={'rounded-8 bg-slate-500 p-8 text-white'}>
            코드변경사항에 대해 코드리뷰 생성하기
          </button>
        </div>
        {review && <pre>{review}</pre>}
      </section>
    </main>
  );
}
