import type { TabType } from '../../types';

interface OutputTabsProps {
  activeTab: Exclude<TabType, 'intermediate'>;
  setActiveTab: (tab: Exclude<TabType, 'intermediate'>) => void;
}

export default function OutputTabs({ activeTab, setActiveTab }: OutputTabsProps) {
  const tabs: { id: Exclude<TabType, 'intermediate'>; label: string }[] = [
    { id: 'pass1', label: 'Pass 1' },
    { id: 'symbolTable', label: 'Symbol Table' },
    { id: 'literalTable', label: 'Literal Table' },
    { id: 'objectProgram', label: 'Object Program' },
  ];

  return (
    <div className="flex flex-wrap items-end gap-1 select-none border-b-3 border-black pb-0.5">
      {tabs.map((tab) => {
        const isActive = activeTab === tab.id;

        return (
          <button
            key={tab.id}
            onClick={() => setActiveTab(tab.id)}
            className={`font-mono text-xs uppercase transition-[color,background-color,transform] border-3 border-black cursor-pointer font-bold select-none ${
              isActive
                ? 'bg-cyber-yellow text-black px-4 py-2.5 translate-y-[2px] shadow-[3px_3px_0px_#000] z-10 text-[13px]'
                : 'bg-cyber-panel text-white hover:bg-white hover:text-black px-3 py-1.5 text-[11px]'
            }`}
            style={{
              transition: 'none', 
            }}
          >
            {tab.label}
          </button>
        );
      })}
    </div>
  );
}