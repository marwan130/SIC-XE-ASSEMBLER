import { useState } from 'react';
import { Download, FileArchive } from 'lucide-react';
import JSZip from 'jszip';
import type { CompiledOutputs, TabType } from '../../types';
import OutputTabs from './OutputTabs';
import LoadingCoin from './LoadingCoin';
import ErrorBanner from './ErrorBanner';

interface OutputPanelProps {
  outputs: CompiledOutputs | null;
  isLoading: boolean;
  error: string | null;
  onClearError: () => void;
  sessionTitle: string;
}

const TAB_MAPPING: Record<Exclude<TabType, 'intermediate'>, { key: keyof CompiledOutputs; filename: string }> = {
  pass1: { key: 'pass1', filename: 'pass1_locctr.txt' },
  symbolTable: { key: 'symbolTable', filename: 'symbol_table.txt' },
  literalTable: { key: 'literalTable', filename: 'literal_table.txt' },
  objectProgram: { key: 'objectProgram', filename: 'object_program.obj' },
};

export default function OutputPanel({
  outputs,
  isLoading,
  error,
  onClearError,
  sessionTitle,
}: OutputPanelProps) {
  const [activeTab, setActiveTab] = useState<Exclude<TabType, 'intermediate'>>('pass1');

  const downloadSingleFile = () => {
    if (!outputs) return;
    const tabInfo = TAB_MAPPING[activeTab];
    const content = outputs[tabInfo.key];
    const blob = new Blob([content], { type: 'text/plain;charset=utf-8' });
    const url = URL.createObjectURL(blob);
    
    const a = document.createElement('a');
    a.href = url;
    a.download = `${sessionTitle.toLowerCase()}_${tabInfo.filename}`;
    document.body.appendChild(a);
    a.click();
    document.body.removeChild(a);
    URL.revokeObjectURL(url);
  };

  const downloadAllFilesAsZip = async () => {
    if (!outputs) return;
    try {
      const zip = new JSZip();
      
      Object.keys(TAB_MAPPING).forEach((tabKey) => {
        const info = TAB_MAPPING[tabKey as Exclude<TabType, 'intermediate'>];
        zip.file(info.filename, outputs[info.key]);
      });

      const content = await zip.generateAsync({ type: 'blob' });
      const url = URL.createObjectURL(content);

      const a = document.createElement('a');
      a.href = url;
      a.download = `${sessionTitle.toLowerCase()}_assembly_records.zip`;
      document.body.appendChild(a);
      a.click();
      document.body.removeChild(a);
      URL.revokeObjectURL(url);
    } catch (err) {
      // Failed to generate archive zip
    }
  };

  const getDisplayContent = () => {
    if (!outputs) {
      return `. SYSTEM READY\n. PLACEHOLDER: WAITING FOR COMPILATION RUN\n. ENTER ASSEMBLY SOURCE AND CLICK "ASSEMBLE".`;
    }
    const key = TAB_MAPPING[activeTab].key;
    return outputs[key];
  };

  return (
    <div className="flex-1 flex flex-col h-auto md:h-full bg-transparent p-4 relative min-w-0 border-l md:border-l border-white/5 select-none">
      <div className="mb-3">
        <OutputTabs activeTab={activeTab} setActiveTab={setActiveTab as any} />
      </div>

      <div className="flex-1 bg-black/80 p-4 overflow-auto relative flex flex-col shadow-[inset_0_0_12px_rgba(0,0,0,0.9)] pixel-border custom-scrollbar">
        {isLoading && <LoadingCoin />}

        {!isLoading && (
          <pre
            className="flex-1 font-mono text-[12px] whitespace-pre text-theme-accent select-text overflow-auto leading-relaxed will-change-transform custom-scrollbar"
            style={{
              fontFamily: '"JetBrains Mono", monospace',
            }}
          >
            {getDisplayContent()}
          </pre>
        )}
      </div>

      {error && (
        <div className="mt-4">
          <ErrorBanner message={error} onClear={onClearError} />
        </div>
      )}

      <div className="mt-4 grid grid-cols-2 gap-4">
        <button
          onClick={downloadSingleFile}
          disabled={!outputs || isLoading}
          className="bg-electric-blue text-black font-press text-[10px] font-bold p-3 border-3 border-black shadow-[4px_4px_0px_#000] active:translate-x-[4px] active:translate-y-[4px] active:shadow-none transition-all cursor-pointer hover:bg-white hover:text-black uppercase flex items-center justify-center gap-2 disabled:opacity-30"
        >
          <Download className="w-4 h-4 shrink-0" />
          <span>DOWNLOAD THIS</span>
        </button>

        <button
          onClick={downloadAllFilesAsZip}
          disabled={!outputs || isLoading}
          className="bg-hot-pink text-black font-press text-[10px] font-bold p-3 border-3 border-black shadow-[4px_4px_0px_#000] active:translate-x-[4px] active:translate-y-[4px] active:shadow-none transition-all cursor-pointer hover:bg-white hover:text-black uppercase flex items-center justify-center gap-2 disabled:opacity-30"
        >
          <FileArchive className="w-4 h-4 shrink-0" />
          <span>DOWNLOAD ALL</span>
        </button>
      </div>
    </div>
  );
}