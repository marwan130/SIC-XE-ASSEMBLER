export default function PixelDivider() {
  return (
    <div className="relative w-2 h-full flex-shrink-0 bg-black overflow-hidden select-none pointer-events-none">
      <div className="absolute inset-0 w-full h-full neon-divider" />

      <style>{`
        @keyframes sync-cycle {
          /* Cyan Pulse */
          0% { opacity: 0; background-color: #00E0FF; box-shadow: 0 0 15px #00E0FF, 0 0 30px #00E0FF, 0 0 45px #00E0FF; }
          25% { opacity: 1; }
          50% { opacity: 0; background-color: #00E0FF; }

          /* Color Switch happens here */
          50.1% { background-color: #FF007A; }

          /* Pink Pulse */
          75% { opacity: 1; background-color: #FF007A; box-shadow: 0 0 15px #FF007A, 0 0 30px #FF007A, 0 0 45px #FF007A; }
          100% { opacity: 0; background-color: #FF007A; }
        }

        .neon-divider {
          animation: sync-cycle 10s linear infinite;
        }
      `}</style>
    </div>
  );
}