import { useRef, useState } from 'react';
import type { ChangeEvent, DragEvent } from 'react';
import { Upload, Code } from 'lucide-react';

interface CodeEditorProps {
  code: string;
  setCode: (code: string) => void;
  title: string;
  setTitle: (title: string) => void;
  onAssemble: () => void;
  isAssembling: boolean;
}

export default function CodeEditor({
  code,
  setCode,
  title,
  setTitle,
  onAssemble,
  isAssembling,
}: CodeEditorProps) {
  const fileInputRef = useRef<HTMLInputElement | null>(null);
  const [isDragOver, setIsDragOver] = useState(false);
  const editorRef = useRef<HTMLTextAreaElement>(null);
  const lineNumbersRef = useRef<HTMLDivElement>(null);

  const handleFileChange = (e: ChangeEvent<HTMLInputElement>) => {
    const file = e.target.files?.[0];
    if (!file) return;
    readFile(file);
  };

  const readFile = (file: File) => {
    const reader = new FileReader();
    reader.onload = (event) => {
      if (event.target?.result && typeof event.target.result === 'string') {
        setCode(event.target.result);
      }
    };
    reader.readAsText(file);
  };

  const handleDragOver = (e: DragEvent) => {
    e.preventDefault();
    setIsDragOver(true);
  };

  const handleDragLeave = () => {
    setIsDragOver(false);
  };

  const handleDrop = (e: DragEvent) => {
    e.preventDefault();
    setIsDragOver(false);
    const file = e.dataTransfer.files?.[0];
    if (file) {
      readFile(file);
    }
  };

  const handleScroll = () => {
    if (editorRef.current && lineNumbersRef.current) {
      lineNumbersRef.current.scrollTop = editorRef.current.scrollTop;
    }
  };

  const lineCount = code.split('\n').length;
  const lineNumbers = Array.from({ length: Math.max(lineCount, 1) }, (_, i) => i + 1);

  return (
    <div className="flex-1 flex flex-col h-full bg-transparent p-4 select-none min-w-0">
      <div className="flex flex-col gap-1 mb-3">
        <label className="font-press text-[12px] text-gray-400 uppercase tracking-wider">
          SESSION NAME
        </label>
        <input
          type="text"
          value={title}
          onChange={(e) => setTitle(e.target.value)}
          placeholder="SYS_CORE_IO"
          className="w-full bg-cyber-panel/80 backdrop-blur-xs p-3 font-mono text-[14px] text-cyber-yellow placeholder:text-gray-600 focus:outline-none focus:ring-0 focus:border-neon-green transition-colors shadow-[2px_2px_0px_#000]"
        />
      </div>

      <div className="grid grid-cols-2 gap-4 mb-4">
        <div
          onDragOver={handleDragOver}
          onDragLeave={handleDragLeave}
          onDrop={handleDrop}
          onClick={() => fileInputRef.current?.click()}
          className={`border-3 border-dashed flex flex-col items-center justify-center py-2 px-3 bg-cyber-panel/70 backdrop-blur-xs hover:bg-white/5 active:bg-white/10 transition-colors text-center cursor-pointer relative shadow-[2px_2px_0px_#000] ${
            isDragOver ? 'border-electric-blue bg-electric-blue/20' : 'border-black'
          }`}
          title="DRAG AND DROP OR CLICK TO LOAD .TXT ASSEMBLY"
        >
          <input
            type="file"
            ref={fileInputRef}
            onChange={handleFileChange}
            accept=".txt,.asm,.src"
            className="hidden"
          />
          <div className="flex items-center gap-2 text-white">
            <Upload className="w-4 h-4 text-electric-blue" />
            <span className="font-press text-[9px] tracking-tight uppercase font-bold">
              LOAD SOURCE (.TXT)
            </span>
          </div>
          <span className="font-mono text-[8px] text-gray-500 mt-1 block">
            {isDragOver ? '[ DROP FILE HERE! ]' : '[ Drag & Drop support ]'}
          </span>
        </div>

        <button
          onClick={(e) => {
            e.preventDefault();
            onAssemble();
          }}
          type="button"
          disabled={isAssembling}
          className="bg-neon-green text-black font-press text-[12px] font-bold p-3 shadow-[4px_4px_0px_#000] active:translate-x-[4px] active:translate-y-[4px] active:shadow-none transition-all cursor-pointer uppercase flex items-center justify-center gap-2"
        >
          <Code className="w-4 h-4" />
          <span>ASSEMBLE</span>
        </button>
      </div>

      <div 
        className="flex-1 flex bg-black/50 backdrop-blur-md overflow-hidden relative shadow-[inset_0_0_20px_rgba(0,0,0,0.9)]"
        style={{ minHeight: '300px' }}
      >
        <div 
          ref={lineNumbersRef}
          className="bg-black/20 text-right pr-2 pl-3 py-3 font-mono text-[12px] text-gray-600 select-none border-r border-white/5 flex flex-col overflow-hidden"
        >
          {lineNumbers.map((num) => (
            <div key={num} className="h-[21px] leading-[21px]">
              {num.toString().padStart(3, '0')}
            </div>
          ))}
        </div>

        <textarea
          ref={editorRef}
          value={code}
          onChange={(e) => setCode(e.target.value)}
          onScroll={handleScroll}
          placeholder="COPY    START   1000\nFIRST   LDX     #0\n..."
          spellCheck={false}
          className="flex-1 bg-transparent text-neon-green p-3 font-mono text-[13px] leading-[21px] outline-none border-none resize-none overflow-y-auto focus:ring-0 focus:outline-none placeholder:text-green-900/40"
          style={{
            fontFamily: '"JetBrains Mono", Courier, monospace',
            tabSize: 8,
          }}
        />
      </div>
    </div>
  );
}