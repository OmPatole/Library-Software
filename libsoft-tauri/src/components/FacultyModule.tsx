import { useState } from "react";
import { UserPlus, Edit2, GraduationCap, Search, Mail, Save } from "lucide-react";
import {
  Field, Input, Select, Textarea, BtnPrimary, BtnSuccess,
  Card, SectionHeader, ReadonlyField, StatusPill, Divider,
} from "./ui";

const BRANCHES = ["Computer Science", "Information Technology", "Electronics", "Mechanical", "Civil"];
const JOIN_UNDER = ["Permanent", "Contract", "Visiting", "Guest"];
const STATUS_OP = ["Active", "Inactive"];

// ─── Add/Edit Faculty ──────────────────────────────────────────────────────────

const FacultyForm = ({ mode }: { mode: "add" | "edit" }) => {
  const [uid, setUid] = useState("");
  const [form, setForm] = useState({
    first_name: "", middle_name: "", last_name: "",
    joining_date: new Date().toISOString().split("T")[0],
    joining_under: JOIN_UNDER[0], branch: BRANCHES[0], status: STATUS_OP[0],
    mobile: "", email: "", address: "",
  });

  return (
    <div className="flex flex-col gap-4 max-w-2xl mx-auto">
      {mode === "edit" && (
        <Card className="p-3 flex gap-2">
          <Input placeholder="Enter UID of Faculty" value={uid} onChange={e => setUid(e.target.value)} />
          <BtnPrimary><Search size={12} /> Search</BtnPrimary>
        </Card>
      )}

      <Card className="p-4">
        <SectionHeader color="bg-blue-500">Personal Details</SectionHeader>
        <div className="grid grid-cols-3 gap-3">
          <Field label="First Name"><Input value={form.first_name} onChange={e => setForm({ ...form, first_name: e.target.value })} /></Field>
          <Field label="Middle Name"><Input value={form.middle_name} onChange={e => setForm({ ...form, middle_name: e.target.value })} /></Field>
          <Field label="Last Name"><Input value={form.last_name} onChange={e => setForm({ ...form, last_name: e.target.value })} /></Field>
        </div>
      </Card>

      <Card className="p-4">
        <SectionHeader color="bg-blue-500">Other Details</SectionHeader>
        <div className="grid grid-cols-3 gap-3">
          <Field label="Joining Date"><Input type="date" value={form.joining_date} onChange={e => setForm({ ...form, joining_date: e.target.value })} /></Field>
          <Field label="Joining Under">
            <Select value={form.joining_under} onChange={e => setForm({ ...form, joining_under: e.target.value })}>
              {JOIN_UNDER.map(u => <option key={u}>{u}</option>)}
            </Select>
          </Field>
          <Field label="Branch">
            <Select value={form.branch} onChange={e => setForm({ ...form, branch: e.target.value })}>
              {BRANCHES.map(b => <option key={b}>{b}</option>)}
            </Select>
          </Field>
          {mode === "edit" && (
            <Field label="Current Status">
              <Select value={form.status} onChange={e => setForm({ ...form, status: e.target.value })}>
                {STATUS_OP.map(s => <option key={s}>{s}</option>)}
              </Select>
            </Field>
          )}
        </div>
      </Card>

      <Card className="p-4">
        <SectionHeader color="bg-blue-500">Contact Details</SectionHeader>
        <div className="grid grid-cols-2 gap-3 mb-3">
          <Field label="Mobile No."><Input value={form.mobile} onChange={e => setForm({ ...form, mobile: e.target.value })} /></Field>
          <Field label="Email ID"><Input type="email" value={form.email} onChange={e => setForm({ ...form, email: e.target.value })} /></Field>
        </div>
        <Field label="Address"><Textarea value={form.address} onChange={e => setForm({ ...form, address: e.target.value })} /></Field>
      </Card>

      <div className="flex justify-end">
        <BtnSuccess><Save size={12} /> {mode === "add" ? "Add Faculty" : "Save Changes"}</BtnSuccess>
      </div>
    </div>
  );
};

// ─── Faculty Details View ──────────────────────────────────────────────────────

const FacultyDetails = () => {
  return (
    <div className="flex flex-col gap-4 max-w-2xl mx-auto h-full">
      <Card className="p-3 flex gap-2">
        <Input placeholder="Enter UID of Faculty" />
        <BtnPrimary><Search size={12} /> Search</BtnPrimary>
      </Card>

      <Card className="p-4 flex-1">
        <SectionHeader color="bg-blue-500">Faculty Information</SectionHeader>
        <Divider />
        <div className="grid grid-cols-2 gap-x-6 gap-y-4">
          <ReadonlyField label="Full Name" value="—" />
          <ReadonlyField label="Joining Date" value="—" />
          <ReadonlyField label="Joining Under" value="—" />
          <ReadonlyField label="Current Issue Status" value="—" />
          <ReadonlyField label="Branch" value="—" />
          <ReadonlyField label="Activation Status" value="—" />
          <ReadonlyField label="Total Due (Rs.)" value="—" />
        </div>
        <Divider />
        <div className="flex justify-between items-center mt-4">
          <div className="flex items-center gap-2">
            <span className="text-[0.65rem] text-gray-500">Faculty ID:</span>
            <span className="text-xs font-mono text-blue-300">FAC-XXXX</span>
            <StatusPill status="Inactive" />
          </div>
          <BtnPrimary><Mail size={12} /> Generate Mail</BtnPrimary>
        </div>
      </Card>
    </div>
  );
};

// ─── Faculty Module Root ───────────────────────────────────────────────────────

export default function FacultyModule() {
  const [tab, setTab] = useState("Add Faculty");
  const TABS = ["Add Faculty", "Edit Faculty", "Faculty Details"];
  const ICONS = [<UserPlus size={13} />, <Edit2 size={13} />, <GraduationCap size={13} />];

  return (
    <div className="flex flex-col flex-1 overflow-hidden bg-gray-950 p-4 gap-4">
      <div className="flex items-center gap-3 shrink-0">
        <GraduationCap size={18} className="text-blue-400" />
        <h1 className="text-sm font-bold uppercase tracking-widest text-gray-200">Faculty Module</h1>
      </div>
      <div className="flex gap-1 bg-gray-800 rounded-lg p-1 shrink-0">
        {TABS.map((t, i) => (
          <button key={t} onClick={() => setTab(t)}
            className={`flex-1 flex items-center justify-center gap-1.5 text-xs font-semibold py-1.5
              rounded-md transition-colors ${tab === t ? "bg-blue-600 text-white" : "text-gray-400 hover:text-white hover:bg-gray-700"}`}>
            {ICONS[i]} {t}
          </button>
        ))}
      </div>
      <div className="flex-1 overflow-y-auto">
        {tab === "Add Faculty" && <FacultyForm mode="add" />}
        {tab === "Edit Faculty" && <FacultyForm mode="edit" />}
        {tab === "Faculty Details" && <FacultyDetails />}
      </div>
    </div>
  );
}
