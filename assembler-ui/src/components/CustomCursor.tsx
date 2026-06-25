import { useEffect, useState } from 'react';

interface CursorPosition {
  x: number;
  y: number;
}

export const CustomCursor = () => {
  const [position, setPosition] = useState<CursorPosition>({ x: 0, y: 0 });
  const [magneticPosition, setMagneticPosition] = useState<CursorPosition>({ x: 0, y: 0 });

  useEffect(() => {
    const handleMouseMove = (e: MouseEvent) => {
      setPosition({ x: e.clientX, y: e.clientY });
    };

    const handleMouseOver = (e: MouseEvent) => {
      const target = e.target as HTMLElement;
      if (target.closest('.magnetic')) {
        const rect = target.getBoundingClientRect();
        const centerX = rect.left + rect.width / 2;
        const centerY = rect.top + rect.height / 2;
        const distance = 20; // Magnetic radius
        const angle = Math.atan2(e.clientY - centerY, e.clientX - centerX);
        setMagneticPosition({
          x: centerX + Math.cos(angle) * distance - e.clientX,
          y: centerY + Math.sin(angle) * distance - e.clientY,
        });
      } else {
        setMagneticPosition({ x: 0, y: 0 });
      }
    };

    document.addEventListener('mousemove', handleMouseMove);
    document.addEventListener('mouseover', handleMouseOver);

    return () => {
      document.removeEventListener('mousemove', handleMouseMove);
      document.removeEventListener('mouseover', handleMouseOver);
    };
  }, []);

  return (
    <div
      className="fixed pointer-events-none z-50 mix-blend-difference"
      style={{
        left: `${position.x + magneticPosition.x}px`,
        top: `${position.y + magneticPosition.y}px`,
        transform: 'translate(-50%, -50%)',
      }}
    >
      <div
        className="w-6 h-6 border-2 border-white bg-black"
        style={{
          clipPath: 'polygon(0 0, 100% 50%, 0 100%, 20% 50%)',
        }}
      />
    </div>
  );
};
