import { useEffect, useRef } from 'react';

interface Star {
  x: number;
  y: number;
  size: number;
  color: string;
  twinkleSpeed: number;
  twinklePhase: number;
  driftSpeed: number;
}

const PALETTE = [
  '#FFFFFF', 
  '#00FF66', 
  '#FFF500', 
  '#00E0FF', 
  '#FF007A', 
];

export default function StarField() {
  const canvasRef = useRef<HTMLCanvasElement | null>(null);

  useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas) return;

    const ctx = canvas.getContext('2d');
    if (!ctx) return;

    let animationFrameId: number;
    let stars: Star[] = [];

    const resizeCanvas = () => {
      canvas.width = window.innerWidth;
      canvas.height = window.innerHeight;
      initStars();
    };

    const initStars = () => {
      const density = 0.00015; // stars per square pixel
      const area = canvas.width * canvas.height;
      const count = Math.min(Math.floor(area * density), 300); 
      
      stars = Array.from({ length: count }, () => ({
        x: Math.random() * canvas.width,
        y: Math.random() * canvas.height,
        size: Math.random() < 0.8 ? 1.5 : 2.5, 
        color: PALETTE[Math.floor(Math.random() * PALETTE.length)],
        twinkleSpeed: 0.01 + Math.random() * 0.03,
        twinklePhase: Math.random() * Math.PI * 2,
        driftSpeed: 0.15 + Math.random() * 0.35, 
      }));
    };

    window.addEventListener('resize', resizeCanvas);
    resizeCanvas();

    const draw = () => {
      ctx.clearRect(0, 0, canvas.width, canvas.height);

      stars.forEach((star) => {
        // Update twinkling phase
        star.twinklePhase += star.twinkleSpeed;
        const opacity = 0.35 + Math.abs(Math.sin(star.twinklePhase)) * 0.65;

        // Move star upward
        star.y -= star.driftSpeed;
        if (star.y < 0) {
          star.y = canvas.height;
          star.x = Math.random() * canvas.width;
        }

        // Draw pixel star
        ctx.fillStyle = star.color;
        ctx.globalAlpha = opacity;
        ctx.fillRect(
          Math.floor(star.x),
          Math.floor(star.y),
          Math.floor(star.size),
          Math.floor(star.size)
        );
      });

      ctx.globalAlpha = 1.0; // reset
      animationFrameId = requestAnimationFrame(draw);
    };

    draw();

    return () => {
      window.removeEventListener('resize', resizeCanvas);
      cancelAnimationFrame(animationFrameId);
    };
  }, []);

  return (
    <canvas
      id="starfield-canvas"
      ref={canvasRef}
      className="fixed inset-0 w-full h-full -z-10 pointer-events-none"
    />
  );
}
