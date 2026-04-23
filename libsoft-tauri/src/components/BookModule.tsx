import { useState } from "react";
import { invoke } from "../lib/tauri";
import {
  BookOpen, PlusCircle, Search, Edit2, ListChecks,
  FileUp, CheckCircle, AlertCircle, Save,
} from "lucide-react";
import {
  Field, Input, Select, BtnPrimary, BtnGhost, BtnSuccess,
  Card, Divider,
} from "./ui";

// ─── Types ─────────────────────────────────────────────────────────────────────

interface Book {
  accession_no: string; call_no: string; title: string; author: string;
  branch: string; publisher: string; price: number; bill_no: string; status: string;
}

const emptyBook = (): Book => ({
  accession_no: "", call_no: "", title: "", author: "",
  branch: "Computer Science", publisher: "", price: 0, bill_no: "", status: "Available",
});

const Feedback = ({ msg }: { msg: { t: "ok" | "err"; v: string } }) => (
  <div className={`flex items-center gap-1.5 text-xs px-3 py-2 rounded mb-3
    ${msg.t === "ok" ? "bg-green-900/50 text-green-300" : "bg-red-900/50 text-red-300"}`}>
    {msg.t === "ok" ? <CheckCircle size={12} /> : <AlertCircle size={12} />} {msg.v}
  </div>
);

// ─── Add Books (Bulk) ──────────────────────────────────────────────────────────

const AddBooksBulk = () => {
  const [rows, setRows] = useState<Book[]>([emptyBook()]);
  const [msg, setMsg] = useState<{ t: "ok" | "err"; v: string } | null>(null);

  const addRow = () => setRows([...rows, emptyBook()]);

  const update = (idx: number, key: keyof Book, val: string | number) => {
    const newRows = [...rows];
    newRows[idx] = { ...newRows[idx], [key]: val };
    setRows(newRows);
  };

  const save = async () => {
    try {
      for (const b of rows) {
        if (b.accession_no && b.title) await invoke("add_book", { book: b });
      }
      setMsg({ t: "ok", v: "Books saved successfully." });
      setRows([emptyBook()]);
    } catch (e) { setMsg({ t: "err", v: String(e) }); }
  };

  return (
    <div className="flex flex-col h-full gap-3">
      {msg && <Feedback msg={msg} />}
      <div className="flex-1 overflow-auto rounded-lg border border-gray-700 bg-gray-900">
        <table className="w-full text-xs border-collapse">
          <thead className="sticky top-0 bg-gray-800 z-10">
            <tr className="border-b border-gray-700">
              {["Acc. No.", "Call No.", "Book Name", "Author", "Branch", "Publisher", "Price (Rs.)", "Bill No."].map((h) => (
                <th key={h} className="text-left py-2 px-2 text-[0.65rem] font-bold uppercase text-blue-400">{h}</th>
              ))}
            </tr>
          </thead>
          <tbody>
            {rows.map((r, i) => (
              <tr key={i} className="border-b border-gray-800">
                <td className="p-1"><Input className="px-1.5 py-1 text-[0.65rem]" value={r.accession_no} onChange={e => update(i, "accession_no", e.target.value)} /></td>
                <td className="p-1"><Input className="px-1.5 py-1 text-[0.65rem]" value={r.call_no} onChange={e => update(i, "call_no", e.target.value)} /></td>
                <td className="p-1"><Input className="px-1.5 py-1 text-[0.65rem]" value={r.title} onChange={e => update(i, "title", e.target.value)} /></td>
                <td className="p-1"><Input className="px-1.5 py-1 text-[0.65rem]" value={r.author} onChange={e => update(i, "author", e.target.value)} /></td>
                <td className="p-1"><Input className="px-1.5 py-1 text-[0.65rem]" value={r.branch} onChange={e => update(i, "branch", e.target.value)} /></td>
                <td className="p-1"><Input className="px-1.5 py-1 text-[0.65rem]" value={r.publisher} onChange={e => update(i, "publisher", e.target.value)} /></td>
                <td className="p-1"><Input className="px-1.5 py-1 text-[0.65rem]" type="number" value={r.price} onChange={e => update(i, "price", parseFloat(e.target.value) || 0)} /></td>
                <td className="p-1"><Input className="px-1.5 py-1 text-[0.65rem]" value={r.bill_no} onChange={e => update(i, "bill_no", e.target.value)} /></td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
      <div className="flex justify-between mt-2 shrink-0">
        <BtnGhost onClick={addRow}><PlusCircle size={12} /> Add Row</BtnGhost>
        <div className="flex gap-2">
          <BtnGhost>Validate Data</BtnGhost>
          <BtnSuccess onClick={save}><Save size={12} /> Save Books</BtnSuccess>
        </div>
      </div>
    </div>
  );
};

// ─── Edit Book ─────────────────────────────────────────────────────────────────

const EditBook = () => {
  const [acc, setAcc] = useState("");
  const [form, setForm] = useState<Book | null>(null);
  const [msg, setMsg] = useState<{ t: "ok" | "err"; v: string } | null>(null);

  const search = async () => {
    try {
      const res: Book[] = await invoke("search_books", { query: acc });
      const b = res.find(x => x.accession_no === acc);
      if (!b) { setMsg({ t: "err", v: "Book not found." }); setForm(null); return; }
      setForm(b); setMsg(null);
    } catch (e) { setMsg({ t: "err", v: String(e) }); }
  };

  const save = async () => {
    if (!form) return;
    try {
      await invoke("add_book", { book: form });
      setMsg({ t: "ok", v: "Book updated." });
    } catch (e) { setMsg({ t: "err", v: String(e) }); }
  };

  return (
    <div className="flex flex-col gap-4 max-w-2xl mx-auto">
      <Card className="p-3 flex gap-2">
        <Input placeholder="Enter Accession Number" value={acc} onChange={e => setAcc(e.target.value)}
          onKeyDown={e => e.key === 'Enter' && search()} />
        <BtnPrimary onClick={search}><Search size={12} /> Search</BtnPrimary>
      </Card>

      {msg && <Feedback msg={msg} />}

      {form && (
        <Card className="p-4 flex flex-col gap-4">
          <div className="grid grid-cols-2 gap-4">
            <Field label="Accession Number"><Input value={form.accession_no} disabled /></Field>
            <Field label="Book Name"><Input value={form.title} onChange={e => setForm({ ...form, title: e.target.value })} /></Field>
            <Field label="Branch"><Input value={form.branch} onChange={e => setForm({ ...form, branch: e.target.value })} /></Field>
            <Field label="Price & Bill">
              <div className="flex gap-2">
                <Input type="number" placeholder="Price" className="w-24" value={form.price} onChange={e => setForm({ ...form, price: parseFloat(e.target.value) || 0 })} />
                <Input placeholder="Bill No." value={form.bill_no} onChange={e => setForm({ ...form, bill_no: e.target.value })} />
              </div>
            </Field>
            <Field label="Call Number"><Input value={form.call_no} onChange={e => setForm({ ...form, call_no: e.target.value })} /></Field>
            <Field label="Author Name"><Input value={form.author} onChange={e => setForm({ ...form, author: e.target.value })} /></Field>
            <Field label="Publisher"><Input value={form.publisher} onChange={e => setForm({ ...form, publisher: e.target.value })} /></Field>
            <Field label="Issue Status">
              <Select value={form.status} onChange={e => setForm({ ...form, status: e.target.value })}>
                <option>Available</option>
                <option>Issued</option>
                <option>Lost</option>
              </Select>
            </Field>
          </div>
          <div className="flex justify-end mt-2">
            <BtnSuccess onClick={save}><Save size={12} /> Save Changes</BtnSuccess>
          </div>
        </Card>
      )}
    </div>
  );
};

// ─── Stock Verification ────────────────────────────────────────────────────────

const StockVerification = () => {
  return (
    <div className="flex flex-col h-full gap-4">
      <div className="flex gap-4 shrink-0">
        <Card className="flex-1 p-4 flex flex-col items-center justify-center border-blue-500/30 bg-blue-900/10">
          <span className="text-2xl font-bold text-blue-400">1,245</span>
          <span className="text-xs text-gray-400 uppercase">Total Books</span>
        </Card>
        <Card className="flex-1 p-4 flex flex-col items-center justify-center border-green-500/30 bg-green-900/10">
          <span className="text-2xl font-bold text-green-400">890</span>
          <span className="text-xs text-gray-400 uppercase">Verified Books</span>
        </Card>
        <Card className="flex-1 p-4 flex flex-col items-center justify-center border-red-500/30 bg-red-900/10">
          <span className="text-2xl font-bold text-red-400">355</span>
          <span className="text-xs text-gray-400 uppercase">Unverified Books</span>
        </Card>
      </div>

      <div className="flex gap-4 flex-1 overflow-hidden">
        <Card className="w-48 p-2 flex flex-col gap-1 shrink-0">
          <BtnGhost className="justify-start">All Books List</BtnGhost>
          <BtnGhost className="justify-start text-green-400">Verified Book List</BtnGhost>
          <BtnGhost className="justify-start text-red-400">Unverified Book List</BtnGhost>
        </Card>

        <Card className="flex-1 p-4 flex flex-col gap-4">
          <div className="flex gap-2">
            <Input placeholder="Scan Barcode / Enter Accession No..." className="text-sm py-2" autoFocus />
            <BtnPrimary><Search size={14} /></BtnPrimary>
          </div>
          <Divider />
          <div className="flex-1 flex flex-col items-center justify-center text-gray-500 gap-2">
            <ListChecks size={40} className="opacity-20" />
            <p className="text-xs">Scan a book to verify</p>
          </div>
        </Card>
      </div>
    </div>
  );
};

// ─── Import Book Data ──────────────────────────────────────────────────────────

const ImportBooks = () => {
  return (
    <div className="flex flex-col h-full gap-4">
      <Card className="p-3 flex gap-2 shrink-0">
        <Input placeholder="Select Excel / CSV file..." readOnly />
        <BtnGhost><Search size={12} /> Find</BtnGhost>
      </Card>
      <div className="flex-1 border border-gray-700 bg-gray-900 rounded-lg flex items-center justify-center">
        <p className="text-xs text-gray-500">Data preview will appear here</p>
      </div>
      <div className="flex justify-end gap-2 shrink-0">
        <BtnGhost>Validate Data</BtnGhost>
        <BtnSuccess><FileUp size={12} /> Add to Database</BtnSuccess>
      </div>
    </div>
  );
};

// ─── Book Module Root ──────────────────────────────────────────────────────────

export default function BookModule() {
  const [tab, setTab] = useState("Add Books");
  const TABS = ["Add Books", "Edit Book", "Stock Verification", "Import Book Data"];
  const ICONS = [<PlusCircle size={13} />, <Edit2 size={13} />, <ListChecks size={13} />, <FileUp size={13} />];

  return (
    <div className="flex flex-col flex-1 overflow-hidden bg-gray-950 p-4 gap-4">
      <div className="flex items-center gap-3 shrink-0">
        <BookOpen size={18} className="text-blue-400" />
        <h1 className="text-sm font-bold uppercase tracking-widest text-gray-200">Book Details</h1>
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
      <div className="flex-1 overflow-hidden">
        {tab === "Add Books" && <AddBooksBulk />}
        {tab === "Edit Book" && <EditBook />}
        {tab === "Stock Verification" && <StockVerification />}
        {tab === "Import Book Data" && <ImportBooks />}
      </div>
    </div>
  );
}
