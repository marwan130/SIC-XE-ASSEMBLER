import { useEffect, useState } from 'react';

export default function LoadingCoin() {
  const [dots, setDots] = useState('');

  useEffect(() => {
    const interval = setInterval(() => {
      setDots((prev) => (prev.length >= 3 ? '' : prev + '.'));
    }, 400);
    return () => clearInterval(interval);
  }, []);

  return (
    <div className="absolute inset-0 bg-black/90 flex flex-col items-center justify-center select-none z-25">
      {/* 3D Pixelated spinning coin container */}
      <div className="coin-wrapper mb-6">
        <div className="pixel-coin">
          {/* Inner details for 16-bit coin shading */}
          <div className="coin-inner" />
          <div className="coin-star" />
        </div>
      </div>

      <div className="flex flex-col items-center gap-2">
        <h3 className="font-press text-[14px] text-cyber-yellow animate-pulse tracking-wider">
          ASSEMBLING{dots}
          <span className="blinking-cursor ml-1">█</span>
        </h3>
        <p className="font-mono text-[9px] text-gray-500 uppercase tracking-widest mt-2">
          calculating LOCCTR and address offsets
        </p>
      </div>

      <style>{`
        .coin-wrapper {
          perspective: 600px;
          display: flex;
          justify-content: center;
          align-items: center;
          width: 80px;
          height: 80px;
        }

        /* Pixel Gold Coin drawn in CSS */
        .pixel-coin {
          width: 48px;
          height: 48px;
          background-color: #FFD700;
          border: 4px solid #000000;
          box-shadow: 
            inset -4px -4px 0px #FF8C00,
            inset 4px 4px 0px #FFFFE0,
            0px 0px 0px 4px #000;
          position: relative;
          transform-style: preserve-3d;
          animation: coin-spin 0.9s steps(8) infinite;
          image-rendering: pixelated;
        }

        .coin-inner {
          position: absolute;
          inset: 8px;
          border: 4px solid #000000;
          background-color: #FF8C00;
          box-shadow: inset -2px -2px 0px #B8860B, inset 2px 2px 0px #FFD700;
        }

        .coin-star {
          position: absolute;
          width: 6px;
          height: 6px;
          background-color: #FFFFFF;
          top: 14px;
          left: 17px;
          box-shadow: 0 4px 0 #FFFFFF, 4px 0 0 #FFFFFF, -4px 0 0 #FFFFFF, 0 -4px 0 #FFFFFF;
          opacity: 0.8;
        }

        @keyframes coin-spin {
          0% {
            transform: rotateY(0deg);
          }
          100% {
            transform: rotateY(360deg);
          }
        }

        .blinking-cursor {
          animation: blink-block 0.8s steps(2) infinite;
        }

        @keyframes blink-block {
          0%, 100% {
            opacity: 1;
          }
          50% {
            opacity: 0;
          }
        }
      `}</style>
    </div>
  );
}
