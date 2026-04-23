import { useState, useEffect, useCallback, useRef } from "react";
import { invoke } from "./lib/tauri";
import {
  BookMarked, Users, BookOpen, FileText, Settings, Wifi,
  GraduationCap, ChevronDown, Clock,
} from "lucide-react";
import Dashboard from "./components/Dashboard";
import MembershipModule from "./components/MembershipModule";
import BookModule from "./components/BookModule";
import FacultyModule from "./components/FacultyModule";
import { ReportsModule, SettingsModule } from "./components/SystemModule";

// ─── Types ─────────────────────────────────────────────────────────────────────

interface Transaction {
  id: number | null; accession_no: string; user_id: string; user_name: string;
  issue_date: string; expected_return_date: string;
  actual_return_date: string | null; status: string;
}

type ActiveModule = "Dashboard" | "Membership" | "Books" | "Reports" | "Faculty" | "Settings_Activation" | "Settings_Branch";

// ─── Clock ─────────────────────────────────────────────────────────────────────

const LiveClock = () => {
  const [t, setT] = useState(new Date());
  useEffect(() => { const id = setInterval(() => setT(new Date()), 1000); return () => clearInterval(id); }, []);
  return (
    <span className="tabular-nums">
      {t.toLocaleDateString("en-IN", { day: "2-digit", month: "short", year: "numeric" })}
      {" · "}
      {t.toLocaleTimeString("en-IN", { hour: "2-digit", minute: "2-digit", second: "2-digit" })}
    </span>
  );
};

// ─── Navigation Bar ─────────────────────────────────────────────────────────────
// NOTE: No "Admin" option is included per specification.

interface NavMenu {
  label: string;
  icon: React.ReactNode;
  module?: ActiveModule;
  items?: { label: string; action?: () => void }[];
}

const NavBar = ({
  active,
  onNavigate,
}: {
  active: ActiveModule;
  onNavigate: (m: ActiveModule) => void;
}) => {
  const [open, setOpen] = useState<string | null>(null);
  const navRef = useRef<HTMLElement>(null);

  useEffect(() => {
    const h = (e: MouseEvent) => {
      if (navRef.current && !navRef.current.contains(e.target as Node)) {
        setOpen(null);
      }
    };
    document.addEventListener("mousedown", h);
    return () => document.removeEventListener("mousedown", h);
  }, []);

  const menus: NavMenu[] = [
    {
      label: "Membership", icon: <Users size={13} />, module: "Membership",
      items: [
        { label: "Add Member",          action: () => onNavigate("Membership") },
        { label: "Edit Member",         action: () => onNavigate("Membership") },
        { label: "Member Details",      action: () => onNavigate("Membership") },
        { label: "Export Member Data",  action: () => onNavigate("Membership") },
      ],
    },
    {
      label: "Book Details", icon: <BookOpen size={13} />, module: "Books",
      items: [
        { label: "Add Books",           action: () => onNavigate("Books") },
        { label: "Edit Book",           action: () => onNavigate("Books") },
        { label: "Book Detail",         action: () => onNavigate("Books") },
        { label: "Stock Verification",  action: () => onNavigate("Books") },
        { label: "Import Book Data",    action: () => onNavigate("Books") },
      ],
    },
    {
      label: "Reports", icon: <FileText size={13} />, module: "Reports",
      items: [
        { label: "Generate Report",     action: () => onNavigate("Reports") },
        { label: "Overdue Report",      action: () => onNavigate("Reports") },
        { label: "Issue History",       action: () => onNavigate("Reports") },
      ],
    },
    {
      label: "Faculty Module", icon: <GraduationCap size={13} />, module: "Faculty",
      items: [
        { label: "Add Faculty",         action: () => onNavigate("Faculty") },
        { label: "Edit Faculty",        action: () => onNavigate("Faculty") },
        { label: "Faculty Details",     action: () => onNavigate("Faculty") },
      ],
    },
    {
      label: "Settings", icon: <Settings size={13} />,
      items: [
        { label: "Activation Settings", action: () => onNavigate("Settings_Activation") },
        { label: "Edit Branch",         action: () => onNavigate("Settings_Branch") },
        { label: "Database Info" },
      ],
    },
    {
      label: "Connection", icon: <Wifi size={13} />,
      items: [
        { label: "Database: libsoft.db (local)" },
        { label: "Status: ✔ Connected" },
      ],
    },
  ];

  return (
    <nav ref={navRef} className="flex items-center gap-0.5 px-4 h-10 bg-gray-900 border-b border-gray-700
                    select-none shrink-0 z-50">
      {/* Logo — click to go to dashboard */}
      <button
        onClick={() => onNavigate("Dashboard")}
        className="flex items-center gap-2 mr-4 hover:opacity-80 transition-opacity"
      >
        <BookMarked size={16} className="text-blue-400" />
        <span className="text-blue-400 font-bold text-sm tracking-wide">
          Lib<span className="text-white">Soft</span>
        </span>
      </button>
      <div className="w-px h-5 bg-gray-700 mr-2" />

      {menus.map((menu) => (
        <div key={menu.label} className="relative">
          <button
            id={`nav-${menu.label.replace(/\s+/g, "-").toLowerCase()}`}
            onClick={() => setOpen((p) => (p === menu.label ? null : menu.label))}
            className={`flex items-center gap-1.5 px-3 py-2 text-xs font-medium rounded
              transition-colors duration-150
              ${active === menu.module || (menu.label === "Settings" && active.startsWith("Settings"))
                ? "bg-blue-700/40 text-blue-300"
                : open === menu.label
                ? "bg-gray-800 text-blue-300"
                : "text-gray-300 hover:text-white hover:bg-gray-800"}`}
          >
            {menu.icon}
            {menu.label}
            {menu.items && (
              <ChevronDown size={11}
                className={`transition-transform duration-150 ${open === menu.label ? "rotate-180" : ""}`} />
            )}
          </button>

          {open === menu.label && menu.items && (
            <div className="absolute top-full left-0 mt-1 w-52 bg-gray-800 border border-gray-700
                         rounded-lg shadow-2xl z-50 py-1 overflow-hidden">
              {menu.items.map((item) => (
                <button key={item.label}
                  onClick={() => { item.action?.(); setOpen(null); }}
                  className="block w-full text-left px-4 py-2 text-xs text-gray-300
                             hover:bg-gray-700 hover:text-white transition-colors">
                  {item.label}
                </button>
              ))}
            </div>
          )}
        </div>
      ))}

      <div className="ml-auto flex items-center gap-1.5 text-[0.65rem] text-gray-500">
        <Clock size={11} />
        <LiveClock />
      </div>
    </nav>
  );
};

// ─── Root App ───────────────────────────────────────────────────────────────────

export default function App() {
  const [module, setModule] = useState<ActiveModule>("Dashboard");
  const [transactions, setTransactions] = useState<Transaction[]>([]);
  const [txnLoading, setTxnLoading] = useState(true);

  const loadTransactions = useCallback(async () => {
    setTxnLoading(true);
    try { setTransactions(await invoke<Transaction[]>("get_active_transactions")); }
    catch (e) { console.error(e); }
    finally { setTxnLoading(false); }
  }, []);

  useEffect(() => { loadTransactions(); }, [loadTransactions]);

  return (
    <div className="flex flex-col h-screen bg-gray-950 overflow-hidden font-mono">
      <NavBar active={module} onNavigate={setModule} />

      {module === "Dashboard"           && <Dashboard transactions={transactions} txnLoading={txnLoading} onRefresh={loadTransactions} />}
      {module === "Membership"          && <MembershipModule />}
      {module === "Books"               && <BookModule />}
      {module === "Faculty"             && <FacultyModule />}
      {module === "Reports"             && <ReportsModule />}
      {module === "Settings_Activation" && <SettingsModule view="activation" />}
      {module === "Settings_Branch"     && <SettingsModule view="branch" />}
    </div>
  );
}
