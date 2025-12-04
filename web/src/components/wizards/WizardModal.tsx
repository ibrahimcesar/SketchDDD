import { useState } from 'react';
import type { ReactNode } from 'react';
import { X, ChevronLeft, ChevronRight, Check } from 'lucide-react';

export interface WizardStep {
  title: string;
  description?: string;
  content: ReactNode;
  isValid?: boolean;
}

export interface WizardModalProps {
  title: string;
  steps: WizardStep[];
  isOpen: boolean;
  onClose: () => void;
  onFinish: () => void;
}

export function WizardModal({ title, steps, isOpen, onClose, onFinish }: WizardModalProps) {
  const [currentStep, setCurrentStep] = useState(0);

  if (!isOpen) return null;

  const step = steps[currentStep];
  const isFirstStep = currentStep === 0;
  const isLastStep = currentStep === steps.length - 1;
  const canProceed = step.isValid !== false;

  const handleNext = () => {
    if (isLastStep) {
      onFinish();
      onClose();
      setCurrentStep(0);
    } else {
      setCurrentStep((prev) => prev + 1);
    }
  };

  const handleBack = () => {
    setCurrentStep((prev) => Math.max(0, prev - 1));
  };

  const handleClose = () => {
    onClose();
    setCurrentStep(0);
  };

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center">
      {/* Backdrop */}
      <div
        className="absolute inset-0 bg-black/50"
        onClick={handleClose}
      />

      {/* Modal */}
      <div className="relative bg-white dark:bg-slate-800 rounded-xl shadow-2xl w-full max-w-lg mx-4 overflow-hidden">
        {/* Header */}
        <div className="flex items-center justify-between px-6 py-4 border-b border-slate-200 dark:border-slate-700">
          <h2 className="text-lg font-semibold">{title}</h2>
          <button
            onClick={handleClose}
            className="p-1 rounded hover:bg-slate-100 dark:hover:bg-slate-700"
          >
            <X className="w-5 h-5" />
          </button>
        </div>

        {/* Progress */}
        <div className="px-6 py-3 bg-slate-50 dark:bg-slate-900/50">
          <div className="flex items-center gap-2">
            {steps.map((_, i) => (
              <div key={i} className="flex items-center gap-2">
                <div
                  className={`
                    w-8 h-8 rounded-full flex items-center justify-center text-sm font-medium
                    ${i < currentStep
                      ? 'bg-primary text-white'
                      : i === currentStep
                      ? 'bg-primary text-white'
                      : 'bg-slate-200 dark:bg-slate-700 text-slate-500'
                    }
                  `}
                >
                  {i < currentStep ? <Check className="w-4 h-4" /> : i + 1}
                </div>
                {i < steps.length - 1 && (
                  <div
                    className={`w-8 h-0.5 ${
                      i < currentStep ? 'bg-primary' : 'bg-slate-200 dark:bg-slate-700'
                    }`}
                  />
                )}
              </div>
            ))}
          </div>
          <div className="mt-2 text-sm font-medium">{step.title}</div>
          {step.description && (
            <div className="text-xs text-slate-500 dark:text-slate-400 mt-0.5">
              {step.description}
            </div>
          )}
        </div>

        {/* Content */}
        <div className="px-6 py-4 min-h-[200px]">
          {step.content}
        </div>

        {/* Footer */}
        <div className="flex items-center justify-between px-6 py-4 border-t border-slate-200 dark:border-slate-700 bg-slate-50 dark:bg-slate-900/50">
          <button
            onClick={handleBack}
            disabled={isFirstStep}
            className={`
              flex items-center gap-2 px-4 py-2 rounded-lg text-sm font-medium
              ${isFirstStep
                ? 'text-slate-400 cursor-not-allowed'
                : 'text-slate-700 dark:text-slate-300 hover:bg-slate-100 dark:hover:bg-slate-700'
              }
            `}
          >
            <ChevronLeft className="w-4 h-4" />
            Back
          </button>

          <button
            onClick={handleNext}
            disabled={!canProceed}
            className={`
              flex items-center gap-2 px-4 py-2 rounded-lg text-sm font-medium
              ${canProceed
                ? 'bg-primary text-white hover:bg-primary-hover'
                : 'bg-slate-300 text-slate-500 cursor-not-allowed'
              }
            `}
          >
            {isLastStep ? 'Finish' : 'Next'}
            {!isLastStep && <ChevronRight className="w-4 h-4" />}
          </button>
        </div>
      </div>
    </div>
  );
}
