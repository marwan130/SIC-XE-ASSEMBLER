import { useRef, useState } from 'react';
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

  const readFile = (file: File) => {
    const reader = new FileReader();
    reader.onload = (event) => {
      if (event.target?.result && typeof event.target.result === 'string') {
        setCode(event.target.result);
      }
    };
    reader.readAsText(file);
  };

  const handleScroll = () => {
    if (editorRef.current && lineNumbersRef.current) {
      lineNumbersRef.current.scrollTop = editorRef.current.scrollTop;
    }
  };

  const lineCount = code.split('\n').length;
  const lineNumbers = Array.from({ length: Math.max(lineCount, 1) }, (_, i) => i + 1);

  return (
    <div className="flex-1 flex flex-col h-auto md:h-full bg-transparent p-4 select-none min-w-0">
      <div className="flex flex-col gap-1 mb-3">
        <label className="font-press text-[12px] text-gray-400 uppercase tracking-wider">
          SESSION NAME
        </label>
        <input
          type="text"
          value={title}
          onChange={(e) => setTitle(e.target.value)}
          placeholder="SYS_CORE_IO"
          className="w-full bg-black/90 p-3 font-mono text-[12px] text-cyber-yellow placeholder:text-gray-600 focus:outline-none focus:ring-0 focus:border-theme-accent transition-colors shadow-[2px_2px_0px_#000]"
        />
      </div>

      <div className="grid grid-cols-2 gap-4 mb-4">
        <div
          onDragOver={(e) => { e.preventDefault(); setIsDragOver(true); }}
          onDragLeave={() => setIsDragOver(false)}
          onDrop={(e) => { e.preventDefault(); setIsDragOver(false); const f = e.dataTransfer.files?.[0]; if(f) readFile(f); }}
          onClick={() => fileInputRef.current?.click()}
          className={`border-3 border-dashed flex flex-col items-center justify-center py-2 px-3 bg-black/80 hover:bg-black/60 transition-colors text-center cursor-pointer relative shadow-[2px_2px_0px_#000] ${
            isDragOver ? 'border-electric-blue bg-electric-blue/20' : 'border-black'
          }`}
        >
          <input 
            type="file" 
            ref={fileInputRef} 
            onChange={(e) => e.target.files?.[0] && readFile(e.target.files[0])} 
            accept=".txt,.asm,.src" 
            className="hidden" 
          />
          <div className="flex items-center gap-2 text-white">
            <Upload className="w-4 h-4 text-electric-blue" />
            <span className="font-press text-[10px] tracking-tight uppercase font-bold">LOAD SOURCE</span>
          </div>
        </div>

        <button
          type="button"
          onClick={(e) => { 
            e.preventDefault(); 
            e.stopPropagation();
            onAssemble(); 
          }}
          disabled={isAssembling}
          className="bg-theme-accent text-black font-press text-[12px] font-bold p-3 shadow-[4px_4px_0px_#000] active:translate-x-[4px] active:translate-y-[4px] active:shadow-none transition-all cursor-pointer uppercase flex items-center justify-center gap-2"
        >
          <Code className="w-4 h-4" />
          <span>{isAssembling ? 'ASSEMBLING...' : 'ASSEMBLE'}</span>
        </button>
      </div>

      <div 
        className="flex-1 flex bg-black/90 overflow-hidden relative shadow-[inset_0_0_20px_rgba(0,0,0,0.9)]"
        style={{ minHeight: '300px' }}
      >
        <div 
          ref={lineNumbersRef}
          className="bg-black/40 text-right pr-2 pl-3 py-3 font-mono text-[12px] text-gray-600 select-none border-r border-white/5 flex flex-col overflow-hidden"
        >
          {lineNumbers.map((num) => (
            <div key={num} className="h-[21px] leading-[21px]">{num.toString().padStart(3, '0')}</div>
          ))}
        </div>

        <textarea
          ref={editorRef}
          value={code}
          onChange={(e) => setCode(e.target.value)}
          onScroll={handleScroll}
          spellCheck={false}
          className="flex-1 bg-transparent text-neon-green p-3 font-mono text-[13px] leading-[21px] outline-none border-none resize-none overflow-y-auto will-change-transform custom-scrollbar"
          style={{ fontFamily: '"JetBrains Mono", Courier, monospace', tabSize: 8 }}
        />
      </div>
    </div>
  );
}