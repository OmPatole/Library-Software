use std::sync::Mutex;
use tauri::State;
use crate::db::{Database, Book, Member, Transaction};

type DbState<'a> = State<'a, Mutex<Database>>;

// ─── Book commands ─────────────────────────────────────────────────────────────

/// Search books by accession no, title, author, or call number.
/// Invoked from React via: invoke('search_books', { query: '...' })
#[tauri::command]
pub fn search_books(db: DbState, query: String) -> Result<Vec<Book>, String> {
    let db = db.lock().map_err(|e| e.to_string())?;
    db.search_books(&query).map_err(|e| e.to_string())
}

/// Add or update a book record.
/// Invoked from React via: invoke('add_book', { book: { ... } })
#[tauri::command]
pub fn add_book(db: DbState, book: Book) -> Result<(), String> {
    let db = db.lock().map_err(|e| e.to_string())?;
    db.add_book(&book).map_err(|e| e.to_string())
}

// ─── Member commands ───────────────────────────────────────────────────────────

/// Fetch all library members.
/// Invoked from React via: invoke('get_members')
#[tauri::command]
pub fn get_members(db: DbState) -> Result<Vec<Member>, String> {
    let db = db.lock().map_err(|e| e.to_string())?;
    db.get_members().map_err(|e| e.to_string())
}

/// Add or update a member record.
/// Invoked from React via: invoke('add_member', { member: { ... } })
#[tauri::command]
pub fn add_member(db: DbState, member: Member) -> Result<(), String> {
    let db = db.lock().map_err(|e| e.to_string())?;
    db.add_member(&member).map_err(|e| e.to_string())
}

// ─── Transaction commands ──────────────────────────────────────────────────────

/// Issue a book to a member/faculty.
/// Invoked from React via: invoke('issue_book', { transaction: { ... } })
#[tauri::command]
pub fn issue_book(db: DbState, transaction: Transaction) -> Result<(), String> {
    let db = db.lock().map_err(|e| e.to_string())?;
    db.issue_book(&transaction).map_err(|e| e.to_string())
}

/// Mark a book as returned.
/// Invoked from React via: invoke('return_book', { accessionNo: '...', returnDate: '...' })
#[tauri::command]
pub fn return_book(db: DbState, accession_no: String, return_date: String) -> Result<(), String> {
    let db = db.lock().map_err(|e| e.to_string())?;
    db.return_book(&accession_no, &return_date).map_err(|e| e.to_string())
}

/// Renew a book (extend its due date).
/// Invoked from React via: invoke('renew_book', { accessionNo: '...', newReturnDate: '...' })
#[tauri::command]
pub fn renew_book(db: DbState, accession_no: String, new_return_date: String) -> Result<(), String> {
    let db = db.lock().map_err(|e| e.to_string())?;
    db.renew_book(&accession_no, &new_return_date).map_err(|e| e.to_string())
}

/// Fetch all currently active (Issued / Renewed) transactions.
/// Invoked from React via: invoke('get_active_transactions')
#[tauri::command]
pub fn get_active_transactions(db: DbState) -> Result<Vec<Transaction>, String> {
    let db = db.lock().map_err(|e| e.to_string())?;
    db.get_active_transactions().map_err(|e| e.to_string())
}
