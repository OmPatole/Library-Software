import { useState } from "react";
import { invoke } from "../lib/tauri";
import {
  Search, X, ChevronRight, RefreshCw, CheckCircle, CornerDownLeft,
  Inbox, AlertCircle, BookOpen,
} from "lucide-react";
import {
  Field, Input, Select, BtnPrimary, BtnGhost,
  SectionHeader, Divider, StatusPill, TabBar,
} from "./ui";

// ─── Types ─────────────────────────────────────────────────────────────────────

interface Book {
  accession_no: string; call_no: string; title: string; author: string;
  branch: string; publisher: string; price: number; bill_no: string; status: string;
}

interface Transaction {
  id: number | null; accession_no: string; user_id: string; user_name: string;
  issue_date: string; expected_return_date: string; actual_return_date: string | null; status: string;
}

interface Member {
  id: string; first_name: string; middle_name: string | null; last_name: string;
  course: string | null; branch: string | null; current_year: string | null; is_active: boolean;
}

const today = () => new Date().toISOString().split("T")[0];
const futureDate = (d: number) => { const dt = new Date(); dt.setDate(dt.getDate() + d); return dt.toISOString().split("T")[0]; };

// ─── Left Panel — Book Search ──────────────────────────────────────────────────

const LeftPanel = ({ onResults }: { onResults: (b: Book[]) => void }) => {
  const [form, setForm] = useState({
    accession_no: "", title: "", author: "", call_no: "",
    status: "", publisher: "", verification: "",
  });
  const [found, setFound] = useState<Book | null>(null);
  const [loading, setLoading] = useState(false);

  const doSearch = async () => {
    const q = form.accession_no || form.title || form.author || form.call_no || form.publisher || "";
    if (!q) return;
    setLoading(true);
    try {
      const r: Book[] = await invoke("search_books", { query: q });
      onResults(r); setFound(r[0] ?? null);
    } finally { setLoading(false); }
  };

  const doClear = () => {
    setForm({ accession_no: "", title: "", author: "", call_no: "", status: "", publisher: "", verification: "" });
    setFound(null); onResults([]);
  };

  const fields: [string, keyof typeof form, string][] = [
    ["Accession No.", "accession_no", "e.g. ACC-001"],
    ["Book Name",     "title",        "Search title…"],
    ["Author Name",   "author",       "Author…"],
    ["Call No.",      "call_no",      "e.g. 005.13"],
    ["Issue Status",  "status",       "Available / Issued"],
    ["Publisher",     "publisher",    "Publisher…"],
    ["Verification",  "verification", "Verified / Not Verified"],
  ];

  return (
    <aside className="w-60 shrink-0 flex flex-col gap-3 border-r border-gray-700 bg-gray-900 p-3 overflow-y-auto">
      <SectionHeader color="bg-blue-500">Book Search</SectionHeader>
      <Divider />

      {fields.map(([label, key, ph]) => (
        <Field key={key} label={label}>
          <Input placeholder={ph} value={form[key]}
            onChange={(e) => setForm({ ...form, [key]: e.target.value })}
            onKeyDown={(e) => e.key === "Enter" && doSearch()} />
        </Field>
      ))}

      <div className="flex gap-2 mt-2">
        <BtnPrimary className="flex-1" onClick={doSearch} disabled={loading}>
          <Search size={12} /> {loading ? "…" : "Search"}
        </BtnPrimary>
        <BtnGhost onClick={doClear}><X size={12} /></BtnGhost>
      </div>

      {found && (
        <>
          <Divider />
          <div className="bg-gray-800 rounded-lg p-2.5 space-y-1">
            <p className="text-xs text-blue-300 font-semibold truncate">{found.title}</p>
            <p className="text-[0.7rem] text-gray-400">{found.author}</p>
            <p className="text-[0.65rem] text-gray-500">Acc: {found.accession_no}</p>
            <StatusPill status={found.status} />
          </div>
        </>
      )}
    </aside>
  );
};

// ─── Center Panel — Issue / Return / Renew ──────────────────────────────────────

// Issue Tab
const IssueTab = ({ onDone }: { onDone: () => void }) => {
  const [form, setForm] = useState({
    user_id: "", user_name: "", course: "", branch: "", current_year: "",
    accession_no: "", title: "", author: "", call_no: "", book_status: "",
    issue_date: today(), expected_return_date: futureDate(14),
  });
  const [memberInfo, setMemberInfo] = useState<Member | null>(null);
  const [bookInfo, setBookInfo] = useState<Book | null>(null);
  const [msg, setMsg] = useState<{ t: "ok" | "err"; v: string } | null>(null);

  const getDetails = async () => {
    try {
      if (form.user_id) {
        const members: Member[] = await invoke("get_members");
        const m = members.find((x) => x.id === form.user_id);
        if (m) {
          setMemberInfo(m);
          setForm((f) => ({ ...f, user_name: `${m.first_name} ${m.last_name}`,
            course: m.course ?? "", branch: m.branch ?? "", current_year: m.current_year ?? "" }));
        }
      }
      if (form.accession_no) {
        const books: Book[] = await invoke("search_books", { query: form.accession_no });
        const b = books[0];
        if (b) {
          setBookInfo(b);
          setForm((f) => ({ ...f, title: b.title, author: b.author, call_no: b.call_no, book_status: b.status }));
        }
      }
    } catch (e) { setMsg({ t: "err", v: String(e) }); }
  };

  const proceed = async () => {
    if (!form.accession_no || !form.user_id) {
      setMsg({ t: "err", v: "Accession No. and Student DLR/ID are required." }); return;
    }
    try {
      await invoke("issue_book", {
        transaction: { accession_no: form.accession_no, user_id: form.user_id,
          issue_date: form.issue_date, expected_return_date: form.expected_return_date,
          user_name: form.user_name, actual_return_date: null, status: "Issued", id: null },
      });
      setMsg({ t: "ok", v: "Book issued successfully." });
      setForm({ user_id: "", user_name: "", course: "", branch: "", current_year: "",
        accession_no: "", title: "", author: "", call_no: "", book_status: "",
        issue_date: today(), expected_return_date: futureDate(14) });
      setMemberInfo(null); setBookInfo(null); onDone();
    } catch (e) { setMsg({ t: "err", v: String(e) }); }
  };

  const set = (k: string, v: string) => setForm((f) => ({ ...f, [k]: v }));

  return (
    <div className="flex flex-col gap-3">
      {/* Student details */}
      <div className="bg-gray-800 rounded-lg p-3 space-y-2">
        <p className="text-[0.65rem] font-bold uppercase tracking-wider text-blue-400 mb-1">Student Info</p>
        <Field label="DLR of Student">
          <Input placeholder="Enter DLR / Member ID" value={form.user_id} onChange={(e) => set("user_id", e.target.value)} />
        </Field>
        <Field label="Student Name & Status">
          <Input placeholder="Auto-filled on Get Details" value={form.user_name} onChange={(e) => set("user_name", e.target.value)} />
        </Field>
        <div className="grid grid-cols-3 gap-2">
          <Field label="Course"><Input value={form.course} readOnly placeholder="Auto-filled" className="opacity-70" /></Field>
          <Field label="Branch"><Input value={form.branch} readOnly placeholder="Auto-filled" className="opacity-70" /></Field>
          <Field label="Year"><Input value={form.current_year} readOnly placeholder="Auto-filled" className="opacity-70" /></Field>
        </div>
        {memberInfo && <div className="flex gap-1"><StatusPill status={memberInfo.is_active ? "Active" : "Inactive"} /></div>}
      </div>

      {/* Book details */}
      <div className="bg-gray-800 rounded-lg p-3 space-y-2">
        <p className="text-[0.65rem] font-bold uppercase tracking-wider text-blue-400 mb-1">Book Info</p>
        <Field label="Accession Number">
          <Input placeholder="e.g. ACC-001" value={form.accession_no} onChange={(e) => set("accession_no", e.target.value)} />
        </Field>
        <div className="grid grid-cols-2 gap-2">
          <Field label="Book Name"><Input value={form.title} onChange={(e) => set("title", e.target.value)} /></Field>
          <Field label="Author"><Input value={form.author} onChange={(e) => set("author", e.target.value)} /></Field>
        </div>
        <div className="grid grid-cols-2 gap-2">
          <Field label="Status">
            <Input value={form.book_status} readOnly className="opacity-60 cursor-not-allowed" />
          </Field>
          <Field label="Call No."><Input value={form.call_no} onChange={(e) => set("call_no", e.target.value)} /></Field>
        </div>
        {bookInfo && <StatusPill status={bookInfo.status} />}
      </div>

      {/* Dates */}
      <div className="grid grid-cols-2 gap-2">
        <Field label="Issue Date"><Input type="date" value={form.issue_date} onChange={(e) => set("issue_date", e.target.value)} /></Field>
        <Field label="Return Date"><Input type="date" value={form.expected_return_date} onChange={(e) => set("expected_return_date", e.target.value)} /></Field>
      </div>

      {msg && (
        <div className={`flex items-center gap-1.5 text-xs px-3 py-2 rounded
          ${msg.t === "ok" ? "bg-green-900/50 text-green-300" : "bg-red-900/50 text-red-300"}`}>
          {msg.t === "ok" ? <CheckCircle size={12} /> : <AlertCircle size={12} />} {msg.v}
        </div>
      )}

      <div className="flex gap-2">
        <BtnGhost className="flex-1" onClick={getDetails}><Search size={12} /> Get Details</BtnGhost>
        <BtnPrimary className="flex-1" onClick={proceed}><ChevronRight size={12} /> Proceed</BtnPrimary>
      </div>
    </div>
  );
};

// Return Tab
const ReturnTab = ({ onDone }: { onDone: () => void }) => {
  const [accNo, setAccNo] = useState("");
  const [returnDate, setReturnDate] = useState(today());
  const [bookInfo, setBookInfo] = useState<Book | null>(null);
  const [msg, setMsg] = useState<{ t: "ok" | "err"; v: string } | null>(null);

  const getDetails = async () => {
    try {
      const books: Book[] = await invoke("search_books", { query: accNo });
      setBookInfo(books[0] ?? null);
    } catch (e) { setMsg({ t: "err", v: String(e) }); }
  };

  const proceed = async () => {
    if (!accNo) { setMsg({ t: "err", v: "Enter accession number." }); return; }
    try {
      await invoke("return_book", { accessionNo: accNo, returnDate });
      setMsg({ t: "ok", v: "Book returned successfully." });
      setAccNo(""); setBookInfo(null); onDone();
    } catch (e) { setMsg({ t: "err", v: String(e) }); }
  };

  return (
    <div className="flex flex-col gap-3">
      <div className="bg-gray-800 rounded-lg p-3 space-y-2">
        <Field label="Accession Number">
          <Input placeholder="e.g. ACC-001" value={accNo} onChange={(e) => setAccNo(e.target.value)} />
        </Field>
        {bookInfo && (
          <div className="space-y-1 pt-1">
            <p className="text-xs text-blue-300 font-medium">{bookInfo.title}</p>
            <p className="text-[0.7rem] text-gray-400">{bookInfo.author}</p>
            <StatusPill status={bookInfo.status} />
          </div>
        )}
      </div>
      <Field label="Return Date">
        <Input type="date" value={returnDate} onChange={(e) => setReturnDate(e.target.value)} />
      </Field>
      {msg && (
        <div className={`flex items-center gap-1.5 text-xs px-3 py-2 rounded
          ${msg.t === "ok" ? "bg-green-900/50 text-green-300" : "bg-red-900/50 text-red-300"}`}>
          {msg.t === "ok" ? <CheckCircle size={12} /> : <AlertCircle size={12} />} {msg.v}
        </div>
      )}
      <div className="flex gap-2">
        <BtnGhost className="flex-1" onClick={getDetails}><Search size={12} /> Get Details</BtnGhost>
        <BtnPrimary className="flex-1" onClick={proceed}><CornerDownLeft size={12} /> Proceed</BtnPrimary>
      </div>
    </div>
  );
};

// Renew Tab
const RenewTab = ({ onDone }: { onDone: () => void }) => {
  const [accNo, setAccNo] = useState("");
  const [newDate, setNewDate] = useState(futureDate(14));
  const [bookInfo, setBookInfo] = useState<Book | null>(null);
  const [msg, setMsg] = useState<{ t: "ok" | "err"; v: string } | null>(null);

  const getDetails = async () => {
    try {
      const books: Book[] = await invoke("search_books", { query: accNo });
      setBookInfo(books[0] ?? null);
    } catch (e) { setMsg({ t: "err", v: String(e) }); }
  };

  const proceed = async () => {
    if (!accNo) { setMsg({ t: "err", v: "Enter accession number." }); return; }
    try {
      await invoke("renew_book", { accessionNo: accNo, newReturnDate: newDate });
      setMsg({ t: "ok", v: "Book renewed successfully." });
      setAccNo(""); setBookInfo(null); onDone();
    } catch (e) { setMsg({ t: "err", v: String(e) }); }
  };

  return (
    <div className="flex flex-col gap-3">
      <div className="bg-gray-800 rounded-lg p-3 space-y-2">
        <Field label="Accession Number">
          <Input placeholder="e.g. ACC-001" value={accNo} onChange={(e) => setAccNo(e.target.value)} />
        </Field>
        {bookInfo && (
          <div className="space-y-1 pt-1">
            <p className="text-xs text-blue-300 font-medium">{bookInfo.title}</p>
            <p className="text-[0.7rem] text-gray-400">{bookInfo.author}</p>
            <StatusPill status={bookInfo.status} />
          </div>
        )}
      </div>
      <Field label="New Due Date">
        <Input type="date" value={newDate} onChange={(e) => setNewDate(e.target.value)} />
      </Field>
      {msg && (
        <div className={`flex items-center gap-1.5 text-xs px-3 py-2 rounded
          ${msg.t === "ok" ? "bg-green-900/50 text-green-300" : "bg-red-900/50 text-red-300"}`}>
          {msg.t === "ok" ? <CheckCircle size={12} /> : <AlertCircle size={12} />} {msg.v}
        </div>
      )}
      <div className="flex gap-2">
        <BtnGhost className="flex-1" onClick={getDetails}><Search size={12} /> Get Details</BtnGhost>
        <BtnPrimary className="flex-1" onClick={proceed}><RefreshCw size={12} /> Proceed</BtnPrimary>
      </div>
    </div>
  );
};

const CenterPanel = ({ onDone }: { onDone: () => void }) => {
  const [tab, setTab] = useState("Issue");
  return (
    <section className="w-80 shrink-0 flex flex-col border-r border-gray-700 bg-gray-900 p-3 overflow-y-auto">
      <SectionHeader color="bg-green-500">Transactions</SectionHeader>
      <Divider />
      <TabBar tabs={["Issue", "Return", "Renew"]} active={tab} onChange={setTab} />
      {tab === "Issue"  && <IssueTab  onDone={onDone} />}
      {tab === "Return" && <ReturnTab onDone={onDone} />}
      {tab === "Renew"  && <RenewTab  onDone={onDone} />}
    </section>
  );
};

// ─── Right Panel — Active Transactions Grid ─────────────────────────────────────

const RightPanel = ({ txns, loading }: { txns: Transaction[]; loading: boolean }) => (
  <section className="flex-1 flex flex-col bg-gray-900 p-3 overflow-hidden">
    <div className="flex items-center justify-between mb-3">
      <SectionHeader color="bg-yellow-400">Active Transactions</SectionHeader>
      <span className="text-[0.65rem] bg-gray-800 border border-gray-700 text-gray-400
        px-2 py-0.5 rounded-full">{loading ? "…" : `${txns.length} records`}</span>
    </div>
    <Divider />

    <div className="flex-1 overflow-auto rounded-lg border border-gray-700">
      {loading ? (
        <div className="flex items-center justify-center h-full text-gray-500 gap-2 text-xs">
          <RefreshCw size={14} className="animate-spin" /> Loading…
        </div>
      ) : txns.length === 0 ? (
        <div className="flex flex-col items-center justify-center h-full gap-2 text-gray-600">
          <Inbox size={32} className="opacity-30" />
          <p className="text-xs">No active transactions</p>
        </div>
      ) : (
        <table className="w-full text-xs border-collapse">
          <thead className="sticky top-0 bg-gray-800 z-10">
            <tr className="border-b border-gray-700">
              {["DLR / Acc. No.", "Name", "Status", "Cur. Issue (Due)"].map((h) => (
                <th key={h} className="text-left py-2 px-3 text-[0.65rem] font-bold uppercase
                  tracking-wider text-blue-400">{h}</th>
              ))}
            </tr>
          </thead>
          <tbody>
            {txns.map((t, i) => {
              const overdue = t.status !== "Returned" && new Date(t.expected_return_date) < new Date();
              return (
                <tr key={t.id ?? i} className="border-b border-gray-800 hover:bg-gray-800/60 transition-colors">
                  <td className="py-2 px-3 font-mono text-blue-300">{t.accession_no}</td>
                  <td className="py-2 px-3 text-gray-200 max-w-[140px] truncate">{t.user_name || t.user_id}</td>
                  <td className="py-2 px-3">
                    <StatusPill status={overdue ? "Overdue" : t.status} />
                  </td>
                  <td className={`py-2 px-3 tabular-nums ${overdue ? "text-red-400" : "text-gray-400"}`}>
                    {t.expected_return_date}
                  </td>
                </tr>
              );
            })}
          </tbody>
        </table>
      )}
    </div>
  </section>
);

// ─── Bottom Global Search ───────────────────────────────────────────────────────

const BottomSearch = ({ books }: { books: Book[] }) => {
  const [q, setQ] = useState("");
  const [results, setResults] = useState<Book[]>([]);
  const [busy, setBusy] = useState(false);

  const search = async () => {
    if (!q.trim()) { setResults([]); return; }
    setBusy(true);
    try { setResults(await invoke<Book[]>("search_books", { query: q })); }
    finally { setBusy(false); }
  };

  // Debounce
  const handleChange = (v: string) => {
    setQ(v);
    if (!v.trim()) { setResults([]); return; }
    const t = setTimeout(search, 350);
    return () => clearTimeout(t);
  };

  const display = results.length > 0 ? results
    : books.filter((b) => b.title.toLowerCase().includes(q.toLowerCase()));

  return (
    <div className="relative shrink-0 border-t border-gray-700 bg-gray-900/95 backdrop-blur-sm px-4 py-2">
      <div className="flex items-center gap-3">
        <Search size={14} className="text-gray-500 shrink-0" />
        <input type="text" value={q} onChange={(e) => handleChange(e.target.value)}
          placeholder="Global Search — title, author, accession no., member ID…"
          className="flex-1 bg-transparent border-none text-sm text-gray-200
            placeholder-gray-600 outline-none focus:ring-0"
        />
        {busy && <RefreshCw size={12} className="text-gray-500 animate-spin shrink-0" />}
        {q && <button onClick={() => { setQ(""); setResults([]); }}
          className="text-gray-500 hover:text-white transition-colors shrink-0"><X size={13} /></button>}
      </div>

      {q && display.length > 0 && (
        <div className="absolute bottom-full left-4 right-4 mb-1 max-h-52 overflow-auto
          bg-gray-800 border border-gray-700 rounded-lg shadow-2xl z-50">
          {display.slice(0, 20).map((b, i) => (
            <div key={i} onClick={() => setQ(b.title)}
              className="flex items-center justify-between px-4 py-2
                hover:bg-gray-700 cursor-pointer transition-colors border-b border-gray-700/60 last:border-0">
              <div className="flex items-center gap-2">
                <BookOpen size={12} className="text-blue-400 shrink-0" />
                <div>
                  <p className="text-xs text-white font-medium">{b.title}</p>
                  <p className="text-[0.65rem] text-gray-400">{b.author} · {b.accession_no}</p>
                </div>
              </div>
              <StatusPill status={b.status} />
            </div>
          ))}
        </div>
      )}
    </div>
  );
};

// ─── Main Dashboard Export ──────────────────────────────────────────────────────

interface DashboardProps {
  transactions: Transaction[];
  txnLoading: boolean;
  onRefresh: () => void;
}

export default function Dashboard({ transactions, txnLoading, onRefresh }: DashboardProps) {
  const [searchResults, setSearchResults] = useState<Book[]>([]);

  return (
    <div className="flex flex-col flex-1 overflow-hidden">
      <main className="flex flex-1 overflow-hidden">
        <LeftPanel onResults={setSearchResults} />
        <CenterPanel onDone={onRefresh} />
        <RightPanel txns={transactions} loading={txnLoading} />
      </main>
      <BottomSearch books={searchResults} />
    </div>
  );
}
