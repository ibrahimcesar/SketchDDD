import { toPng, toSvg } from 'html-to-image';

export type ImageFormat = 'png' | 'svg';

export interface ImageExportOptions {
  format: ImageFormat;
  backgroundColor?: string;
  padding?: number;
  quality?: number;
}

const defaultOptions: ImageExportOptions = {
  format: 'png',
  backgroundColor: '#ffffff',
  padding: 20,
  quality: 1,
};

/**
 * Get the React Flow viewport element for export
 */
function getReactFlowViewport(): HTMLElement | null {
  return document.querySelector('.react-flow__viewport');
}

/**
 * Export the React Flow canvas as an image
 */
export async function exportCanvasAsImage(
  options: Partial<ImageExportOptions> = {}
): Promise<{ dataUrl: string; filename: string } | null> {
  const opts = { ...defaultOptions, ...options };
  const viewport = getReactFlowViewport();

  if (!viewport) {
    console.error('React Flow viewport not found');
    return null;
  }

  // Find the bounds of all nodes to determine the export area
  const nodeElements = viewport.querySelectorAll('.react-flow__node');
  if (nodeElements.length === 0) {
    console.error('No nodes found to export');
    return null;
  }

  const filter = (node: HTMLElement) => {
    // Exclude the minimap and controls from the export
    const className = node.className || '';
    if (typeof className === 'string') {
      return (
        !className.includes('react-flow__minimap') &&
        !className.includes('react-flow__controls') &&
        !className.includes('react-flow__panel')
      );
    }
    return true;
  };

  try {
    let dataUrl: string;
    const timestamp = new Date().toISOString().split('T')[0];
    let filename: string;

    if (opts.format === 'svg') {
      dataUrl = await toSvg(viewport, {
        filter,
        backgroundColor: opts.backgroundColor,
        style: {
          padding: `${opts.padding}px`,
        },
      });
      filename = `sketchddd-diagram-${timestamp}.svg`;
    } else {
      dataUrl = await toPng(viewport, {
        filter,
        backgroundColor: opts.backgroundColor,
        quality: opts.quality,
        pixelRatio: 2, // Higher resolution
        style: {
          padding: `${opts.padding}px`,
        },
      });
      filename = `sketchddd-diagram-${timestamp}.png`;
    }

    return { dataUrl, filename };
  } catch (error) {
    console.error('Failed to export canvas:', error);
    return null;
  }
}

/**
 * Download the exported image
 */
export function downloadImage(dataUrl: string, filename: string): void {
  const link = document.createElement('a');
  link.href = dataUrl;
  link.download = filename;
  link.click();
}

/**
 * Export and download the canvas as an image
 */
export async function exportAndDownloadCanvas(
  options: Partial<ImageExportOptions> = {}
): Promise<boolean> {
  const result = await exportCanvasAsImage(options);
  if (!result) return false;

  downloadImage(result.dataUrl, result.filename);
  return true;
}
