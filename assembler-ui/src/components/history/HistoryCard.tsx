import type { AssemblySession } from '../../types';

interface HistoryCardProps {
  key?: string;
  session: AssemblySession;
  onSelect: () => void;
  onDelete: () => void;
}

export default function HistoryCard({ session, onSelect, onDelete }: HistoryCardProps) {
  // Format creation date 
  const dateStr = new Date(session.createdAt).toLocaleDateString('sv-SE', {
    year: 'numeric',
    month: '2-digit',
    day: '2-digit',
    hour: '2-digit',
    minute: '2-digit',
  }).replace(/-/g, '.');

  // Preview snippet of code 
  const lines = session.code.split('\n').filter(l => l.trim().length > 0).slice(0, 3);
  const snippet = lines.join('\n');

  return (
    <div 
      onClick={onSelect}
      className="bg-cyber-panel border-3 border-black p-5 relative select-none cursor-pointer transition-[color,background-color] duration-75 hover:bg-white hover:text-black hover:border-black group shadow-[4px_4px_0px_#000]"
      style={{
        transition: 'all 0.05s steps(2)',
      }}
    >
      {/* Corner Delete Button */}
      <button
        onClick={(e) => {
          e.stopPropagation(); // prevent loading workspace
          onDelete();
        }}
        className="absolute top-2 right-2 w-6 h-6 bg-transparent hover:bg-hot-pink border-2 border-transparent hover:border-black text-hot-pink hover:text-black flex items-center justify-center font-press text-[10px] font-bold z-10"
        style={{ transition: 'none' }}
        title="PURGE ASSEMBLY SNAPSHOT"
      >
        X
      </button>

      {/* Title */}
      <h3 className="font-press text-[12px] text-neon-green group-hover:text-black font-bold uppercase mb-2 tracking-tight truncate pr-6">
        {session.title}
      </h3>

      {/* Log Date */}
      <div className="font-mono text-[10px] text-gray-500 mb-3 group-hover:text-black/60">
        RECORDED: {dateStr}
      </div>

      {/* Code Snippet Box */}
      <div className="bg-[#050508] border-2 border-black/40 group-hover:border-black p-2 rounded-none overflow-hidden select-none">
        <pre className="font-mono text-[10px] text-gray-400 group-hover:text-black leading-tight overflow-hidden whitespace-pre truncate">
          {snippet || '. EMPTY FILE'}
        </pre>
      </div>

      <div className="mt-3 text-right">
        <span className="font-press text-[10px] text-cyber-yellow group-hover:text-black font-bold block">
          [ LOAD SNAPSHOT &gt;&gt; ]
        </span>
      </div>
    </div>
  );
}