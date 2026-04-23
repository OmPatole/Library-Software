import { useState } from "react";
import { FileText, Download, Search, PlusCircle, Settings, Power } from "lucide-react";
import { Field, Input, BtnPrimary, BtnSuccess, Card, SectionHeader, Select } from "./ui";

// ─── Reports Module ────────────────────────────────────────────────────────────

export function ReportsModule() {
  const [date, setDate] = useState(new Date().toISOString().split("T")[0]);

  return (
    <div className="flex flex-col flex-1 overflow-hidden bg-gray-950 p-4 gap-4">
      <div className="flex items-center gap-3 shrink-0">
        <FileText size={18} className="text-blue-400" />
        <h1 className="text-sm font-bold uppercase tracking-widest text-gray-200">Generate Report</h1>
      </div>

      <Card className="p-3 flex gap-4 shrink-0 items-end">
        <Field label="Report Date">
          <Input type="date" value={date} onChange={e => setDate(e.target.value)} />
        </Field>
        <BtnPrimary className="mb-0.5">Generate</BtnPrimary>
        <div className="flex-1" />
        <Field label="Export Path">
          <div className="flex gap-2">
            <Input placeholder="Select folder..." readOnly className="w-64" />
            <BtnPrimary><Search size={12} /> Find</BtnPrimary>
          </div>
        </Field>
      </Card>

      <div className="flex flex-col gap-4 flex-1 overflow-hidden">
        <Card className="flex-1 p-4 flex flex-col gap-2">
          <SectionHeader color="bg-blue-500">Issue Transactions</SectionHeader>
          <div className="flex-1 bg-gray-800 border border-gray-700 rounded-lg flex items-center justify-center text-gray-500 text-xs">
            Issue report data will appear here
          </div>
        </Card>
        <Card className="flex-1 p-4 flex flex-col gap-2">
          <SectionHeader color="bg-green-500">Return Transactions</SectionHeader>
          <div className="flex-1 bg-gray-800 border border-gray-700 rounded-lg flex items-center justify-center text-gray-500 text-xs">
            Return report data will appear here
          </div>
        </Card>
      </div>

      <div className="flex justify-end shrink-0">
        <BtnSuccess><Download size={12} /> Export to Excel</BtnSuccess>
      </div>
    </div>
  );
}

// ─── Settings Modals ───────────────────────────────────────────────────────────

export function SettingsModule({ view }: { view: "activation" | "branch" }) {
  const [branchTab, setBranchTab] = useState<"Add Branch" | "Edit Branch">("Add Branch");

  return (
    <div className="flex flex-col flex-1 overflow-hidden bg-gray-950 p-4 gap-4 items-center justify-center">
      
      {view === "activation" && (
        <Card className="p-6 w-96 flex flex-col gap-4 shadow-2xl">
          <div className="flex items-center gap-3">
            <Power size={18} className="text-blue-400" />
            <h2 className="text-sm font-bold uppercase tracking-widest text-gray-200">Activation Settings</h2>
          </div>
          <p className="text-xs text-gray-400">Bulk change the activation status for all members in the system.</p>
          <Field label="Global Status">
            <Select>
              <option>Activate All</option>
              <option>Deactivate All</option>
            </Select>
          </Field>
          <div className="flex justify-end mt-2">
            <BtnPrimary>Apply Settings</BtnPrimary>
          </div>
        </Card>
      )}

      {view === "branch" && (
        <Card className="p-6 w-96 flex flex-col gap-4 shadow-2xl">
          <div className="flex items-center gap-3">
            <Settings size={18} className="text-blue-400" />
            <h2 className="text-sm font-bold uppercase tracking-widest text-gray-200">Branch Management</h2>
          </div>
          
          <div className="flex gap-1 bg-gray-800 rounded-lg p-1">
            {["Add Branch", "Edit Branch"].map(t => (
              <button key={t} onClick={() => setBranchTab(t as any)}
                className={`flex-1 text-xs font-semibold py-1.5 rounded-md transition-colors
                  ${branchTab === t ? "bg-blue-600 text-white" : "text-gray-400 hover:text-white"}`}>
                {t}
              </button>
            ))}
          </div>

          <Field label="Name of Branch">
            <Input placeholder="e.g. Computer Science" />
          </Field>
          
          <div className="flex justify-end mt-2">
            <BtnSuccess><PlusCircle size={12} /> {branchTab}</BtnSuccess>
          </div>
        </Card>
      )}

    </div>
  );
}
