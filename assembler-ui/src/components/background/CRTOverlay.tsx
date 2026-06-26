export default function CRTOverlay() {
  return (
    <div className="fixed inset-0 w-full h-full pointer-events-none z-10 overflow-hidden">
      {/* Repeating horizontal scanlines */}
      <div className="absolute inset-0 w-full h-full crt-scanlines opacity-[0.06]" />

      {/* Slowly scrolling horizontal scan shadow line */}
      <div 
        className="absolute w-full h-1/3 bg-gradient-to-b from-transparent via-white/[0.02] to-transparent pointer-events-none"
        style={{
          animation: 'crt-flicker 12s linear infinite',
          willChange: 'transform',
        }}
      />

      <style>{`
        @keyframes crt-flicker {
          0% {
            transform: translateY(-100%);
          }
          100% {
            transform: translateY(300%);
          }
        }
      `}</style>
    </div>
  );
}
