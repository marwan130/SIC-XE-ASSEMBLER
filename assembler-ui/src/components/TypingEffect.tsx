import { useEffect, useState } from 'react';

interface TypingEffectProps {
  text: string;
  speed?: number;
  className?: string;
}

export const TypingEffect = ({ text, speed = 50, className = '' }: TypingEffectProps) => {
  const [displayedText, setDisplayedText] = useState('');
  const [currentIndex, setCurrentIndex] = useState(0);

  useEffect(() => {
    if (currentIndex < text.length) {
      const timeout = setTimeout(() => {
        setDisplayedText((prev) => prev + text[currentIndex]);
        setCurrentIndex((prev) => prev + 1);
      }, speed);
      return () => clearTimeout(timeout);
    }
  }, [currentIndex, text, speed]);

  return (
    <span className={`font-mono ${className}`}>
      {displayedText}
      <span className="animate-pulse">_</span>
    </span>
  );
};