export interface CompiledOutputs {
  intermediate: string;
  pass1: string;
  symbolTable: string;
  literalTable: string;
  objectProgram: string;
}

export interface AssemblySession {
  id: string;
  title: string;
  code: string;
  outputs: CompiledOutputs | null;
  createdAt: string;
}

export type TabType = 
  | 'intermediate'
  | 'pass1'
  | 'symbolTable'
  | 'literalTable'
  | 'objectProgram';

export type TerminalTheme = 'neon' | 'cyber' | 'pink';