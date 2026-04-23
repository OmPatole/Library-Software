import { useState } from "react";
import { invoke } from "../lib/tauri";
import {
  Search, UserPlus, Edit2, Users, Download,
  Trash2, Save, XCircle, RefreshCw, CheckCircle, AlertCircle,
} from "lucide-react";
import {
  Field, Input, Select, Textarea, BtnPrimary, BtnGhost, BtnDanger, BtnSuccess,
  Card, SectionHeader, Divider, ReadonlyField, StatusPill,
} from "./ui";

// ─── Types ─────────────────────────────────────────────────────────────────────

interface Member {
  id: string; first_name: string; middle_name: string | null; last_name: string;
  admission_year: string | null; course: string | null; current_year: string | null;
  branch: string | null; mobile_no: string | null; email: string | null;
  address?: string | null; is_active: boolean; total_due: number;
}

const COURSES   = ["B.Tech", "M.Tech", "B.Sc", "M.Sc", "BCA", "MCA", "B.Com", "MBA", "B.Pharm", "Diploma"];
const BRANCHES  = ["Computer Science", "Information Technology", "Electronics", "Mechanical", "Civil", "Electrical", "Chemical", "Biotechnology"];
const YEARS     = ["1st Year", "2nd Year", "3rd Year", "4th Year", "5th Year"];
const ADMY      = Array.from({ length: 10 }, (_, i) => String(new Date().getFullYear() - i));
const STATUS_OP = ["Active", "Inactive"];

const emptyMember = (): Omit<Member, "total_due"> & { address: string; total_due?: number } => ({
  id: "", first_name: "", middle_name: "", last_name: "", admission_year: ADMY[0],
  course: COURSES[0], current_year: YEARS[0], branch: BRANCHES[0],
  mobile_no: "", email: "", address: "", is_active: true,
});

const Feedback = ({ msg }: { msg: { t: "ok" | "err"; v: string } }) => (
  <div className={`flex items-center gap-1.5 text-xs px-3 py-2 rounded
    ${msg.t === "ok" ? "bg-green-900/50 text-green-300" : "bg-red-900/50 text-red-300"}`}>
    {msg.t === "ok" ? <CheckCircle size={12} /> : <AlertCircle size={12} />} {msg.v}
  </div>
);

// ─── Add Member Form ────────────────────────────────────────────────────────────

const AddMemberForm = () => {
  const [form, setForm] = useState(emptyMember());
  const [msg, setMsg] = useState<{ t: "ok" | "err"; v: string } | null>(null);
  const [genLoading, setGenLoading] = useState(false);

  const set = (k: string, v: string | boolean) => setForm((f) => ({ ...f, [k]: v }));

  const generateId = async () => {
    setGenLoading(true);
    try {
      let count = 1;
      try {
        const members: Member[] = await invoke("get_members");
        count = members.length + 1;
      } catch (e) {
        // Fallback if backend isn't wired yet
        count = Math.floor(Math.random() * 900) + 100;
      }
      const yearStr = form.admission_year || String(new Date().getFullYear());
      const shortYear = yearStr.slice(-2);
      const uid = `${shortYear}${count.toString().padStart(3, "0")}`;
      set("id", uid);
    } finally { setGenLoading(false); }
  };

  const handleSubmit = async () => {
    if (!form.first_name || !form.last_name || !form.id) {
      setMsg({ t: "err", v: "Name and ID are required." }); return;
    }
    try {
      await invoke("add_member", {
        member: { ...form, middle_name: form.middle_name || null, is_active: true, total_due: 0 },
      });
      setMsg({ t: "ok", v: `Member ${form.id} added successfully.` });
      setForm(emptyMember());
    } catch (e) { setMsg({ t: "err", v: String(e) }); }
  };

  return (
    <div className="flex flex-col gap-5 max-w-2xl mx-auto">
      {/* Personal Details */}
      <Card className="p-4">
        <div className="flex items-center justify-between mb-3">
          <SectionHeader color="bg-blue-500">Personal Details</SectionHeader>
          <div className="flex items-center gap-2">
            <span className="text-[0.65rem] text-gray-500">Member ID:</span>
            <Input value={form.id} onChange={(e) => set("id", e.target.value)}
              placeholder="Auto or manual" className="w-40" />
            <BtnGhost onClick={generateId} disabled={genLoading}>
              <RefreshCw size={11} className={genLoading ? "animate-spin" : ""} />
              Generate ID
            </BtnGhost>
          </div>
        </div>
        <div className="grid grid-cols-3 gap-3">
          <Field label="First Name"><Input value={form.first_name} onChange={(e) => set("first_name", e.target.value)} placeholder="First name" /></Field>
          <Field label="Middle Name"><Input value={form.middle_name ?? ""} onChange={(e) => set("middle_name", e.target.value)} placeholder="Middle name" /></Field>
          <Field label="Last Name"><Input value={form.last_name} onChange={(e) => set("last_name", e.target.value)} placeholder="Last name" /></Field>
        </div>
      </Card>

      {/* Academic Details */}
      <Card className="p-4">
        <SectionHeader color="bg-blue-500">Academic Details</SectionHeader>
        <div className="grid grid-cols-4 gap-3">
          <Field label="Admission Year">
            <Select value={form.admission_year ?? ""} onChange={(e) => set("admission_year", e.target.value)}>
              {ADMY.map((y) => <option key={y}>{y}</option>)}
            </Select>
          </Field>
          <Field label="Course">
            <Select value={form.course ?? ""} onChange={(e) => set("course", e.target.value)}>
              {COURSES.map((c) => <option key={c}>{c}</option>)}
            </Select>
          </Field>
          <Field label="Current Year">
            <Select value={form.current_year ?? ""} onChange={(e) => set("current_year", e.target.value)}>
              {YEARS.map((y) => <option key={y}>{y}</option>)}
            </Select>
          </Field>
          <Field label="Branch">
            <Select value={form.branch ?? ""} onChange={(e) => set("branch", e.target.value)}>
              {BRANCHES.map((b) => <option key={b}>{b}</option>)}
            </Select>
          </Field>
        </div>
      </Card>

      {/* Contact Details */}
      <Card className="p-4">
        <SectionHeader color="bg-blue-500">Contact Details</SectionHeader>
        <div className="grid grid-cols-2 gap-3">
          <Field label="Mobile No."><Input value={form.mobile_no ?? ""} onChange={(e) => set("mobile_no", e.target.value)} placeholder="10-digit number" /></Field>
          <Field label="Email ID"><Input type="email" value={form.email ?? ""} onChange={(e) => set("email", e.target.value)} placeholder="email@example.com" /></Field>
        </div>
        <div className="mt-3">
          <Field label="Address"><Textarea value={form.address ?? ""} onChange={(e) => set("address", e.target.value)} placeholder="Full address…" /></Field>
        </div>
      </Card>

      {msg && <Feedback msg={msg} />}

      <div className="flex justify-end">
        <BtnSuccess onClick={handleSubmit}>
          <UserPlus size={13} /> Add Member
        </BtnSuccess>
      </div>
    </div>
  );
};

// ─── Edit Member Form ───────────────────────────────────────────────────────────

const EditMemberForm = () => {
  const [uid, setUid] = useState("");
  const [form, setForm] = useState<(typeof emptyMember) extends () => infer R ? R : never>(emptyMember());
  const [found, setFound] = useState(false);
  const [issueStatus, setIssueStatus] = useState("No active issue");
  const [msg, setMsg] = useState<{ t: "ok" | "err"; v: string } | null>(null);

  const set = (k: string, v: string | boolean) => setForm((f: typeof form) => ({ ...f, [k]: v }));

  const handleSearch = async () => {
    try {
      const members: Member[] = await invoke("get_members");
      const m = members.find((x) => x.id === uid);
      if (!m) { setMsg({ t: "err", v: "Member not found." }); setFound(false); return; }
      setForm({
        id: m.id, first_name: m.first_name, middle_name: m.middle_name ?? "",
        last_name: m.last_name, admission_year: m.admission_year ?? ADMY[0],
        course: m.course ?? COURSES[0], current_year: m.current_year ?? YEARS[0],
        branch: m.branch ?? BRANCHES[0], mobile_no: m.mobile_no ?? "",
        email: m.email ?? "", address: "", is_active: m.is_active,
      });
      setIssueStatus(m.total_due > 0 ? `Due: ₹${m.total_due}` : "No active issue");
      setFound(true); setMsg(null);
    } catch (e) { setMsg({ t: "err", v: String(e) }); }
  };

  const handleSave = async () => {
    try {
      await invoke("add_member", {
        member: { ...form, middle_name: form.middle_name || null, total_due: 0 },
      });
      setMsg({ t: "ok", v: "Member updated successfully." });
    } catch (e) { setMsg({ t: "err", v: String(e) }); }
  };

  const handleDelete = async () => {
    if (!confirm(`Delete member ${uid}? This cannot be undone.`)) return;
    setMsg({ t: "ok", v: "Delete not yet wired to backend." });
  };

  return (
    <div className="flex flex-col gap-4 max-w-2xl mx-auto">
      {/* Search bar */}
      <Card className="p-3">
        <div className="flex items-center gap-2">
          <Input value={uid} onChange={(e) => setUid(e.target.value)}
            onKeyDown={(e) => e.key === "Enter" && handleSearch()}
            placeholder="Enter UID of Member" className="flex-1" />
          <BtnPrimary onClick={handleSearch}><Search size={12} /> Search</BtnPrimary>
        </div>
      </Card>

      {found && (
        <>
          <Card className="p-4">
            <div className="flex items-center justify-between mb-3">
              <SectionHeader color="bg-blue-500">Personal Details</SectionHeader>
              <div className="flex items-center gap-2 text-[0.65rem] text-gray-400">
                <span>ID: <span className="text-blue-300 font-mono">{form.id}</span></span>
              </div>
            </div>
            <div className="grid grid-cols-3 gap-3">
              <Field label="First Name"><Input value={form.first_name} onChange={(e) => set("first_name", e.target.value)} /></Field>
              <Field label="Middle Name"><Input value={form.middle_name ?? ""} onChange={(e) => set("middle_name", e.target.value)} /></Field>
              <Field label="Last Name"><Input value={form.last_name} onChange={(e) => set("last_name", e.target.value)} /></Field>
            </div>
          </Card>

          <Card className="p-4">
            <SectionHeader color="bg-blue-500">Academic Details</SectionHeader>
            <div className="grid grid-cols-4 gap-3">
              <Field label="Admission Year">
                <Select value={form.admission_year ?? ""} onChange={(e) => set("admission_year", e.target.value)}>
                  {ADMY.map((y) => <option key={y}>{y}</option>)}
                </Select>
              </Field>
              <Field label="Course">
                <Select value={form.course ?? ""} onChange={(e) => set("course", e.target.value)}>
                  {COURSES.map((c) => <option key={c}>{c}</option>)}
                </Select>
              </Field>
              <Field label="Current Year">
                <Select value={form.current_year ?? ""} onChange={(e) => set("current_year", e.target.value)}>
                  {YEARS.map((y) => <option key={y}>{y}</option>)}
                </Select>
              </Field>
              <Field label="Branch">
                <Select value={form.branch ?? ""} onChange={(e) => set("branch", e.target.value)}>
                  {BRANCHES.map((b) => <option key={b}>{b}</option>)}
                </Select>
              </Field>
            </div>
          </Card>

          <Card className="p-4">
            <SectionHeader color="bg-blue-500">Contact Details</SectionHeader>
            <div className="grid grid-cols-2 gap-3">
              <Field label="Mobile No."><Input value={form.mobile_no ?? ""} onChange={(e) => set("mobile_no", e.target.value)} /></Field>
              <Field label="Email ID"><Input type="email" value={form.email ?? ""} onChange={(e) => set("email", e.target.value)} /></Field>
            </div>
            <div className="mt-3">
              <Field label="Address"><Textarea value={form.address ?? ""} onChange={(e) => set("address", e.target.value)} /></Field>
            </div>
          </Card>

          <Card className="p-4">
            <SectionHeader color="bg-blue-500">Status</SectionHeader>
            <div className="grid grid-cols-2 gap-3">
              <Field label="Activation Status">
                <Select value={form.is_active ? "Active" : "Inactive"}
                  onChange={(e) => set("is_active", e.target.value === "Active")}>
                  {STATUS_OP.map((s) => <option key={s}>{s}</option>)}
                </Select>
              </Field>
              <Field label="Issue / Due Status">
                <Input value={issueStatus} readOnly className="opacity-60 cursor-not-allowed" />
              </Field>
            </div>
          </Card>

          {msg && <Feedback msg={msg} />}

          <div className="flex gap-2 justify-end">
            <BtnGhost onClick={() => { setForm(emptyMember()); setFound(false); setUid(""); }}>
              <XCircle size={12} /> Clear Old Data
            </BtnGhost>
            <BtnDanger onClick={handleDelete}><Trash2 size={12} /> Delete Member</BtnDanger>
            <BtnSuccess onClick={handleSave}><Save size={12} /> Save</BtnSuccess>
          </div>
        </>
      )}

      {!found && msg && <Feedback msg={msg} />}
    </div>
  );
};

// ─── Member Details View ────────────────────────────────────────────────────────

const MemberDetailsView = () => {
  const [dlr, setDlr] = useState("");
  const [member, setMember] = useState<Member | null>(null);
  const [msg, setMsg] = useState<{ t: "ok" | "err"; v: string } | null>(null);

  const doSearch = async () => {
    try {
      const members: Member[] = await invoke("get_members");
      const m = members.find((x) => x.id === dlr);
      if (!m) { setMsg({ t: "err", v: "Member not found." }); setMember(null); return; }
      setMember(m); setMsg(null);
    } catch (e) { setMsg({ t: "err", v: String(e) }); }
  };

  return (
    <div className="flex flex-col gap-4 max-w-2xl mx-auto">
      <Card className="p-3">
        <div className="flex items-center gap-2">
          <Input value={dlr} onChange={(e) => setDlr(e.target.value)}
            onKeyDown={(e) => e.key === "Enter" && doSearch()}
            placeholder="Enter DLR / UID of Member" className="flex-1" />
          <BtnPrimary onClick={doSearch}><Search size={12} /> Search</BtnPrimary>
        </div>
      </Card>

      {msg && <Feedback msg={msg} />}

      {member && (
        <Card className="p-4">
          <SectionHeader color="bg-blue-500">Member Information</SectionHeader>
          <Divider />
          <div className="grid grid-cols-2 gap-x-6 gap-y-4">
            <ReadonlyField label="Full Name" value={`${member.first_name} ${member.middle_name ?? ""} ${member.last_name}`.trim()} />
            <ReadonlyField label="Admission Year" value={member.admission_year ?? "—"} />
            <ReadonlyField label="Activation Status" value={member.is_active ? "Active" : "Inactive"} />
            <ReadonlyField label="Current Issue Status" value={member.total_due > 0 ? `Due: ₹${member.total_due}` : "No active issue"} />
            <ReadonlyField label="Course" value={member.course ?? "—"} />
            <ReadonlyField label="Branch" value={member.branch ?? "—"} />
            <ReadonlyField label="Current Year" value={member.current_year ?? "—"} />
            <ReadonlyField label="Total Due (Rs.)" value={`₹ ${member.total_due.toFixed(2)}`} />
          </div>
          <Divider />
          <div className="flex items-center gap-2">
            <span className="text-[0.65rem] text-gray-500">Member ID:</span>
            <span className="text-xs font-mono text-blue-300">{member.id}</span>
            <StatusPill status={member.is_active ? "Active" : "Inactive"} />
          </div>
        </Card>
      )}
    </div>
  );
};

// ─── Export Member Data ─────────────────────────────────────────────────────────

const ExportMemberData = () => {
  const [filter, setFilter] = useState({
    course: "", branch: "", year: "", type: "Student", onlyActive: false,
  });
  const [members, setMembers] = useState<Member[]>([]);
  const [loading, setLoading] = useState(false);
  const [msg, setMsg] = useState<{ t: "ok" | "err"; v: string } | null>(null);

  const setF = (k: string, v: string | boolean) => setFilter((f) => ({ ...f, [k]: v }));

  const doSearch = async () => {
    setLoading(true);
    try {
      let all: Member[] = await invoke("get_members");
      if (filter.onlyActive) all = all.filter((m) => m.is_active);
      if (filter.course) all = all.filter((m) => m.course === filter.course);
      if (filter.branch) all = all.filter((m) => m.branch === filter.branch);
      if (filter.year)   all = all.filter((m) => m.current_year === filter.year);
      setMembers(all);
    } catch (e) { setMsg({ t: "err", v: String(e) }); }
    finally { setLoading(false); }
  };

  const showAll = async () => {
    setFilter({ course: "", branch: "", year: "", type: "Student", onlyActive: false });
    setLoading(true);
    try { setMembers(await invoke("get_members")); }
    finally { setLoading(false); }
  };

  const exportExcel = () => setMsg({ t: "ok", v: "Excel export requires native Tauri file dialog — wire via Rust tauri command." });

  return (
    <div className="flex flex-col gap-4 h-full">
      {/* Filter bar */}
      <Card className="p-3">
        <div className="flex flex-wrap items-end gap-3">
          <div className="flex-1 min-w-28">
            <Field label="Course">
              <Select value={filter.course} onChange={(e) => setF("course", e.target.value)}>
                <option value="">All Courses</option>
                {COURSES.map((c) => <option key={c}>{c}</option>)}
              </Select>
            </Field>
          </div>
          <div className="flex-1 min-w-28">
            <Field label="Branch">
              <Select value={filter.branch} onChange={(e) => setF("branch", e.target.value)}>
                <option value="">All Branches</option>
                {BRANCHES.map((b) => <option key={b}>{b}</option>)}
              </Select>
            </Field>
          </div>
          <div className="flex-1 min-w-24">
            <Field label="Year">
              <Select value={filter.year} onChange={(e) => setF("year", e.target.value)}>
                <option value="">All Years</option>
                {YEARS.map((y) => <option key={y}>{y}</option>)}
              </Select>
            </Field>
          </div>
          <div className="flex-1 min-w-24">
            <Field label="Type">
              <Select value={filter.type} onChange={(e) => setF("type", e.target.value)}>
                <option>Student</option><option>Faculty</option>
              </Select>
            </Field>
          </div>
          <div className="flex items-center gap-1.5 pb-0.5">
            <input type="checkbox" id="only-active" checked={filter.onlyActive}
              onChange={(e) => setF("onlyActive", e.target.checked)}
              className="accent-blue-500 w-3.5 h-3.5" />
            <label htmlFor="only-active" className="text-[0.7rem] text-gray-400 cursor-pointer whitespace-nowrap">
              Only Active
            </label>
          </div>
          <BtnPrimary onClick={doSearch} disabled={loading}>
            <Search size={12} /> {loading ? "…" : "Search"}
          </BtnPrimary>
          <BtnGhost onClick={showAll}><Users size={12} /> Show All</BtnGhost>
        </div>
      </Card>

      {msg && <Feedback msg={msg} />}

      {/* Data grid */}
      <div className="flex-1 overflow-auto rounded-lg border border-gray-700 bg-gray-900">
        {members.length === 0 ? (
          <div className="flex flex-col items-center justify-center h-full gap-2 text-gray-600">
            <Users size={32} className="opacity-30" />
            <p className="text-xs">Search to load member data</p>
          </div>
        ) : (
          <table className="w-full text-xs border-collapse">
            <thead className="sticky top-0 bg-gray-800 z-10">
              <tr className="border-b border-gray-700">
                {["ID", "Name", "Course", "Branch", "Year", "Email", "Mobile", "Status"].map((h) => (
                  <th key={h} className="text-left py-2 px-3 text-[0.65rem] font-bold uppercase tracking-wider text-blue-400">{h}</th>
                ))}
              </tr>
            </thead>
            <tbody>
              {members.map((m) => (
                <tr key={m.id} className="border-b border-gray-800 hover:bg-gray-800/60">
                  <td className="py-1.5 px-3 font-mono text-blue-300">{m.id}</td>
                  <td className="py-1.5 px-3">{m.first_name} {m.last_name}</td>
                  <td className="py-1.5 px-3 text-gray-400">{m.course ?? "—"}</td>
                  <td className="py-1.5 px-3 text-gray-400">{m.branch ?? "—"}</td>
                  <td className="py-1.5 px-3 text-gray-400">{m.current_year ?? "—"}</td>
                  <td className="py-1.5 px-3 text-gray-400">{m.email ?? "—"}</td>
                  <td className="py-1.5 px-3 text-gray-400">{m.mobile_no ?? "—"}</td>
                  <td className="py-1.5 px-3"><StatusPill status={m.is_active ? "Active" : "Inactive"} /></td>
                </tr>
              ))}
            </tbody>
          </table>
        )}
      </div>

      <div className="flex justify-between items-center">
        <span className="text-[0.65rem] text-gray-500">{members.length} records loaded</span>
        <BtnSuccess onClick={exportExcel}><Download size={12} /> Export to Excel</BtnSuccess>
      </div>
    </div>
  );
};

// ─── Membership Module (Tabbed) ────────────────────────────────────────────────

const TABS = ["Add Member", "Edit Member", "Member Details", "Export Data"];
const ICONS = [<UserPlus size={13} />, <Edit2 size={13} />, <Users size={13} />, <Download size={13} />];

export default function MembershipModule() {
  const [tab, setTab] = useState("Add Member");

  return (
    <div className="flex flex-col flex-1 overflow-hidden bg-gray-950 p-4 gap-4">
      {/* Module header */}
      <div className="flex items-center gap-3 shrink-0">
        <Users size={18} className="text-blue-400" />
        <h1 className="text-sm font-bold uppercase tracking-widest text-gray-200">Membership Module</h1>
      </div>

      {/* Tab bar */}
      <div className="flex gap-1 bg-gray-800 rounded-lg p-1 shrink-0">
        {TABS.map((t, i) => (
          <button key={t} onClick={() => setTab(t)}
            className={`flex-1 flex items-center justify-center gap-1.5 text-xs font-semibold py-1.5
              rounded-md transition-colors
              ${tab === t ? "bg-blue-600 text-white" : "text-gray-400 hover:text-white hover:bg-gray-700"}`}>
            {ICONS[i]} {t}
          </button>
        ))}
      </div>

      {/* Content */}
      <div className="flex-1 overflow-y-auto">
        {tab === "Add Member"     && <AddMemberForm />}
        {tab === "Edit Member"    && <EditMemberForm />}
        {tab === "Member Details" && <MemberDetailsView />}
        {tab === "Export Data"    && <ExportMemberData />}
      </div>
    </div>
  );
}
