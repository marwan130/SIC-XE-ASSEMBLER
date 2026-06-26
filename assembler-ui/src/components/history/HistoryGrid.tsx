import { useState } from 'react';
import type { AssemblySession } from '../../types';
import HistoryCard from './HistoryCard';

interface HistoryGridProps {
  history: AssemblySession[];
  onSelectSession: (session: AssemblySession) => void;
  onDeleteSession: (id: string) => void;
}

export default function HistoryGrid({
  history,
  onSelectSession,
  onDeleteSession,
}: HistoryGridProps) {
  const [search, setSearch] = useState('');

  // Filter history by title search
  const filteredHistory = history.filter((session) =>
    session.title.toLowerCase().includes(search.toLowerCase())
  );

  return (
    <div className="flex-1 flex flex-col p-6 overflow-y-auto select-none max-w-7xl mx-auto w-full h-full">
      {/* Page Header */}
      <div className="flex flex-col md:flex-row justify-between items-start md:items-center gap-4 mb-8 border-b-3 border-black pb-4">
        <div>
          <h2 className="font-press text-[16px] text-cyber-yellow font-bold uppercase tracking-tight">
            SYSTEM SNAPSHOT SNAPSHOTS_LOG
          </h2>
          <p className="font-mono text-[10px] text-gray-500 uppercase mt-1">
            Browse and load historic SIC/XE assembler listings
          </p>
        </div>

        {/* Search Input */}
        <input
          type="text"
          value={search}
          onChange={(e) => setSearch(e.target.value)}
          placeholder="SEARCH SNAPSHOTS..."
          className="bg-cyber-panel border-3 border-black p-2.5 font-mono text-xs text-neon-green placeholder:text-gray-600 focus:outline-none focus:ring-0 focus:border-neon-green shadow-[2px_2px_0px_#000] w-full md:w-64 uppercase"
        />
      </div>

      {/* Grid List or Empty State */}
      {filteredHistory.length === 0 ? (
        <div className="flex-1 flex flex-col items-center justify-center p-8 text-center bg-cyber-panel/20 border-3 border-black border-dashed min-h-[350px]">
          {/* Pure CSS Pixel Art Chest */}
          <div className="pixel-chest-wrapper mb-6 animate-bounce">
            <div className="pixel-chest">
              <div className="chest-lid" />
              <div className="chest-lock" />
              <div className="chest-body" />
            </div>
          </div>
          
          <h3 className="font-press text-[12px] text-hot-pink font-bold uppercase tracking-wider mb-2">
            NO SAVES YET
          </h3>
          <p className="font-mono text-[9px] text-gray-500 uppercase tracking-widest">
            {search ? 'NO CORRESPONDING SNAPSHOTS DETECTED' : 'ASSEMBLE AND SAVE TO LOG OFFSETS'}
          </p>
        </div>
      ) : (
        <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-6">
          {filteredHistory.map((session) => (
            <HistoryCard
              key={session.id}
              session={session}
              onSelect={() => onSelectSession(session)}
              onDelete={() => onDeleteSession(session.id)}
            />
          ))}
        </div>
      )}

      {/* Styled pure CSS pixel chest details */}
      <style>{`
        .pixel-chest-wrapper {
          width: 80px;
          height: 80px;
          display: flex;
          align-items: center;
          justify-content: center;
        }

        .pixel-chest {
          width: 54px;
          height: 46px;
          background-color: #8B4513; /* brown */
          border: 4px solid #000;
          position: relative;
          box-shadow: 
            inset -4px -4px 0px #5C2E0B, 
            inset 4px 4px 0px #CD853F,
            0px 6px 0px rgba(0,0,0,0.5);
        }

        .chest-lid {
          position: absolute;
          top: 0;
          left: 0;
          right: 0;
          height: 18px;
          background-color: #A0522D;
          border-bottom: 4px solid #000;
          box-shadow: inset 4px 4px 0px #CD853F;
        }

        .chest-lock {
          position: absolute;
          width: 10px;
          height: 12px;
          background-color: #FFF500; /* gold lock */
          border: 3px solid #000;
          top: 12px;
          left: 18px;
          z-index: 10;
          box-shadow: inset -2px -2px 0px #D3CB00;
        }

        .chest-lock::after {
          content: '';
          position: absolute;
          width: 2px;
          height: 4px;
          background-color: #000;
          bottom: 1px;
          left: 1px;
        }

        .chest-body {
          position: absolute;
          bottom: 0;
          left: 0;
          right: 0;
          top: 22px;
          background-color: #8B4513;
        }
      `}</style>
    </div>
  );
}