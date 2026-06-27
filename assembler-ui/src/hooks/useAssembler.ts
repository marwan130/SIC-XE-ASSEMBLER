import { useState, useEffect, useCallback } from 'react';
import { api } from '../lib/api';
import type { CompiledOutputs, AssemblySession } from '../types';

const DEFAULT_CODE = `PRROGA  START   0000
        USE     DEFAULTB
        LDA     =C'A' 
        +LDB    #RESULT
        BASE    RESULT
        ADD     WOD
        LDT     #256
        USE     DEFAULT
        USE     DEFAULT
        TIO
        MULR    A,X
        USE     DEFAULTB
        CADD    A,WOD,Z
        +LDA    GAMMA
        J       @RETADR
        USE     CDATA
        LTORG
* =C'A'
        USE     DEFAULTB
        CSUB    A,GAMMA,N
        USE     CDATA
WOD     WORD    5
GAMMA   BYTE    X'02'
        USE     DEFAULTB
        COMP    RESULT
        CLOAD   T,DATA,C
        CSTORE  T,RESULT,Z
        CJUMP   LENGTH,N
        STA     =X'07'
        USE     CDATA
DATA    WORD    5
        USE     CBLKS
BUFFER  RESB    10
RETADR  RESB    4096
RESULT  RESW    1
        USE     CDATA
LENGTH  BYTE    X'FF'
        LTORG
* =X'07'
        +LDA    GAMMA
        END     0000`;

export function useAssembler(isLoggedIn: boolean) {
  const [code, setCode] = useState<string>(() => {
    const saved = localStorage.getItem('sicxe_active_code');
    return saved !== null ? saved : DEFAULT_CODE;
  });
  
  const [sessionTitle, setSessionTitle] = useState<string>(() => {
    return localStorage.getItem('sicxe_active_title') || 'SYS_CORE_IO';
  });

  const [outputs, setOutputs] = useState<CompiledOutputs | null>(() => {
    const saved = localStorage.getItem('sicxe_active_outputs');
    return saved ? JSON.parse(saved) : null;
  });

  const [history, setHistory] = useState<AssemblySession[]>([]);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    localStorage.setItem('sicxe_active_code', code);
  }, [code]);

  useEffect(() => {
    localStorage.setItem('sicxe_active_title', sessionTitle);
  }, [sessionTitle]);

  useEffect(() => {
    if (outputs) {
      localStorage.setItem('sicxe_active_outputs', JSON.stringify(outputs));
    } else {
      localStorage.removeItem('sicxe_active_outputs');
    }
  }, [outputs]);

  // Fetch History Sessions from backend
  const fetchHistory = useCallback(async () => {
    try {
      const response = await api.get('/history');
      const jobs = response.data;
      // Map backend AssemblyJob to frontend AssemblySession
      const sessions: AssemblySession[] = jobs.map((job: any) => ({
        id: job.id,
        title: job.title,
        code: job.code,
        outputs: {
          intermediate: job.intermediate,
          pass1: job.pass1,
          symbolTable: job.symb_table,
          literalTable: job.lit_table,
          objectProgram: job.object_program,
        },
        createdAt: job.created_at,
      }));
      setHistory(sessions);
    } catch (err) {
      console.error('Failed to retrieve session history', err);
    }
  }, []);

  useEffect(() => {
    if (isLoggedIn) {
      fetchHistory();
    }
  }, [isLoggedIn, fetchHistory]);

  // Check for duplicate sessions 
  const checkForDuplicate = useCallback(() => {
    const normalizedCode = code.trim();
    const normalizedTitle = sessionTitle.trim();

    const duplicate = history.find(
      (session) =>
        session.title.trim() === normalizedTitle &&
        session.code.trim() === normalizedCode
    );

    return duplicate;
  }, [code, sessionTitle, history]);

  // Assemble Code using backend API
  const assemble = async () => {
    setLoading(true);
    setError(null);

    // Check for duplicate before assembling
    const duplicate = checkForDuplicate();
    if (duplicate) {
      setError('DUPLICATE SESSION DETECTED - SAME TITLE AND CODE ALREADY EXISTS');
      setLoading(false);
      return { success: false, error: 'DUPLICATE SESSION DETECTED - SAME TITLE AND CODE ALREADY EXISTS' };
    }

    try {
      const response = await api.post('/assemble', {
        code,
        title: sessionTitle,
      });

      const data = response.data;
      const newOutputs: CompiledOutputs = {
        intermediate: data.intermediate,
        pass1: data.pass1,
        symbolTable: data.symb_table,
        literalTable: data.lit_table,
        objectProgram: data.object_program,
      };

      setOutputs(newOutputs);

      // handle cleanly without disrupting the output panel display.
      try {
        await fetchHistory();
      } catch (historyErr) {
        console.warn('Assembly completed successfully, but history update caught auth/network error:', historyErr);
      }

      return { success: true, outputs: newOutputs };
    } catch (err: any) {
      const errMsg = err.response?.data?.error || 'COMPILATION PANIC: HEX INSTRUCTION CORRUPTED';
      setError(errMsg);
      setOutputs(null);
      return { success: false, error: errMsg };
    } finally {
      setLoading(false);
    }
  };

  // Delete Session from backend
  const deleteSession = async (id: string) => {
    try {
      await api.delete(`/history/${id}`);
      setHistory(prev => prev.filter(item => item.id !== id));
      return true;
    } catch (err) {
      console.error('Failed to purge session', err);
      return false;
    }
  };

  // Load Session into Workspace
  const loadSession = (session: AssemblySession) => {
    setCode(session.code);
    setSessionTitle(session.title);
    setOutputs(session.outputs);
    setError(null);
  };

  return {
    code,
    setCode,
    sessionTitle,
    setSessionTitle,
    outputs,
    setOutputs,
    history,
    loading,
    error,
    assemble,
    deleteSession,
    loadSession,
    refreshHistory: fetchHistory,
    clearError: () => setError(null),
  };
}