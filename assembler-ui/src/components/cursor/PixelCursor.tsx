import { useEffect } from 'react';

const TEXT_FIELD_SELECTOR = `
  input:not([type='button']):not([type='submit']):not([type='checkbox']):not([type='radio']),
  textarea, [contenteditable='true']
`;

const CLICKABLE_SELECTOR = `
  a, button, select, label, summary, [role="button"], [onclick],
  input[type="button"], input[type="submit"], input[type="checkbox"], input[type="radio"],
  .interactive-cursor, .neo-button
`;

// Mickey design cursor
const MICKEY_OPEN_SVG = `
<svg width="28" height="28" viewBox="0 0 32 32" fill="none" xmlns="http://www.w3.org/2000/svg">
  <path d="M12 2C9.5 2 8 3.5 8 6C8 7.5 8.5 8.5 9 9C8 9 7 10 7 12C7 13.5 7.5 14 8 14.5C7 15 6 16 6 18C6 20.5 7.5 22 10 22H14C17 22 23 21 24.5 17C25 15.5 25 13.5 24.5 12C24 10 22 8.5 19.5 8.5C19 8.5 18 8.7 17.5 9C17 8 15.5 7 14 7C13.5 7 12.8 7.2 12.3 7.5C12.1 5.5 12.5 2 12 2Z" fill="black" />
  <path d="M11.5 3C10.5 3 9.5 3.8 9.5 5.5C9.5 7.5 11 8.5 11.5 9H10C9 9 8.5 9.8 8.5 11.5C8.5 13 9.5 13.5 10 13.8H9C8 13.8 7.5 14.5 7.5 17C7.5 19.5 8.5 20.5 10.5 20.5H14C16.5 20.5 21.5 19.8 23 16.5C23.5 15 23.5 13.5 23 12C22.5 10.5 21 9.5 19 9.5C18 9.5 17 10 16 11V7.5C16 6.5 15 5.5 14 5.5C13.5 5.5 12.5 6 12 7V3.5C12 3.2 11.8 3 11.5 3Z" fill="white" />
  <rect x="10" y="21" width="10" height="4" rx="2" fill="white" stroke="black" strokeWidth="1.5" />
  <rect x="11" y="24" width="8" height="4" rx="1" fill="#E0E0E0" stroke="black" strokeWidth="1" />
  <path d="M13 13V17" stroke="black" strokeWidth="1.5" strokeLinecap="round" />
  <path d="M15.5 13V17.5" stroke="black" strokeWidth="1.5" strokeLinecap="round" />
  <path d="M18 13V17" stroke="black" strokeWidth="1.5" strokeLinecap="round" />
</svg>`;

// Cyber target cursor
const CYBER_TARGET_SVG = `
<svg width="32" height="32" viewBox="0 0 32 32" fill="none" xmlns="http://www.w3.org/2000/svg">
  <path d="M14 6H18V10H14V6ZM14 22H18V26H14V22ZM6 14H10V18H6V14ZM22 14H26V18H22V14Z" fill="black"/>
  <rect x="15" y="4" width="2" height="6" fill="#00E0FF"/>
  <rect x="15" y="22" width="2" height="6" fill="#00E0FF"/>
  <rect x="4" y="15" width="6" height="2" fill="#00E0FF"/>
  <rect x="22" y="15" width="6" height="2" fill="#00E0FF"/>
  <rect x="14" y="14" width="4" height="4" fill="black"/>
  <rect x="15" y="15" width="2" height="2" fill="white"/>
</svg>`;

const createDataUri = (svgStr: string) => `url("data:image/svg+xml,${encodeURIComponent(svgStr.trim())}")`;

export default function PixelCursor() {
  useEffect(() => {
    const mq = window.matchMedia('(pointer: fine)');
    if (!mq.matches) return;

    const mickeyCursorUri = createDataUri(MICKEY_OPEN_SVG);
    const targetCursorUri = createDataUri(CYBER_TARGET_SVG);

    const styleNode = document.createElement('style');
    styleNode.innerHTML = `
      @media (pointer: fine) {
        html, body {
          cursor: ${mickeyCursorUri} 16 16, auto !important;
        }
        ${CLICKABLE_SELECTOR} {
          cursor: ${targetCursorUri} 16 16, pointer !important;
        }
        ${TEXT_FIELD_SELECTOR} {
          cursor: text !important;
        }
      }
    `;
    document.head.appendChild(styleNode);

    return () => {
      styleNode.remove();
    };
  }, []);

  return null;
}