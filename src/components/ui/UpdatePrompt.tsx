import React from 'react';

export const UpdatePrompt = ({ onDismiss }: { onDismiss: () => void }) => {
  return (
    <div className="fixed bottom-4 right-4 bg-blue-500 text-white rounded-lg shadow-xl p-4 flex items-center gap-4 z-50">
      <span>A new version is available!</span>
      <button className="font-bold hover:underline">
        Update Now
      </button>
      <button onClick={onDismiss} className="font-bold text-lg leading-none">
        &times;
      </button>
    </div>
  );
}; 