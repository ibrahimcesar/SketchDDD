import { useState, useEffect } from 'react';
import { X, Moon, Sun, Monitor, Trash2, RotateCcw } from 'lucide-react';
import { useDomainStore } from '@/stores';

type Theme = 'light' | 'dark' | 'system';

interface SettingsPanelProps {
  isOpen: boolean;
  onClose: () => void;
}

export function SettingsPanel({ isOpen, onClose }: SettingsPanelProps) {
  const { reset } = useDomainStore();
  const [theme, setTheme] = useState<Theme>(() => {
    return (localStorage.getItem('sketchddd-theme') as Theme) || 'system';
  });
  const [showResetConfirm, setShowResetConfirm] = useState(false);

  useEffect(() => {
    const root = document.documentElement;

    if (theme === 'system') {
      const prefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
      root.classList.toggle('dark', prefersDark);
    } else {
      root.classList.toggle('dark', theme === 'dark');
    }

    localStorage.setItem('sketchddd-theme', theme);
  }, [theme]);

  // Listen for system theme changes
  useEffect(() => {
    if (theme !== 'system') return;

    const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)');
    const handler = (e: MediaQueryListEvent) => {
      document.documentElement.classList.toggle('dark', e.matches);
    };

    mediaQuery.addEventListener('change', handler);
    return () => mediaQuery.removeEventListener('change', handler);
  }, [theme]);

  const handleReset = () => {
    reset();
    setShowResetConfirm(false);
    onClose();
  };

  const handleClearStorage = () => {
    localStorage.removeItem('sketchddd-domain-store');
    window.location.reload();
  };

  if (!isOpen) return null;

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/50">
      <div className="bg-white dark:bg-slate-800 rounded-lg shadow-xl w-full max-w-md">
        {/* Header */}
        <div className="flex items-center justify-between p-4 border-b border-slate-200 dark:border-slate-700">
          <h2 className="text-lg font-semibold">Settings</h2>
          <button
            onClick={onClose}
            className="p-1 rounded hover:bg-slate-100 dark:hover:bg-slate-700"
          >
            <X className="w-5 h-5" />
          </button>
        </div>

        {/* Content */}
        <div className="p-4 space-y-6">
          {/* Theme Section */}
          <div>
            <h3 className="text-sm font-medium text-slate-700 dark:text-slate-300 mb-3">
              Appearance
            </h3>
            <div className="flex gap-2">
              <button
                onClick={() => setTheme('light')}
                className={`flex-1 flex items-center justify-center gap-2 p-3 rounded-lg border ${
                  theme === 'light'
                    ? 'border-primary bg-primary/10 text-primary'
                    : 'border-slate-200 dark:border-slate-600 hover:bg-slate-50 dark:hover:bg-slate-700'
                }`}
              >
                <Sun className="w-4 h-4" />
                <span className="text-sm">Light</span>
              </button>
              <button
                onClick={() => setTheme('dark')}
                className={`flex-1 flex items-center justify-center gap-2 p-3 rounded-lg border ${
                  theme === 'dark'
                    ? 'border-primary bg-primary/10 text-primary'
                    : 'border-slate-200 dark:border-slate-600 hover:bg-slate-50 dark:hover:bg-slate-700'
                }`}
              >
                <Moon className="w-4 h-4" />
                <span className="text-sm">Dark</span>
              </button>
              <button
                onClick={() => setTheme('system')}
                className={`flex-1 flex items-center justify-center gap-2 p-3 rounded-lg border ${
                  theme === 'system'
                    ? 'border-primary bg-primary/10 text-primary'
                    : 'border-slate-200 dark:border-slate-600 hover:bg-slate-50 dark:hover:bg-slate-700'
                }`}
              >
                <Monitor className="w-4 h-4" />
                <span className="text-sm">System</span>
              </button>
            </div>
          </div>

          {/* Data Section */}
          <div>
            <h3 className="text-sm font-medium text-slate-700 dark:text-slate-300 mb-3">
              Data Management
            </h3>
            <div className="space-y-2">
              <button
                onClick={() => setShowResetConfirm(true)}
                className="w-full flex items-center gap-3 p-3 rounded-lg border border-slate-200 dark:border-slate-600 hover:bg-slate-50 dark:hover:bg-slate-700 text-left"
              >
                <RotateCcw className="w-4 h-4 text-slate-500" />
                <div>
                  <div className="text-sm font-medium">Reset Canvas</div>
                  <div className="text-xs text-slate-500 dark:text-slate-400">
                    Clear all nodes and morphisms from the current model
                  </div>
                </div>
              </button>
              <button
                onClick={handleClearStorage}
                className="w-full flex items-center gap-3 p-3 rounded-lg border border-red-200 dark:border-red-900 hover:bg-red-50 dark:hover:bg-red-900/20 text-left"
              >
                <Trash2 className="w-4 h-4 text-red-500" />
                <div>
                  <div className="text-sm font-medium text-red-600 dark:text-red-400">
                    Clear All Data
                  </div>
                  <div className="text-xs text-slate-500 dark:text-slate-400">
                    Remove all saved data and reload the application
                  </div>
                </div>
              </button>
            </div>
          </div>

          {/* Keyboard Shortcuts */}
          <div>
            <h3 className="text-sm font-medium text-slate-700 dark:text-slate-300 mb-3">
              Keyboard Shortcuts
            </h3>
            <div className="space-y-1 text-sm">
              <div className="flex justify-between py-1">
                <span className="text-slate-600 dark:text-slate-400">Undo</span>
                <kbd className="px-2 py-0.5 bg-slate-100 dark:bg-slate-700 rounded text-xs">
                  Ctrl+Z
                </kbd>
              </div>
              <div className="flex justify-between py-1">
                <span className="text-slate-600 dark:text-slate-400">Redo</span>
                <kbd className="px-2 py-0.5 bg-slate-100 dark:bg-slate-700 rounded text-xs">
                  Ctrl+Shift+Z
                </kbd>
              </div>
              <div className="flex justify-between py-1">
                <span className="text-slate-600 dark:text-slate-400">Delete Selected</span>
                <kbd className="px-2 py-0.5 bg-slate-100 dark:bg-slate-700 rounded text-xs">
                  Delete
                </kbd>
              </div>
              <div className="flex justify-between py-1">
                <span className="text-slate-600 dark:text-slate-400">Toggle Palette</span>
                <kbd className="px-2 py-0.5 bg-slate-100 dark:bg-slate-700 rounded text-xs">
                  Ctrl+P
                </kbd>
              </div>
            </div>
          </div>
        </div>

        {/* Footer */}
        <div className="p-4 border-t border-slate-200 dark:border-slate-700 text-center text-xs text-slate-500 dark:text-slate-400">
          SketchDDD v1.0.0
        </div>
      </div>

      {/* Reset Confirmation Dialog */}
      {showResetConfirm && (
        <div className="fixed inset-0 z-60 flex items-center justify-center bg-black/50">
          <div className="bg-white dark:bg-slate-800 rounded-lg shadow-xl p-6 max-w-sm">
            <h3 className="text-lg font-semibold mb-2">Reset Canvas?</h3>
            <p className="text-sm text-slate-600 dark:text-slate-400 mb-4">
              This will clear all contexts, nodes, and relationships. This action cannot be undone.
            </p>
            <div className="flex gap-2 justify-end">
              <button
                onClick={() => setShowResetConfirm(false)}
                className="px-4 py-2 text-sm rounded border border-slate-200 dark:border-slate-600 hover:bg-slate-50 dark:hover:bg-slate-700"
              >
                Cancel
              </button>
              <button
                onClick={handleReset}
                className="px-4 py-2 text-sm rounded bg-red-500 text-white hover:bg-red-600"
              >
                Reset
              </button>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}
