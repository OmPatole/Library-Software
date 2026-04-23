// ─── Shared UI Primitives ──────────────────────────────────────────────────────
import { ReactNode } from "react";

// Field label + input wrapper
export const Field = ({ label, children }: { label: string; children: ReactNode }) => (
  <div className="flex flex-col gap-1">
    <label className="text-[0.65rem] font-semibold uppercase tracking-wider text-gray-400">
      {label}
    </label>
    {children}
  </div>
);

// Standard text input
export const Input = (props: React.InputHTMLAttributes<HTMLInputElement>) => (
  <input
    {...props}
    className={`w-full bg-gray-800 border border-gray-700 text-gray-100 text-xs
      rounded px-2.5 py-1.5 placeholder-gray-600
      focus:outline-none focus:border-blue-500 focus:ring-1 focus:ring-blue-500/40
      disabled:opacity-50 disabled:cursor-not-allowed transition-colors
      ${props.className ?? ""}`}
  />
);

// Standard select
export const Select = (
  props: React.SelectHTMLAttributes<HTMLSelectElement> & { children: ReactNode }
) => (
  <select
    {...props}
    className={`w-full bg-gray-800 border border-gray-700 text-gray-100 text-xs
      rounded px-2.5 py-1.5 focus:outline-none focus:border-blue-500
      focus:ring-1 focus:ring-blue-500/40 disabled:opacity-50 transition-colors
      ${props.className ?? ""}`}
  />
);

// Textarea
export const Textarea = (props: React.TextareaHTMLAttributes<HTMLTextAreaElement>) => (
  <textarea
    rows={3}
    {...props}
    className={`w-full bg-gray-800 border border-gray-700 text-gray-100 text-xs
      rounded px-2.5 py-1.5 placeholder-gray-600 resize-none
      focus:outline-none focus:border-blue-500 focus:ring-1 focus:ring-blue-500/40
      transition-colors ${props.className ?? ""}`}
  />
);

// Primary button (cobalt blue)
export const BtnPrimary = (props: React.ButtonHTMLAttributes<HTMLButtonElement> & { children: ReactNode }) => (
  <button
    {...props}
    className={`inline-flex items-center justify-center gap-1.5 px-4 py-1.5
      bg-blue-600 hover:bg-blue-500 active:bg-blue-700 text-white text-xs font-semibold
      rounded transition-colors disabled:opacity-50 disabled:cursor-not-allowed
      ${props.className ?? ""}`}
  />
);

// Secondary / ghost button
export const BtnGhost = (props: React.ButtonHTMLAttributes<HTMLButtonElement> & { children: ReactNode }) => (
  <button
    {...props}
    className={`inline-flex items-center justify-center gap-1.5 px-4 py-1.5
      bg-gray-700 hover:bg-gray-600 active:bg-gray-800 text-gray-200 text-xs font-semibold
      rounded border border-gray-600 transition-colors
      ${props.className ?? ""}`}
  />
);

// Danger button
export const BtnDanger = (props: React.ButtonHTMLAttributes<HTMLButtonElement> & { children: ReactNode }) => (
  <button
    {...props}
    className={`inline-flex items-center justify-center gap-1.5 px-4 py-1.5
      bg-red-700 hover:bg-red-600 active:bg-red-800 text-white text-xs font-semibold
      rounded transition-colors ${props.className ?? ""}`}
  />
);

// Success button (emerald)
export const BtnSuccess = (props: React.ButtonHTMLAttributes<HTMLButtonElement> & { children: ReactNode }) => (
  <button
    {...props}
    className={`inline-flex items-center justify-center gap-1.5 px-4 py-1.5
      bg-green-600 hover:bg-green-500 active:bg-green-700 text-white text-xs font-semibold
      rounded transition-colors ${props.className ?? ""}`}
  />
);

// Section card
export const Card = ({ children, className = "" }: { children: ReactNode; className?: string }) => (
  <div className={`bg-gray-900 border border-gray-700 rounded-lg ${className}`}>{children}</div>
);

// Section header
export const SectionHeader = ({ color, children }: { color: string; children: ReactNode }) => (
  <div className="flex items-center gap-2 mb-3">
    <div className={`w-1 h-4 rounded-full ${color}`} />
    <h3 className="text-[0.7rem] font-bold uppercase tracking-widest text-gray-300">{children}</h3>
  </div>
);

// Divider
export const Divider = () => <div className="border-t border-gray-700 my-3" />;

// Read-only display value
export const ReadonlyField = ({ label, value }: { label: string; value: string }) => (
  <div className="flex flex-col gap-0.5">
    <span className="text-[0.6rem] uppercase tracking-wider text-gray-500">{label}</span>
    <span className="text-xs text-gray-200 font-medium">{value || "—"}</span>
  </div>
);

// Status badge
export const StatusPill = ({ status }: { status: string }) => {
  const map: Record<string, string> = {
    Active: "bg-green-700/40 text-green-300",
    Inactive: "bg-gray-700 text-gray-400",
    Issued: "bg-blue-700/40 text-blue-300",
    Returned: "bg-green-700/40 text-green-300",
    Renewed: "bg-yellow-700/40 text-yellow-300",
    Overdue: "bg-red-700/40 text-red-300",
  };
  return (
    <span className={`text-[0.65rem] font-semibold px-2 py-0.5 rounded-full ${map[status] ?? "bg-gray-700 text-gray-300"}`}>
      {status}
    </span>
  );
};

// Modal overlay wrapper
export const Modal = ({ children, onClose }: { children: ReactNode; onClose: () => void }) => (
  <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/70 backdrop-blur-sm"
    onClick={onClose}>
    <div className="relative" onClick={(e) => e.stopPropagation()}>
      {children}
    </div>
  </div>
);

// Tab bar
export const TabBar = ({
  tabs, active, onChange,
}: { tabs: string[]; active: string; onChange: (t: string) => void }) => (
  <div className="flex gap-1 bg-gray-800 rounded-lg p-1 mb-4">
    {tabs.map((t) => (
      <button key={t} onClick={() => onChange(t)}
        className={`flex-1 text-xs font-semibold py-1.5 rounded-md transition-colors
          ${active === t ? "bg-blue-600 text-white" : "text-gray-400 hover:text-white hover:bg-gray-700"}`}>
        {t}
      </button>
    ))}
  </div>
);
