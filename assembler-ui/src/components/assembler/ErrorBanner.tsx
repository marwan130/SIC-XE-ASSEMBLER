import { AlertTriangle } from 'lucide-react';

interface ErrorBannerProps {
  message: string;
  onClear: () => void;
}

export default function ErrorBanner({ message, onClear }: ErrorBannerProps) {
  return (
    <div className="border-3 border-black bg-[#1A0A0A] p-4 flex flex-col gap-3 relative overflow-hidden shadow-[4px_4px_0px_#FF4444] mb-4">
      {/* Broken Neon Sign Flickering Header */}
      <div className="flex items-center gap-2 border-b-2 border-black pb-2 neon-flicker">
        <AlertTriangle className="w-5 h-5 text-[#FF4444] shrink-0" />
        <h4 className="font-press text-[10px] text-white font-bold uppercase tracking-wider">
          ASSEMBLY FAILED!
        </h4>
        <button 
          onClick={onClear}
          className="ml-auto font-press text-[8px] text-gray-500 hover:text-white border-2 border-transparent hover:border-black hover:bg-black p-1 transition-all"
          title="CLEAR TERMINAL ERROR"
        >
          [ DISMISS ]
        </button>
      </div>

      {/* Compiler Error Description */}
      <div className="max-h-[160px] overflow-y-auto pr-1">
        <pre className="font-mono text-[12px] text-[#FF9999] whitespace-pre-wrap leading-relaxed select-text">
          {message}
        </pre>
      </div>

      <style>{`
        .neon-flicker {
          animation: flicker-neon 2.5s infinite;
        }

        @keyframes flicker-neon {
          0%, 19%, 21%, 23%, 25%, 54%, 56%, 100% {
            opacity: 1;
            filter: drop-shadow(0 0 4px rgba(255, 68, 68, 0.6));
          }
          20%, 24%, 55% {
            opacity: 0.35;
            filter: none;
          }
          22% {
            opacity: 0.8;
            filter: drop-shadow(0 0 2px rgba(255, 68, 68, 0.4));
          }
        }
      `}</style>
    </div>
  );
}
