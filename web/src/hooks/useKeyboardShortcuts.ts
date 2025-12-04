import { useEffect } from 'react';
import { useDomainStore } from '@/stores';

export function useKeyboardShortcuts() {
  const {
    undo,
    redo,
    canUndo,
    canRedo,
    togglePalette,
    selectedNodeIds,
    selectedEdgeIds,
    activeContextId,
    deleteNode,
    deleteMorphism,
    clearSelection,
  } = useDomainStore();

  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      // Check if we're in an input field
      const target = e.target as HTMLElement;
      if (
        target.tagName === 'INPUT' ||
        target.tagName === 'TEXTAREA' ||
        target.isContentEditable
      ) {
        return;
      }

      const isMac = navigator.platform.toUpperCase().indexOf('MAC') >= 0;
      const modKey = isMac ? e.metaKey : e.ctrlKey;

      // Undo: Ctrl/Cmd + Z
      if (modKey && e.key === 'z' && !e.shiftKey) {
        e.preventDefault();
        if (canUndo()) {
          undo();
        }
        return;
      }

      // Redo: Ctrl/Cmd + Shift + Z or Ctrl/Cmd + Y
      if ((modKey && e.shiftKey && e.key === 'z') || (modKey && e.key === 'y')) {
        e.preventDefault();
        if (canRedo()) {
          redo();
        }
        return;
      }

      // Toggle Palette: Ctrl/Cmd + P
      if (modKey && e.key === 'p') {
        e.preventDefault();
        togglePalette();
        return;
      }

      // Delete: Delete or Backspace
      if ((e.key === 'Delete' || e.key === 'Backspace') && activeContextId) {
        e.preventDefault();

        // Delete selected nodes
        selectedNodeIds.forEach((nodeId) => {
          deleteNode(activeContextId, nodeId);
        });

        // Delete selected edges (morphisms)
        selectedEdgeIds.forEach((edgeId) => {
          deleteMorphism(activeContextId, edgeId);
        });

        return;
      }

      // Escape: Clear selection
      if (e.key === 'Escape') {
        e.preventDefault();
        clearSelection();
        return;
      }
    };

    window.addEventListener('keydown', handleKeyDown);
    return () => window.removeEventListener('keydown', handleKeyDown);
  }, [
    undo,
    redo,
    canUndo,
    canRedo,
    togglePalette,
    selectedNodeIds,
    selectedEdgeIds,
    activeContextId,
    deleteNode,
    deleteMorphism,
    clearSelection,
  ]);
}
