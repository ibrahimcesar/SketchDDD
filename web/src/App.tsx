import { useState, useEffect } from 'react';
import { Header, Sidebar } from '@/components/layout';
import { Palette } from '@/components/palette';
import { Canvas } from '@/components/canvas';
import { PropertiesPanel } from '@/components/panels';
import { ValidationPanel } from '@/components/validation';
import { ContextMapView } from '@/components/contextmap';
import { useDomainStore } from '@/stores';
import { useKeyboardShortcuts } from '@/hooks';
import { initWasm } from '@/wasm';

function App() {
  const [showValidation, setShowValidation] = useState(false);
  const [showContextMap, setShowContextMap] = useState(false);
  const [_wasmReady, setWasmReady] = useState(false);
  const { setActiveContext } = useDomainStore();

  // Initialize WASM module
  useEffect(() => {
    initWasm()
      .then(() => {
        setWasmReady(true);
        console.log('SketchDDD WASM module initialized');
      })
      .catch((err: Error) => {
        console.error('Failed to initialize WASM:', err);
      });
  }, []);

  // Enable global keyboard shortcuts
  useKeyboardShortcuts();

  return (
    <div className="h-screen flex flex-col bg-slate-50 dark:bg-slate-900">
      <Header
        onValidationToggle={() => setShowValidation(!showValidation)}
        onContextMapToggle={() => setShowContextMap(true)}
      />
      <div className="flex-1 flex overflow-hidden">
        <Sidebar />
        <Palette />
        <Canvas />
        <PropertiesPanel />
        <ValidationPanel
          isOpen={showValidation}
          onClose={() => setShowValidation(false)}
        />
      </div>

      {/* Context Map View (full screen overlay) */}
      <ContextMapView
        isOpen={showContextMap}
        onClose={() => setShowContextMap(false)}
        onContextSelect={(contextId) => {
          setActiveContext(contextId);
          setShowContextMap(false);
        }}
      />
    </div>
  );
}

export default App;
